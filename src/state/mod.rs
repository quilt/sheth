pub mod multiproof;

pub use multiproof::Multiproof;

use crate::address::Address;
use crate::error::Error;
use crate::hash::H256;

/// Interface for interacting with the state's Sparse Merkle Tree (SMT).
///
/// The SMT can be modeled as a `FixedVector[Account, 2**256]`. It's merkle tree structure is as
/// follows:
///
/// ```text
///            root
///           /    \
///         ...    ...    <= intermediate nodes
///         / \    / \
///        0   1  n n+1   <= account roots
/// ```
pub trait State<'a> {
    /// Instantiates a new `Backend`.
    fn new(db: &'a mut [u8], height: usize) -> Self;

    /// Calculates the root before making changes to the structure and after in one pass.
    fn root(&mut self) -> Result<H256, Error>;

    /// Returns the value of a specified address.
    fn value(&self, address: Address) -> Result<u64, Error>;

    /// Returns the nonce of a specified address.
    fn nonce(&self, address: Address) -> Result<u64, Error>;

    /// Increase the value of an account at `address`.
    fn add_value(&mut self, address: Address, amount: u64) -> Result<u64, Error>;

    /// Decrease the value of an account at `address`.
    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error>;

    /// Increment the `nonce` of the account at `address` by `1`.
    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error>;
}
