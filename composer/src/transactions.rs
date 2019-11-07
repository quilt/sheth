use crate::accounts::AddressedAccount;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sheth::state::TokenColor;
use sheth::transaction::{Transaction, Transfer};

/// Generate `n` number of transactions between `accounts`.
pub fn generate(n: usize, mut accounts: Vec<AddressedAccount>) -> Vec<Transaction> {
    let mut rng = StdRng::seed_from_u64(42);

    let mut transactions: Vec<Transaction> = vec![];

    for _ in 0..n {
        let to = rng.gen_range(0, accounts.len());
        let from = rng.gen_range(0, accounts.len());

        let (color, amount) = match rng.gen_range(0, 2) {
            0 => (TokenColor::Red, accounts[from].1.red_value),
            1 => (TokenColor::Green, accounts[from].1.green_value),
            2 => (TokenColor::Blue, accounts[from].1.blue_value),
            _ => panic!("Unknown token color"),
        };

        let tx = Transaction::Transfer(Transfer {
            to: accounts[to].0.into(),
            from: accounts[from].0.into(),
            nonce: accounts[from].1.nonce,
            amount: rng.gen_range(0, amount),
            color,
            signature: [0u8; 96],
        });

        match &tx {
            Transaction::Transfer(t) => {
                accounts[from].1.nonce += 1;

                match t.color {
                    TokenColor::Red => {
                        accounts[from].1.red_value -= t.amount;
                        accounts[to].1.red_value += t.amount;
                    }
                    TokenColor::Green => {
                        accounts[from].1.green_value -= t.amount;
                        accounts[to].1.green_value += t.amount;
                    }
                    TokenColor::Blue => {
                        accounts[from].1.blue_value -= t.amount;
                        accounts[to].1.blue_value += t.amount;
                    }
                }
            }
            _ => unreachable!(),
        }

        transactions.push(tx);
    }

    transactions
}

/// Convert an array of `Transaction` to an unaligned array of `u8`.
pub fn serialize(transactions: &[Transaction]) -> Vec<u8> {
    let mut bytes = transactions.len().to_le_bytes()[0..4].to_vec();

    for tx in transactions {
        match tx {
            Transaction::Transfer(tx) => {
                bytes.extend_from_slice(&<[u8; 32]>::from(tx.to));
                bytes.extend_from_slice(&<[u8; 32]>::from(tx.from));
                bytes.extend_from_slice(&tx.nonce.to_le_bytes());
                bytes.extend_from_slice(&tx.amount.to_le_bytes());
                bytes.extend_from_slice(&(tx.color as u8).to_le_bytes());
                bytes.extend_from_slice(&tx.signature);
            }
            _ => unimplemented!(),
        }
    }

    bytes
}
