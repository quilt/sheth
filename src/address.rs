use crate::u264::U264;
use bigint::{U256, U512};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Address(U256);

impl Address {
    pub const fn new(n: U256) -> Address {
        Address(n)
    }
}

impl From<usize> for Address {
    fn from(n: usize) -> Address {
        Address(n.into())
    }
}

impl From<U256> for Address {
    fn from(n: U256) -> Address {
        Address(n)
    }
}

impl From<U512> for Address {
    fn from(n: U512) -> Address {
        Address(n.into())
    }
}

impl From<[u8; 32]> for Address {
    fn from(arr: [u8; 32]) -> Address {
        Address(arr.into())
    }
}

impl From<Address> for U264 {
    fn from(address: Address) -> U264 {
        U264::from(address.0)
    }
}

impl From<Address> for [u8; 32] {
    fn from(a: Address) -> [u8; 32] {
        a.0.into()
    }
}
