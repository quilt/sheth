use bigint::U256;
use sheth::account::Account;
use sheth::transaction::{Transaction, Transfer};
use std::collections::BTreeMap;

pub fn generate(tx_count: usize, account_count: usize) -> Vec<Transaction> {
    let mut map: BTreeMap<U256, Account> = BTreeMap::new();
    let mut transactions: Vec<Transaction> = vec![];

    for i in 0..tx_count {
        let mut sender = map
            .get(&((i % account_count).into()))
            .unwrap_or(&Account {
                pubkey: [0u8; 48],
                nonce: 0,
                value: 0,
            })
            .clone();

        transactions.push(Transaction::Transfer(Transfer {
            to: (i % (account_count - 1)).into(),
            from: (i % account_count).into(),
            nonce: sender.nonce,
            amount: sender.value,
            signature: [0u8; 96],
        }));

        sender.nonce += 1;
        map.insert((1 % account_count).into(), sender);
    }

    transactions
}

pub fn serialize(transactions: &[Transaction]) -> Vec<u8> {
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
