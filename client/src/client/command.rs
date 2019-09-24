use super::error::Error;
use crate::accounts::AddressedAccount;
use arrayref::array_ref;
use bigint::U256;
use sheth::state::{address_to_value_index, InMemoryBackend};

pub enum Command {
    Balance(BalanceCmd),
    Send(SendCmd),
    Accounts(AccountsCmd),
    Exit,
}

pub struct BalanceCmd {
    pub(crate) address: U256,
}

impl BalanceCmd {
    pub fn execute(&self, db: &InMemoryBackend) -> Result<(), Error> {
        // todo get tree heigh from init
        let index = address_to_value_index(self.address.into(), 256);
        let chunk = db.get(index);
        let value = u64::from_le_bytes(*array_ref![chunk, 0, 8]);

        println!("Balance is: {}", value);

        Ok(())
    }
}

pub struct SendCmd {
    pub(crate) from: U256,
    pub(crate) to: U256,
    pub(crate) amount: u64,
}

impl SendCmd {
    pub fn execute(&self, db: &mut InMemoryBackend) -> Result<(), Error> {
        Ok(())
    }
}

pub struct AccountsCmd();

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
