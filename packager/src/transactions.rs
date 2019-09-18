use crate::accounts::AddressedAccount;
use bigint::U256;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sheth::account::Account;
use sheth::transaction::{Transaction, Transfer};
use std::collections::BTreeMap;

pub fn generate(tx_count: usize, mut accounts: Vec<AddressedAccount>) -> Vec<Transaction> {
    let mut rng = StdRng::seed_from_u64(42);

    let mut map: BTreeMap<U256, Account> = BTreeMap::new();
    let mut transactions: Vec<Transaction> = vec![];

    for i in 0..tx_count {
        let to = rng.gen_range(0, accounts.len());
        let from = rng.gen_range(0, accounts.len());

        let tx = Transaction::Transfer(Transfer {
            to: accounts[to].0.into(),
            from: accounts[from].0.into(),
            nonce: accounts[from].1.nonce,
            amount: rng.gen_range(0, accounts[from].1.value),
            signature: [0u8; 96],
        });

        match &tx {
            Transaction::Transfer(t) => {
                accounts[from].1.nonce += 1;
                accounts[from].1.value -= t.amount;
                accounts[to].1.value += t.amount;
            }
            _ => unreachable!(),
        }

        transactions.push(tx);
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
