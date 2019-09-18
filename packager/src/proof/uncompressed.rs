use crate::accounts::AddressedAccount;
use crate::proof::h256::H256;
use arrayref::array_ref;
use bigint::U512;
use sheth::hash::{hash, zh};
use std::collections::BTreeMap;
use std::ops::Shl;

#[derive(Debug, PartialEq)]
pub struct UncompressedProof {
    pub indexes: Vec<U512>,
    pub values: Vec<H256>,
}

pub fn generate(accounts: Vec<AddressedAccount>, height: usize) -> UncompressedProof {
    let mut map: BTreeMap<U512, H256> = BTreeMap::new();

    for account in accounts.into_iter() {
        let (address, account) = (account.0, account.1);

        // Calulate the root index of the account (e.g. `first_leaf` + address)
        let index = (U512::one() << height) + U512::from(address);

        // Copy the values of the account into a buffer
        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey);
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[96..104].copy_from_slice(&account.value.to_le_bytes());

        // Insert children nodes of the account, where structure looks like:
        map.insert(index << 3, H256::new(array_ref![buf, 0, 32]));
        map.insert((index << 3) + 1.into(), H256::new(array_ref![buf, 32, 32]));
        map.insert((index << 2) + 1.into(), H256::new(array_ref![buf, 64, 32]));
        map.insert((index << 2) + 2.into(), H256::new(array_ref![buf, 96, 32]));
        map.insert((index << 2) + 3.into(), H256::new(&[0u8; 32]));
    }

    let mut index_buffer: Vec<U512> = map.keys().clone().map(|x| x.to_owned()).collect();
    index_buffer.sort_by(|a, b| (&b.0).cmp(&a.0));

    let mut indexes: Vec<U512> = index_buffer.clone();

    let mut position = 0;
    while index_buffer[position] > 1.into() {
        let left = index_buffer[position] & (!U512::zero() - 1.into());
        let right = left + 1.into();
        let parent = left / 2.into();

        if !map.contains_key(&parent) {
            let left: H256 = match map.get(&left) {
                Some(x) => x.clone(),
                None => {
                    let mut buf = [0u8; 64];
                    zh((height + 1) - index_buffer[position].bits(), &mut buf);
                    let buf = H256::new(array_ref![buf, 0, 32]);
                    indexes.push(left);
                    map.insert(left, buf.clone());
                    buf
                }
            };

            let right: H256 = match map.get(&right) {
                Some(x) => x.clone(),
                None => {
                    let mut buf = [0u8; 64];
                    zh((height + 1) - index_buffer[position].bits(), &mut buf);
                    let buf = H256::new(array_ref![buf, 0, 32]);
                    indexes.push(right);
                    map.insert(right, buf.clone());
                    buf
                }
            };

            // Calculate hash
            let mut buf = [0u8; 64];
            buf[0..32].copy_from_slice(left.as_bytes());
            buf[32..64].copy_from_slice(right.as_bytes());
            hash(&mut buf);

            // Insert hash to map
            let h = H256::new(array_ref![buf, 0, 32]);
            map.insert(parent, h);

            // Push parent index to calculate next level
            index_buffer.push(parent);
        }

        position += 1;
    }

    // Sort bit-alphabetically
    // https://github.com/ethereum/eth2.0-specs/issues/1303
    indexes.sort_by(|a, b| {
        // Normalize (e.g. right pad until the the most significant bit in `a` and `b` align)
        let max = std::cmp::max(a.bits(), b.bits());

        let (a, a_shift) = if a.bits() < max {
            let shift = max - a.bits();
            (a.shl(shift), shift)
        } else {
            (*a, 0)
        };

        let (b, b_shift) = if b.bits() < max {
            let shift = max - b.bits();
            (b.shl(shift), shift)
        } else {
            (*b, 0)
        };
        // ---------------------------------

        match a.cmp(&b) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a_shift.cmp(&b_shift),
        }
    });

    let mut values = Vec::<H256>::new();

    for i in indexes.iter() {
        values.push(map.get(&i).unwrap().clone());
    }

    UncompressedProof { indexes, values }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::proof::h256::zh;
    use sheth::account::Account;

    #[test]
    fn one_bit_tree() {
        //                 +---------- 1 ----------+
        //                /                         \
        //           +-- 2 --+  <= account 0 root    3
        //          /         \
        //         4           5
        //       /   \       /   \
        //      8     9     10   11
        //     / \
        //    16 17   ^     ^     ^
        //            |     |     |
        //    ^   ^   nonce value padding
        //    |   |
        //    |   |
        //    |   pk[32..48]
        //    pk[0..32]

        let account = Account {
            pubkey: [1u8; 48],
            nonce: 123,
            value: 42,
        };

        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey);
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[96..104].copy_from_slice(&account.value.to_le_bytes());

        assert_eq!(
            generate(vec![AddressedAccount(0.into(), account.clone())], 1),
            UncompressedProof {
                indexes: vec![
                    16.into(),
                    17.into(),
                    9.into(),
                    10.into(),
                    11.into(),
                    3.into()
                ],
                values: vec![
                    H256::new(array_ref![buf, 0, 32]),
                    H256::new(array_ref![buf, 32, 32]),
                    H256::new(array_ref![buf, 64, 32]),
                    H256::new(array_ref![buf, 96, 32]),
                    H256::new(&[0u8; 32]),
                    zh(0),
                ]
            }
        );

        assert_eq!(
            generate(vec![AddressedAccount(1.into(), account)], 1),
            UncompressedProof {
                indexes: vec![
                    2.into(),
                    24.into(),
                    25.into(),
                    13.into(),
                    14.into(),
                    15.into()
                ],
                values: vec![
                    zh(0),
                    H256::new(array_ref![buf, 0, 32]),
                    H256::new(array_ref![buf, 32, 32]),
                    H256::new(array_ref![buf, 64, 32]),
                    H256::new(array_ref![buf, 96, 32]),
                    H256::new(&[0u8; 32]),
                ]
            }
        );
    }

    #[test]
    fn four_bit_tree_single_account() {
        //
        //                 +-------- 1 --------+
        //                /                     \
        //           +-- 2 --+               +-- 3 --+
        //          /         \             /         \
        //         4           5           6           7
        //       /   \       /   \       /   \       /  \
        //      8     9     10   11     12   13     14   15
        //     / \   / \   / \   / \   / \   / \   / \   / \
        //    16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
        //                              /  \
        //                             /    \
        //                            50     51
        //                           / \     / \
        //                         100 101 102 103
        //                         / \   ^   ^   ^
        //                       200 201 |   |   |
        //                        ^  ^   |   |   padding
        //                        |  |   |   value
        //                        |  |   nonce
        //                        |  pk[32..48]
        //                        pk[0..32]

        let account = Account {
            pubkey: [2u8; 48],
            nonce: 42,
            value: 123,
        };

        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey);
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[96..104].copy_from_slice(&account.value.to_le_bytes());

        assert_eq!(
            generate(vec![AddressedAccount(9.into(), account.clone())], 4),
            UncompressedProof {
                indexes: vec![
                    2.into(),
                    24.into(),
                    200.into(),
                    201.into(),
                    101.into(),
                    102.into(),
                    103.into(),
                    13.into(),
                    7.into(),
                ],
                values: vec![
                    zh(3),
                    zh(0),
                    H256::new(array_ref![buf, 0, 32]),
                    H256::new(array_ref![buf, 32, 32]),
                    H256::new(array_ref![buf, 64, 32]),
                    H256::new(array_ref![buf, 96, 32]),
                    H256::new(&[0u8; 32]),
                    zh(1),
                    zh(2),
                ]
            }
        );
    }
}
