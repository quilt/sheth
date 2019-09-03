use arrayref::array_ref;
use bigint::U512;
use sheth::account::Account;
use sheth::hash::{hash, zh};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(arr: &[u8; 32]) -> Self {
        H256(arr.clone())
    }

    pub fn zero() -> Self {
        H256([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Proof {
    indexes: Vec<U512>,
    values: Vec<H256>,
}

pub fn generate(accounts: Vec<(U512, Account)>, height: usize) -> Proof {
    let mut map: BTreeMap<U512, H256> = BTreeMap::new();

    for (address, account) in accounts.iter() {
        // Calulate the root index of the account (e.g. `first_leaf` + address)
        let index = (U512::one() << height) + *address;

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

    indexes.sort_by(|a, b| (&b.0).cmp(&a.0));

    let mut values = Vec::<H256>::new();

    for i in indexes.iter() {
        values.push(map.get(&i).unwrap().clone());
    }

    Proof { indexes, values }
}

#[cfg(test)]
mod test {
    use super::*;

    fn zh(depth: usize) -> H256 {
        let mut buf = [0u8; 64];
        super::zh(depth, &mut buf);
        H256::new(array_ref![buf, 0, 32])
    }

    #[test]
    fn test_1_bit_tree() {
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

        let values = vec![
            H256::new(array_ref![buf, 32, 32]),
            H256::new(array_ref![buf, 0, 32]),
            H256::new(&[0u8; 32]),
            H256::new(array_ref![buf, 96, 32]),
            H256::new(array_ref![buf, 64, 32]),
            zh(0),
        ];

        assert_eq!(
            generate(vec![(0.into(), account.clone())], 1),
            Proof {
                indexes: vec![
                    17.into(),
                    16.into(),
                    11.into(),
                    10.into(),
                    9.into(),
                    3.into()
                ],
                values: values.clone()
            }
        );

        assert_eq!(
            generate(vec![(1.into(), account)], 1),
            Proof {
                indexes: vec![
                    25.into(),
                    24.into(),
                    15.into(),
                    14.into(),
                    13.into(),
                    2.into(),
                ],
                values: values.clone()
            }
        );
    }

    #[test]
    fn test_4_bit_tree_single_account() {
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

        let values = vec![
            H256::new(array_ref![buf, 32, 32]),
            H256::new(array_ref![buf, 0, 32]),
            H256::new(&[0u8; 32]),
            H256::new(array_ref![buf, 96, 32]),
            H256::new(array_ref![buf, 64, 32]),
            zh(0),
            zh(1),
            zh(2),
            zh(3),
        ];

        assert_eq!(
            generate(vec![(9.into(), account.clone())], 4),
            Proof {
                indexes: vec![
                    201.into(),
                    200.into(),
                    103.into(),
                    102.into(),
                    101.into(),
                    24.into(),
                    13.into(),
                    7.into(),
                    2.into(),
                ],
                values: values.clone()
            }
        );
    }
}
