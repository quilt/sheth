mod account;
mod error;
mod hash;
mod process;
mod state;
mod transaction;
mod u264;

use account::{Account, Address};
use arrayref::array_ref;
use bigint::{U256, U512};
use hash::{hash, zh};
use state::Hash256;
use state::{Backend, InMemoryBackend};
use std::collections::HashMap;
use u264::U264;

fn main() {
    let branch = vec![
        7, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 245, 165, 253, 66, 209, 106, 32, 48, 39, 152, 239, 110, 211, 9, 151,
        155, 67, 0, 61, 35, 32, 217, 240, 232, 234, 152, 49, 169, 39, 89, 251, 75,
    ];

    let mut mem = InMemoryBackend::new();
    println!("{:?}", mem.load(&branch));
    println!("{:?}", mem.db.len());

    mem.inc_nonce(0.into());

    println!("{:?}", mem.db);

    // let accounts = vec![
    //     Account {
    //         pubkey: [0u8; 48],
    //         nonce: 0,
    //         value: 0,
    //     },
    //     Account {
    //         pubkey: [0u8; 48],
    //         nonce: 0,
    //         value: 0,
    //     },
    // ];

    // generate_proof(accounts, 2);
}

pub fn generate_proof(accounts: Vec<Account>, height: usize) {
    let mut map: HashMap<U512, Hash256> = HashMap::new();

    for (i, account) in accounts.iter().enumerate() {
        // Calulate the root index of the account (e.g. `first_leaf` + address)
        let index = (U512::one() << height) + i.into();

        // Copy the values of the account into a buffer
        let mut buf = [0u8; 96];
        buf[0..48].copy_from_slice(&account.pubkey);
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[72..80].copy_from_slice(&account.value.to_le_bytes());

        // Insert children nodes of the account, where structure looks like:
        //         account_root(1)
        //         /          \
        //    pubkey_root(2)   [nonce, value](3)
        //    /             \
        // pubkey[0..32](4) pubkey[32..48](5)
        map.insert(index << 2, *array_ref![buf, 0, 32]);
        map.insert((index << 2) + 1.into(), *array_ref![buf, 32, 32]);
        map.insert((index << 1) + 1.into(), *array_ref![buf, 64, 32]);
    }

    let mut indexes: Vec<U512> = map.keys().clone().map(|x| x.to_owned()).collect();
    indexes.sort_by(|a, b| (&b.0).cmp(&a.0));
    let mut proof_indexes: Vec<U512> = indexes.clone();

    let mut position = 0;
    while indexes[position] > 1.into() {
        let left = indexes[position] & (!U512::zero() - 1.into());
        let right = left + 1.into();
        let parent = left / 2.into();

        // println!("left: {:?}, right: {:?}, parent: {:?}", left, right, parent);

        if !map.contains_key(&parent) {
            // Calculate zero hash for this height
            let mut buf = [0u8; 64];

            // The zero hash depth is the log2(index), but to avoid any actual log functions we're
            // just using the `bits()` method. There is an edge case where there is an overflowing
            // subtraction for the sibling of the `[nonce, value]` leaf of an intialized account.
            // It must be `[0u8; 32]` *and* two levels above where the accounts' root hashes are
            // should resolve to `[0u8; 32]` when an account is unintialized. The only time there
            // is an overflowing sub is when the sibling node from above is being calculated, so
            // just return 0 in that instance.
            let depth = {
                let (ret, flag) = (height + 1).overflowing_sub(indexes[position].bits());

                if flag {
                    0
                } else {
                    ret
                }
            };

            zh(depth, &mut buf);

            let buf = *array_ref![buf, 0, 32];
            let left = match map.get(&left) {
                Some(x) => *x,
                None => {
                    proof_indexes.push(left);
                    map.insert(left, buf);
                    buf
                }
            };

            let right = match map.get(&right) {
                Some(x) => *x,
                None => {
                    proof_indexes.push(right);
                    map.insert(right, buf);
                    buf
                }
            };

            // Calculate hash
            let mut buf = [0u8; 64];
            buf[0..32].copy_from_slice(&left);
            buf[32..64].copy_from_slice(&right);
            hash(&mut buf);

            // Insert hash to map
            let h = array_ref![buf, 0, 32];
            map.insert(parent, *h);

            // Push parent index to calculate next level
            indexes.push(parent);
        }

        position += 1;
    }

    println!("root: {:?}", hex::encode(map.get(&1.into()).unwrap()));
    println!("{:?},", &proof_indexes.len().to_le_bytes()[0..4]);
    proof_indexes.sort_by(|a, b| (&b.0).cmp(&a.0));
    let mut buf = [0u8; 64];
    for i in proof_indexes {
        let data = map.get(&i).unwrap();
        // println!("{:?},", hex::encode(data));
        let mut buf = [0u8; 64];
        i.to_little_endian(&mut buf);
        println!("{:?},", &buf[0..33]);
        println!("{:?},", data);
    }
}
