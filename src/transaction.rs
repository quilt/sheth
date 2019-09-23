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

#[cfg(feature = "std")]
impl std::fmt::Debug for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tto: {:?},\n\tfrom: {:?},\n\tnonce: {},\n\tamount: {},\n\t, signature: {:?}\n}}",
            self.to,
            self.from,
            self.nonce,
            self.amount,
            self.signature.to_vec()
        )
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
pub struct Withdrawal;

#[cfg_attr(feature = "std", derive(Debug))]
pub struct Deposit;
