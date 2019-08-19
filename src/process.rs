use crate::error::Error;
use crate::state::{Backend, InMemoryBackend};
use crate::transaction::{Transaction, Transfer};
use alloc::borrow::ToOwned;

pub fn process_transactions<T: Backend>(
    db: &mut T,
    transactions: &[Transaction],
) -> Result<(), Error> {
    for tx in transactions {
        if let Err(_) = tx.verify(db) {
            continue;
        }

        db.inc_nonce(tx.from())?;

        match tx {
            Transaction::Transfer(t) => transfer(db, t)?,
            Transaction::Deposit(d) => unimplemented!(),
            Transaction::Withdrawal(w) => unimplemented!(),
        }
    }

    Ok(())
}

fn transfer<T: Backend>(db: &mut T, tx: &Transfer) -> Result<(), Error> {
    // db.sub_value(tx.from, tx.amount)?;
    // db.add_value(tx.to, tx.amount)?;

    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::account::{Account, Address};
    use crate::hash::{hash, zh};
    use crate::state::Hash256;
    use crate::transaction::{Transaction, Transfer};
    use crate::u264::U264;
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use arrayref::array_ref;
    use bigint::{U256, U512};

    fn serialize_transactions(transactions: &[Transaction]) -> Vec<u8> {
        let mut bytes = transactions.len().to_le_bytes()[0..4].to_vec();

        for tx in transactions {
            match tx {
                Transaction::Transfer(tx) => {
                    bytes.extend_from_slice(&<[u8; 32]>::from(tx.to));
                    bytes.extend_from_slice(&<[u8; 32]>::from(tx.from));
                    bytes.extend_from_slice(&tx.nonce.to_le_bytes());
                    bytes.extend_from_slice(&tx.amount.to_le_bytes());
                    bytes.extend_from_slice(&tx.signature);
                }
                _ => unimplemented!(),
            }
        }
        bytes
    }

    fn generate_proof(accounts: &[Account], height: usize) -> Vec<u8> {
        let mut map: BTreeMap<U512, Hash256> = BTreeMap::new();

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
                let left: [u8; 32] = match map.get(&left) {
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

                let right: [u8; 32] = match map.get(&right) {
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
            }

            position += 1;
        }

        let mut proof = proof_indexes.len().to_le_bytes()[0..4].to_vec();

        proof_indexes.sort_by(|a, b| (&b.0).cmp(&a.0));
        for i in proof_indexes.iter() {
            let mut buf = [0u8; 64];
            i.to_little_endian(&mut buf);
            proof.extend_from_slice(&buf[0..33]);
        }
        for i in proof_indexes.iter() {
            proof.extend_from_slice(&map.get(&i).unwrap()[0..32]);
        }

        proof
    }

    #[test]
    fn two_accounts() {
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

        let transactions = vec![
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 0,
                amount: 0,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 1,
                amount: 0,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(0).into(),
                from: U256::from(1).into(),
                nonce: 0,
                amount: 0,
                signature: [0; 96],
            }),
        ];

        let mut blob = serialize_transactions(&transactions);
        blob.extend(generate_proof(&accounts, 2));
        println!("input blob: {}", hex::encode(blob));

        let mut mem = InMemoryBackend::new(2);

        assert_eq!(mem.load(&generate_proof(&accounts, 2)), Ok(()));
        assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

        let roots = mem.roots().unwrap();

        assert_eq!(
            "7b7a40ea65e15b5d6722d88b7d5ebc8198747bafe7980e6c1aeef8ffdf0d41cd",
            hex::encode(roots.0)
        );

        assert_eq!(
            "0f4872fd12bf989de4278622a31cf9e076886484287f6e44ec45879672b4f27c",
            hex::encode(roots.1)
        );
    }
}
