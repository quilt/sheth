use crate::accounts::AddressedAccount;
use crate::proof::h256::H256;
use crate::proof::sort::alpha_sort;
use arrayref::array_ref;
use bigint::U512;
use sheth::hash::{hash, zh};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct UncompressedProof {
    pub indexes: Vec<U512>,
    pub values: Vec<H256>,
}

pub fn generate(accounts: Vec<AddressedAccount>, height: usize) -> UncompressedProof {
    let mut map = init_multiproof(accounts, height);
    let indexes = fill_proof(&mut map, height);
    let indexes = alpha_sort(&indexes);

    UncompressedProof {
        indexes: indexes.clone(),
        values: indexes
            .iter()
            .map(|i| map.get(&i).unwrap().clone())
            .collect(),
    }
}

pub fn init_multiproof(accounts: Vec<AddressedAccount>, height: usize) -> HashMap<U512, H256> {
    let mut map: HashMap<U512, H256> = HashMap::new();

    for account in accounts.into_iter() {
        let (address, account) = (account.0, account.1);

        // Calulate the root index of the account (e.g. `first_leaf` + address)
        let index = (U512::one() << height) + U512::from(address);

        // Copy the values of the account into a buffer
        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey.as_bytes());
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[96..104].copy_from_slice(&account.value.to_le_bytes());

        // Insert children nodes of the account, where structure looks like:
        map.insert(index << 3, H256::new(array_ref![buf, 0, 32]));
        map.insert((index << 3) + 1.into(), H256::new(array_ref![buf, 32, 32]));
        map.insert((index << 2) + 1.into(), H256::new(array_ref![buf, 64, 32]));
        map.insert((index << 2) + 2.into(), H256::new(array_ref![buf, 96, 32]));
        map.insert((index << 2) + 3.into(), H256::new(&[0u8; 32]));
    }

    map
}

fn fill_proof(map: &mut HashMap<U512, H256>, height: usize) -> Vec<U512> {
    let mut indexes: Vec<U512> = map.keys().map(|x| x.to_owned()).collect();
    indexes.sort();
    indexes.reverse();

    let mut proof_indexes: Vec<U512> = indexes.clone();

    let mut position = 0;

    while indexes[position] > U512::from(1) {
        let left = indexes[position] & (!U512::zero() - U512::one());
        let right = left + 1.into();
        let parent = left / 2.into();

        if !map.contains_key(&parent) {
            let left = get_or_generate(
                map,
                &mut proof_indexes,
                height,
                left,
                indexes[position].bits(),
            );

            let right = get_or_generate(
                map,
                &mut proof_indexes,
                height,
                right,
                indexes[position].bits(),
            );

            // Calculate hash
            let mut buf = [0u8; 64];
            buf[0..32].copy_from_slice(left.as_bytes());
            buf[32..64].copy_from_slice(right.as_bytes());
            hash(&mut buf);

            // Insert hash to map
            map.insert(parent, H256::new(array_ref![buf, 0, 32]));

            // Push parent index to calculate next level
            indexes.push(parent);
        }

        position += 1;
    }

    proof_indexes
}

fn get_or_generate(
    map: &mut HashMap<U512, H256>,
    proof_indexes: &mut Vec<U512>,
    height: usize,
    index: U512,
    zero_bits: usize,
) -> H256 {
    match map.get(&index) {
        Some(x) => x.clone(),
        None => {
            let mut buf = [0u8; 64];
            zh(height + 1 - zero_bits, &mut buf);
            let buf = H256::new(array_ref![buf, 0, 32]);
            proof_indexes.push(index);
            map.insert(index, buf.clone());
            buf
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::proof::h256::zh;
    use sheth::account::Account;
    use sheth::bls::PublicKey;

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
            pubkey: PublicKey::one(),
            nonce: 123,
            value: 42,
        };

        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey.as_bytes());
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
            pubkey: PublicKey::one(),
            nonce: 42,
            value: 123,
        };

        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey.as_bytes());
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
