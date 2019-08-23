use crate::account::Address;
use crate::error::Error;
use crate::state::Backend;
use arrayref::array_ref;
use bigint::U256;
use core::fmt::{self, Write};
use core::mem::transmute;

#[repr(C)]
pub enum Transaction<'a> {
    Transfer(&'a Transfer),
    Withdrawal(&'a Transfer),
    Deposit(&'a Deposit),
}

impl<'a> Transaction<'a> {
    pub fn from(&self) -> &Address {
        match self {
            Transaction::Transfer(t) => t.from(),
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

impl<'a> fmt::Display for Transaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transaction::Transfer(t) => {
                writeln!(f, "to     : {:?}", t.to())?;
                writeln!(f, "from   : {:?}", t.from())?;
                writeln!(f, "nonce  : {:?}", t.nonce())?;
                writeln!(f, "amount : {:?}", t.amount())?;
            }
            Transaction::Withdrawal(w) => unimplemented!(),
            Transaction::Deposit(d) => unimplemented!(),
        }

        Ok(())
    }
}

#[repr(packed)]
pub struct Transfer([u8; 176]);

impl Transfer {
    pub fn new(
        to: Address,
        from: Address,
        nonce: u64,
        amount: u64,
        signature: [u8; 96],
    ) -> Transfer {
        let mut t = Transfer([0u8; 176]);
        t.0[0..32].copy_from_slice(&<[u8; 32]>::from(to));
        t.0[32..64].copy_from_slice(&<[u8; 32]>::from(from));
        t.0[64..72].copy_from_slice(&nonce.to_le_bytes());
        t.0[72..80].copy_from_slice(&amount.to_le_bytes());
        t.0[80..176].copy_from_slice(&signature);
        t
    }

    pub fn to(&self) -> &Address {
        unsafe { transmute::<&[u8; 32], &Address>(array_ref![self.0, 0, 32]) }
    }

    pub fn from(&self) -> &Address {
        unsafe { transmute::<&[u8; 32], &Address>(array_ref![self.0, 32, 32]) }
    }

    pub fn nonce(&self) -> &u64 {
        unsafe { transmute::<&[u8; 8], &u64>(array_ref![self.0, 64, 8]) }
    }

    pub fn amount(&self) -> &u64 {
        unsafe { transmute::<&[u8; 8], &u64>(array_ref![self.0, 72, 8]) }
    }

    pub fn signature(&self) -> &[u8; 96] {
        array_ref![self.0, 80, 96]
    }

    pub fn as_bytes(&self) -> &[u8; 176] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; 176] {
        self.0
    }
}

pub struct Withdrawal;
pub struct Deposit;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transfer() {
        let raw: [u8; 176] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ];

        let tx: &Transfer = unsafe { transmute::<&[u8; 176], &Transfer>(&raw) };

        assert_eq!(tx.to(), <&Address>::from(array_ref![raw, 0, 32]));
        assert_eq!(tx.from(), <&Address>::from(array_ref![raw, 32, 32]));
        assert_eq!(tx.nonce(), &1);
        assert_eq!(tx.amount(), &9);
        // assert_eq!(tx.signature(), array_ref![raw, 80, 96]);
    }

}
