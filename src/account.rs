use crate::address::Address;
use crate::bls::PublicKey;
use crate::state::TokenColor;
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
    pub red_value: u64,
    pub green_value: u64,
    pub blue_value: u64,
}

impl Account {
    pub fn zero() -> Self {
        Account {
            pubkey: PublicKey::zero(),
            nonce: 0,
            red_value: 0,
            green_value: 0,
            blue_value: 0,
        }
    }
}

/// Given an address and tree height, calculate the `value`'s general index.
///
/// ```text
/// value_index = (first_leaf + account) * 4 + 2
/// ```
#[inline]
pub fn calc_value_index(color: TokenColor, address: Address, height: usize) -> U264 {
    ((((((U264::one() << height) + address.into()) << 2) + 2.into()) << 2) + (color as u8).into())
}

/// Given an address and tree height, calculate the `nonce`'s general index.
///
/// ```text
/// nonce_index = (first_leaf + account) * 4 + 1
/// ```
#[inline]
pub fn calc_nonce_index(address: Address, height: usize) -> U264 {
    ((((U264::one() << height) + address.into()) << 2) + 1.into()) << 2
}
