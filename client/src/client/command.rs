use super::error::Error;
use bigint::U256;
use composer::accounts::AddressedAccount;
use composer::transactions::serialize;
use imp::Imp;
use sheth::process::process_transactions;
use sheth::state::State;
use sheth::transaction::{Transaction, Transfer};
use sheth::u264::U264;
use std::collections::HashMap;

/// A enum that describes the possible commands a user might send to the client and their required
/// arguments.
#[derive(Debug, PartialEq)]
pub enum Command {
    Balance(BalanceCmd),
    Transfer(TransferCmd),
    Accounts(AccountsCmd),
    Exit,
}

/// The balance command will return the balance of a specified address.
#[derive(Debug, PartialEq)]
pub struct BalanceCmd {
    pub(crate) address: U256,
}

/// The transfer command will transfer some amount from one specified account to another.
#[derive(Debug, PartialEq)]
pub struct TransferCmd {
    pub(crate) from: U256,
    pub(crate) to: U256,
    pub(crate) amount: u64,
}

/// The accounts command will list the accounts managed by the client.
#[derive(Debug, PartialEq)]
pub struct AccountsCmd();

impl BalanceCmd {
    pub fn execute(&self, db: &Imp<U264>) -> Result<(), Error> {
        let value = db
            .value(self.address.into())
            .map_err(|_| Error::AddressUnknown("".to_string()))?;

        println!("Balance is: {}", value);

        Ok(())
    }
}

impl TransferCmd {
    pub fn execute(&self, db: &mut Imp<U264>) -> Result<(), Error> {
        let nonce = db
            .nonce(self.from.into())
            .map_err(|_| Error::AddressUnknown("".to_string()))?;

        let tx = Transaction::Transfer(Transfer {
            to: self.to.into(),
            from: self.from.into(),
            nonce,
            amount: self.amount,
            signature: [0u8; 96],
        });

        let mut body = serialize(&vec![tx.clone()]);
        body.extend(imp_to_bytes(db));
        let mut request: HashMap<String, String> = HashMap::new();
        request.insert("block_body".to_string(), hex::encode(body));

        reqwest::Client::new()
            .post("http://127.0.0.1:5052/shard/0/block_body")
            .json(&request)
            .send()
            .map_err(|_| Error::TransactionFailed("connection error".to_string()))?;

        process_transactions(db, &vec![tx])
            .map_err(|_| Error::TransactionFailed("local error".to_string()))?;

        Ok(())
    }
}

impl AccountsCmd {
    pub fn execute(&self, accounts: &Vec<AddressedAccount>) -> Result<(), Error> {
        for account in accounts {
            let mut buf = [0u8; 32];
            account.0.to_big_endian(&mut buf);
            println!("0x{}", hex::encode(buf));
        }

        Ok(())
    }
}

fn imp_to_bytes(proof: &Imp<U264>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];
    ret.extend(
        (((proof.offsets.len() + 8) / 8) as u64)
            .to_le_bytes()
            .to_vec(),
    );
    ret.extend(proof.offsets);
    ret.extend(&*proof.db);
    ret
}
