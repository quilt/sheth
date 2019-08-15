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
        11, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 195, 86,
        169, 110, 177, 121, 15, 58, 133, 98, 199, 13, 111, 144, 174, 84, 132, 236, 25, 85, 251,
        220, 228, 101, 75, 111, 42, 177, 146, 186, 203, 153,
    ];

    let mut mem = InMemoryBackend::new(2);
    println!("{:?}", mem.load(&branch));
    println!("{:?}", mem.db.len());

    mem.inc_nonce(0.into());
    mem.inc_nonce(0.into());
    mem.inc_nonce(1.into()).unwrap();

    let roots = mem.roots().unwrap();
    println!(
        "pre_root: {:?}, post_root: {:?}",
        hex::encode(roots.0),
        hex::encode(roots.1)
    );

    // println!("{:?}", mem.db);

    let accounts: Vec<Account> = vec![
        Account {
            pubkey: [0u8; 48],
            nonce: 0,
            value: 0,
        },
        Account {
            pubkey: [0u8; 48],
            nonce: 0,
            value: 0,
        },
    ];

    // generate_proof(accounts, 2);
}

pub fn generate_proof(accounts: Vec<Account>, height: usize) {
    let mut map: HashMap<U512, Hash256> = HashMap::new();

    for (i, account) in accounts.iter().enumerate() {
        // Calulate the root index of the account (e.g. `first_leaf` + address)
        let index = (U512::one() << height) + i.into();

        // Copy the values of the account into a buffer
        let mut buf = [0u8; 128];
        buf[0..48].copy_from_slice(&account.pubkey);
        buf[64..72].copy_from_slice(&account.nonce.to_le_bytes());
        buf[96..104].copy_from_slice(&account.value.to_le_bytes());

        // Insert children nodes of the account, where structure looks like:
        map.insert(index << 3, *array_ref![buf, 0, 32]);
        map.insert((index << 3) + 1.into(), *array_ref![buf, 32, 32]);
        map.insert((index << 2) + 1.into(), *array_ref![buf, 64, 32]);
        map.insert((index << 2) + 2.into(), *array_ref![buf, 96, 32]);
        map.insert((index << 2) + 3.into(), [0u8; 32]);
    }

    let mut indexes: Vec<U512> = map.keys().clone().map(|x| x.to_owned()).collect();
    indexes.sort_by(|a, b| (&b.0).cmp(&a.0));
    let mut proof_indexes: Vec<U512> = indexes.clone();

    let mut position = 0;
    while indexes[position] > 1.into() {
        let left = indexes[position] & (!U512::zero() - 1.into());
        let right = left + 1.into();
        let parent = left / 2.into();

        if !map.contains_key(&parent) {
            let left = match map.get(&left) {
                Some(x) => *x,
                None => {
                    let mut buf = [0u8; 64];
                    zh((height + 1) - indexes[position].bits(), &mut buf);
                    let buf = *array_ref![buf, 0, 32];
                    proof_indexes.push(left);
                    map.insert(left, buf);
                    buf
                }
            };

            let right = match map.get(&right) {
                Some(x) => *x,
                None => {
                    let mut buf = [0u8; 64];
                    zh((height + 1) - indexes[position].bits(), &mut buf);
                    let buf = *array_ref![buf, 0, 32];
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

            // println!(
            //     "{} -- {} | {} => {}",
            //     parent,
            //     hex::encode(left),
            //     hex::encode(right),
            //     hex::encode(h)
            // );
        }

        position += 1;
    }

    println!("{:?},", &proof_indexes.len().to_le_bytes()[0..4]);
    proof_indexes.sort_by(|a, b| (&b.0).cmp(&a.0));
    for i in proof_indexes {
        let data = map.get(&i).unwrap();
        // println!("{:?},", hex::encode(data));
        let mut buf = [0u8; 64];
        i.to_little_endian(&mut buf);
        println!("{:?},", &buf[0..33]);
        println!("{:?},", data);
    }
}
