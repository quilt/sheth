use crate::address::Address;
use crate::bls::PublicKey;
use crate::u264::U264;

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
#[cfg_attr(feature = "std", derive(Hash))]
pub struct Account {
    pub pubkey: PublicKey,
    pub nonce: u64,
    pub value: u64,
}

impl Account {
    pub fn zero() -> Self {
        Account {
            pubkey: PublicKey::zero(),
            nonce: 0,
            value: 0,
        }
    }
}

/// Given an address and tree height, calculate the `value`'s general index.
///
/// ```text
/// value_index = (first_leaf + account) * 4 + 2
/// ```
#[inline]
pub fn calc_value_index(address: Address, height: usize) -> U264 {
    ((((U264::one() << height) + address.into()) << 2) + 2.into()) << 1
}

/// Given an address and tree height, calculate the `nonce`'s general index.
///
/// ```text
/// nonce_index = (first_leaf + account) * 4 + 1
/// ```
#[inline]
pub fn calc_nonce_index(address: Address, height: usize) -> U264 {
    ((((U264::one() << height) + address.into()) << 2) + 3.into()) << 1
}
