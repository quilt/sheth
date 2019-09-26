use crate::address::Address;
use crate::error::Error;
use crate::state::State;

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

    pub fn nonce(&self) -> u64 {
        match self {
            Transaction::Transfer(t) => t.nonce,
            Transaction::Withdrawal(_) => unimplemented!(),
            Transaction::Deposit(_) => unimplemented!(),
        }
    }

    pub fn verify<'a, T: State>(&self, db: &T) -> Result<(), Error> {
        self.verify_signature(db)?;
        self.verify_nonce(db)?;

        Ok(())
    }

    pub fn verify_signature<'a, T: State>(&self, _db: &T) -> Result<(), Error> {
        // TODO: Implement BLS verification
        Ok(())
    }

    pub fn verify_nonce<'a, T: State>(&self, db: &T) -> Result<(), Error> {
        let nonce = db.nonce(self.from())?;

        if nonce == self.nonce() {
            Ok(())
        } else {
            Err(Error::NonceInvalid)
        }
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

#[cfg(test)]
mod test {
    use super::*;

    fn build_transfer() -> Transaction {
        Transaction::Transfer(Transfer {
            to: 0.into(),
            from: 1.into(),
            nonce: 3,
            amount: 4,
            signature: [0u8; 96],
        })
    }

    #[test]
    fn general_from() {
        let transfer = build_transfer();
        assert_eq!(transfer.from(), 1.into());
    }

    #[test]
    fn general_nonce() {
        let transfer = build_transfer();
        assert_eq!(transfer.nonce(), 3);
    }
}
