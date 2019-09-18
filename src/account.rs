use crate::u264::U264;
use bigint::{U256, U512};

///  Account merkle tree schema:
///
/// ```text
///       root
///     /      \
///  pubkey  other_root
///           /      \
///         nonce   value
/// ```
#[derive(Clone)]
pub struct Account {
    pub pubkey: [u8; 48],
    pub nonce: u64,
    pub value: u64,
}

impl Account {
    pub fn zero() -> Self {
        Account {
            pubkey: [0u8; 48],
            nonce: 0,
            value: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Address(U256);

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
