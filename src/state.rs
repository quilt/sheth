use crate::account::Address;
use crate::error::Error;
use crate::hash::hash;
use crate::u264::U264;
use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use arrayref::{array_mut_ref, array_ref};

#[derive(Debug, PartialEq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(arr: [u8; 32]) -> Self {
        H256(arr)
    }

    pub fn zero() -> Self {
        H256([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Offset at which the (index, chunk) pairs begin
const OFFSET: usize = core::mem::size_of::<u32>();

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
pub trait Backend<'a> {
    fn new(offsets: &'a [u8], db: &'a [u8], height: usize) -> Self;

    /// Calculates the root before making changes to the structure and after in one pass.
    fn root(&mut self) -> Result<H256, Error>;

    /// Increase the value of an account at `address`.
    fn add_value(&mut self, address: Address, amount: u64) -> Result<u64, Error>;

    /// Decrease the value of an account at `address`.
    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error>;

    /// Increment the `nonce` of the account at `address` by `1`.
    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error>;
}

pub struct InMemoryBackend<'a> {
    pub offsets: &'a [u8],
    pub db: &'a [u8],
    pub height: usize,
}

impl<'a> InMemoryBackend<'a> {
    pub fn get(&self, index: U264) -> H256 {
        let mut position = 0u64;
        let mut offset = 0u64;

        for i in 0..260 {
            let bit = (index << (260 - i - 1)) & 1.into();

            if bit == 0.into() {
                position += 1;
            } else {
                let skip = u64::from_le_bytes(*array_ref![self.offsets, position as usize, 8]);
                position += skip;
                offset += skip;
            }
        }

        H256::new(*array_ref![self.db, offset as usize, 32])
    }
}

fn helper(proof: &[u8], offsets: &[u8], offset: u64) -> Result<H256, Error> {
    if offsets.len() == 0 {
        return Ok(H256::new(*array_ref![proof, (offset * 32) as usize, 32]));
    }

    let mut left = H256::new(*array_ref![proof, (offset * 32) as usize, 32]);
    let mut right = H256::new(*array_ref![proof, ((offset + 1) * 32) as usize, 32]);

    if offsets[0] != 1 {
        left = helper(proof, &offsets[1..offsets[0] as usize], offset)?;
    }

    if offsets.len() != 1 {
        right = helper(
            proof,
            &offsets[offsets[0] as usize..],
            offsets[0] as u64 + offset,
        )?;
    }

    // Copy chunks into hashing buffer
    let mut buf = [0u8; 64];
    buf[0..32].copy_from_slice(left.as_bytes());
    buf[32..64].copy_from_slice(right.as_bytes());

    // Hash chunks
    hash(array_mut_ref![buf, 0, 64]);

    Ok(H256::new(*array_ref![buf, 0, 32]))
}

impl<'a> Backend<'a> for InMemoryBackend<'a> {
    fn new(offsets: &'a [u8], db: &'a [u8], height: usize) -> Self {
        Self {
            offsets,
            db,
            height,
        }
    }

    fn root(&mut self) -> Result<H256, Error> {
        helper(self.db, self.offsets, 0)
    }

    fn add_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        unimplemented!()
    }

    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        unimplemented!()
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn zh(depth: usize) -> H256 {
        let mut buf = [0u8; 64];
        crate::hash::zh(depth, &mut buf);
        H256::new(*array_ref![buf, 0, 32])
    }

    #[test]
    fn test_simple() {
        // indexes = [4, 10, 11, 3]
        let offsets: Vec<u8> = vec![3, 1, 1];
        let chunks: Vec<H256> = vec![zh(1), zh(0), zh(0), zh(2)];

        let mut proof: Vec<u8> = vec![];
        proof.extend(chunks[0].as_bytes());
        proof.extend(chunks[1].as_bytes());
        proof.extend(chunks[2].as_bytes());
        proof.extend(chunks[3].as_bytes());

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(3)))
    }

    #[test]
    fn test_full() {
        // indexes = [8, 9, 10, 11, 12, 13, 14, 15]
        let offsets: Vec<u8> = vec![4, 2, 1, 1, 2, 1, 1];
        let chunks: Vec<H256> = vec![zh(0), zh(0), zh(0), zh(0), zh(0), zh(0), zh(0), zh(0)];

        let mut proof: Vec<u8> = vec![];
        proof.extend(chunks[0].as_bytes());
        proof.extend(chunks[1].as_bytes());
        proof.extend(chunks[2].as_bytes());
        proof.extend(chunks[3].as_bytes());
        proof.extend(chunks[4].as_bytes());
        proof.extend(chunks[5].as_bytes());
        proof.extend(chunks[6].as_bytes());
        proof.extend(chunks[7].as_bytes());

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(3)))
    }

    #[test]
    fn test_large() {
        // indexes = [2, 6, 7168, 7169, 3585, 1793, 897, 449, 225, 113, 57, 29, 15]
        let offsets: Vec<u8> = vec![1, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let chunks: Vec<H256> = vec![
            zh(11),
            zh(10),
            zh(0),
            zh(0),
            zh(1),
            zh(2),
            zh(3),
            zh(4),
            zh(5),
            zh(6),
            zh(7),
            zh(8),
            zh(9),
        ];

        let mut proof: Vec<u8> = vec![];
        proof.extend(chunks[0].as_bytes());
        proof.extend(chunks[1].as_bytes());
        proof.extend(chunks[2].as_bytes());
        proof.extend(chunks[3].as_bytes());
        proof.extend(chunks[4].as_bytes());
        proof.extend(chunks[5].as_bytes());
        proof.extend(chunks[6].as_bytes());
        proof.extend(chunks[7].as_bytes());
        proof.extend(chunks[8].as_bytes());
        proof.extend(chunks[9].as_bytes());
        proof.extend(chunks[10].as_bytes());
        proof.extend(chunks[11].as_bytes());
        proof.extend(chunks[12].as_bytes());

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(12)))
    }
}
