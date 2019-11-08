use super::error::Error;
use bigint::U256;
use composer::accounts::AddressedAccount;
use composer::transactions::serialize;
use imp::Imp;
use sheth::process::process_transactions;
use sheth::state::{State, TokenColor};
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
            .value(TokenColor::Red, self.address.into())
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
            color: TokenColor::Red,
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

#[cfg(test)]
mod test {
    use super::*;

    use composer::blob;

    const ADDRESS: [u8; 32] = [
        185, 79, 94, 160, 186, 57, 73, 76, 232, 57, 97, 63, 255, 186, 116, 39, 149, 121, 38, 138,
        116, 39, 149, 121, 38, 131, 66, 69, 103, 137, 101, 35,
    ];

    macro_rules! create_db {
        ($blob_name: ident, $db_name: ident, $accounts_expr: expr, $tree_height: expr) => {
            #[allow(unused_mut)]
            let mut $blob_name = blob::generate($accounts_expr, 0, $tree_height);
            #[allow(unused_mut)]
            let mut $db_name = Imp::<U264>::new(&mut $blob_name.proof, $tree_height);
        };
    }

    fn create_test_transfer(accounts: &Vec<AddressedAccount>) -> TransferCmd {
        TransferCmd {
            from: accounts[0].0,
            to: accounts[1].0,
            amount: 45,
        }
    }

    #[test]
    fn balance_known_address_ok() {
        create_db!(blob, db, 1, 256);
        assert_eq!(
            Ok(()),
            BalanceCmd {
                address: blob.accounts[0].0
            }
            .execute(&db)
        );
    }

    #[test]
    #[ignore] // TODO: error handling in Imp(State)::value() and re-enable
    fn balance_unknown_address_ko() {
        create_db!(blob, db, 1, 256);
        assert_eq!(
            Err(Error::AddressUnknown("".to_string())),
            BalanceCmd {
                address: U256::from(ADDRESS)
            }
            .execute(&db)
        );
    }

    #[test]
    fn accounts_ok() {
        create_db!(blob, _db, 2, 256);
        assert_eq!(Ok(()), AccountsCmd {}.execute(&blob.accounts));
    }

    #[test]
    fn transfer_ko() {
        create_db!(blob, db, 2, 256);
        let accounts = blob.accounts;
        assert_eq!(
            Err(Error::TransactionFailed("connection error".to_string())),
            create_test_transfer(&accounts).execute(&mut db)
        );
    }
}
