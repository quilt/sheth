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
            Transaction::Withdrawal(w) => unimplemented!(),
            Transaction::Deposit(d) => unimplemented!(),
        }
    }

    pub fn verify<T>(&self, db: &T) -> Result<(), Error>
    where
        T: Backend,
    {
        self.verify_signature(db)?;
        self.verify_nonce(db)?;

        Ok(())
    }

    pub fn verify_signature<T>(&self, db: &T) -> Result<(), Error>
    where
        T: Backend,
    {
        // TODO: Implement BLS verification
        Ok(())
    }

    pub fn verify_nonce<T>(&self, db: &T) -> Result<(), Error>
    where
        T: Backend,
    {
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
