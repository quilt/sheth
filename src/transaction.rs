use crate::account::Address;
use crate::error::Error;
use crate::state::Backend;

pub enum Transaction {
    Transfer(Transfer),
    Withdrawal(Transfer),
    Deposit(Deposit),
}

impl Transaction {
    pub fn from(&self) -> Address {
        match self {
            Transaction::Transfer(t) => t.from,
            Transaction::Withdrawal(_) => unimplemented!(),
            Transaction::Deposit(_) => unimplemented!(),
        }
    }

    pub fn verify<'a, T: Backend<'a>>(&self, db: &T) -> Result<(), Error> {
        self.verify_signature(db)?;
        self.verify_nonce(db)?;

        Ok(())
    }

    pub fn verify_signature<'a, T: Backend<'a>>(&self, _db: &T) -> Result<(), Error> {
        // TODO: Implement BLS verification
        Ok(())
    }

    pub fn verify_nonce<'a, T: Backend<'a>>(&self, _db: &T) -> Result<(), Error> {
        // TODO: Verify nonce
        Ok(())
    }
}

pub struct Transfer {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    pub amount: u64,
    pub signature: [u8; 96],
}

pub struct Withdrawal;
pub struct Deposit;
