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

impl From<u8> for H256 {
    fn from(n: u8) -> H256 {
        H256::new([
            n, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ])
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
    fn new(offsets: &'a [u8], db: &'a mut [u8], height: usize) -> Self;

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
    pub db: &'a mut [u8],
    pub height: usize,
}

impl<'a> InMemoryBackend<'a> {
    // TODO: add debug check that operations are occuring only on
    // leaf nodes
    pub fn get(&self, index: U264) -> H256 {
        let offset = self.lookup(index) * 32;
        H256::new(*array_ref![self.db, offset, 32])
    }

    pub fn update(&mut self, index: U264, value: H256) {
        let offset = self.lookup(index) * 32;
        self.db[offset..offset + 32].copy_from_slice(value.as_bytes());
    }

    fn lookup(&self, index: U264) -> usize {
        let mut position = 0u64;
        let mut offset = 0u64;

        for i in 1..(self.height + 4) {
            // TODO: abstract to U264
            let bit = (index >> (self.height + 3 - i)) & 1.into();

            if bit == 0.into() {
                position += 1;
            } else {
                let skip =
                    u64::from_le_bytes(*array_ref![self.offsets, (position * 8) as usize, 8]);
                position += skip;
                offset += skip;
            }
        }

        offset as usize
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
    fn new(offsets: &'a [u8], db: &'a mut [u8], height: usize) -> Self {
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
        // `value_index = (first_leaf + account) * 4 + 2`
        let index = ((((U264::one() << self.height) + address.into()) << 2) + 2.into()) << 1;
        let chunk = self.get(index);

        let value = u64::from_le_bytes(*array_ref![chunk.as_bytes(), 0, 8]);

        let (value, overflow) = value.overflowing_add(amount);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&value.to_le_bytes());
        self.update(index, H256::new(buf));

        Ok(value)
    }

    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        // `value_index = (first_leaf + account) * 4 + 2`
        let index = ((((U264::one() << self.height) + address.into()) << 2) + 2.into()) << 1;
        let chunk = self.get(index);

        let value = u64::from_le_bytes(*array_ref![chunk.as_bytes(), 0, 8]);

        let (value, overflow) = value.overflowing_sub(amount);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&value.to_le_bytes());
        self.update(index, H256::new(buf));

        Ok(value)
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        // `nonce_index = (first_leaf + account) * 4 + 1`
        let index = ((((U264::one() << self.height) + address.into()) << 2) + 3.into()) << 1;
        let chunk = self.get(index);

        let nonce = u64::from_le_bytes(*array_ref![chunk.as_bytes(), 0, 8]);

        let (nonce, overflow) = nonce.overflowing_add(1);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&nonce.to_le_bytes());
        self.update(index, H256::new(buf));

        Ok(nonce)
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
    fn lookup_small_branch() {
        // indexes = [4, 10, 11, 3]
        let offset_numbers: Vec<u64> = vec![3, 1, 1];
        let mut offsets: Vec<u8> = vec![];
        offsets.extend(&offset_numbers[0].to_le_bytes());
        offsets.extend(&offset_numbers[1].to_le_bytes());
        offsets.extend(&offset_numbers[2].to_le_bytes());

        let mut proof: Vec<u8> = vec![0; 32 * 4];
        let mem = InMemoryBackend::new(&offsets, &mut proof, 1);

        assert_eq!(mem.lookup((10 << 1).into()), 1);
        assert_eq!(mem.lookup((11 << 1).into()), 2);

        // Intermediate nodes need to be normalized to end nodes
        assert_eq!(mem.lookup((4 << 2).into()), 0);
        assert_eq!(mem.lookup((3 << 3).into()), 3);
    }

    #[test]
    fn lookup_single_account() {
        // indexes = [16, 17, 9, 10, 11, 3]
        let offset_numbers: Vec<u64> = vec![5, 3, 2, 1, 1];
        let mut offsets: Vec<u8> = vec![];

        for i in 0..5 {
            offsets.extend(&offset_numbers[i].to_le_bytes());
        }

        let mem = InMemoryBackend::new(&offsets, &mut [], 1);

        assert_eq!(mem.lookup((9 << 1).into()), 2);
        assert_eq!(mem.lookup((10 << 1).into()), 3);
        assert_eq!(mem.lookup((11 << 1).into()), 4);
        assert_eq!(mem.lookup(16.into()), 0);
        assert_eq!(mem.lookup(17.into()), 1);
    }

    #[test]
    fn lookup_full_tree() {
        // indexes = [8, 9, 10, 11, 12, 13, 14, 15]
        let offset_numbers: Vec<u64> = vec![4, 2, 1, 1, 2, 1, 1];
        let mut offsets: Vec<u8> = vec![];

        for offset in offset_numbers {
            offsets.extend(&offset.to_le_bytes());
        }

        let mem = InMemoryBackend::new(&offsets, &mut [], 1);

        for i in 0..7 {
            assert_eq!(mem.lookup(((i + 8) << 1).into()), i as usize);
        }
    }

    #[test]
    fn add_value() {
        // indexes = [16, 17, 9, 10, 11, 3]
        let offset_numbers: Vec<u64> = vec![5, 3, 2, 1, 1];
        let mut offsets: Vec<u8> = vec![];

        for offset in offset_numbers {
            offsets.extend(&offset.to_le_bytes());
        }

        let hash_chunks: Vec<H256> = vec![zh(0), zh(0), zh(0), H256::zero(), zh(0), zh(0)];
        let mut chunks = vec![];

        for chunk in hash_chunks {
            chunks.extend(chunk.as_bytes());
        }

        let mut mem = InMemoryBackend::new(&offsets, &mut chunks, 1);

        assert_eq!(mem.add_value(0.into(), 1), Ok(1));
        assert_eq!(mem.get((10 << 1).into()), 1.into());
    }

    #[test]
    fn sub_value() {
        // indexes = [16, 17, 9, 10, 11, 3]
        let offset_numbers: Vec<u64> = vec![5, 3, 2, 1, 1];
        let mut offsets: Vec<u8> = vec![];

        for offset in offset_numbers {
            offsets.extend(&offset.to_le_bytes());
        }

        let hash_chunks: Vec<H256> = vec![zh(0), zh(0), zh(0), 2.into(), zh(0), zh(0)];
        let mut chunks = vec![];

        for chunk in hash_chunks {
            chunks.extend(chunk.as_bytes());
        }

        let mut mem = InMemoryBackend::new(&offsets, &mut chunks, 1);

        assert_eq!(mem.sub_value(0.into(), 1), Ok(1));
        assert_eq!(mem.get((10 << 1).into()), 1.into());
    }

    #[test]
    fn inc_nonce() {
        // indexes = [16, 17, 9, 10, 11, 3]
        let offset_numbers: Vec<u64> = vec![5, 3, 2, 1, 1];
        let mut offsets: Vec<u8> = vec![];

        for offset in offset_numbers {
            offsets.extend(&offset.to_le_bytes());
        }

        let hash_chunks: Vec<H256> = vec![zh(0), zh(0), zh(0), 0.into(), 0.into(), zh(0)];
        let mut chunks = vec![];

        for chunk in hash_chunks {
            chunks.extend(chunk.as_bytes());
        }

        let mut mem = InMemoryBackend::new(&offsets, &mut chunks, 1);

        assert_eq!(mem.inc_nonce(0.into()), Ok(1));
        assert_eq!(mem.get((11 << 1).into()), 1.into());
    }

    #[test]
    fn root_simple_branch() {
        // indexes = [4, 10, 11, 3]
        let offsets: Vec<u8> = vec![3, 1, 1];
        let chunks: Vec<H256> = vec![zh(1), zh(0), zh(0), zh(2)];

        let mut proof: Vec<u8> = vec![];

        for i in 0..4 {
            proof.extend(chunks[i].as_bytes());
        }

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(3)))
    }

    #[test]
    fn root_full_tree() {
        // indexes = [8, 9, 10, 11, 12, 13, 14, 15]
        let offsets: Vec<u8> = vec![4, 2, 1, 1, 2, 1, 1];
        let chunks: Vec<H256> = vec![zh(0), zh(0), zh(0), zh(0), zh(0), zh(0), zh(0), zh(0)];
        let mut proof: Vec<u8> = vec![];

        for i in 0..8 {
            proof.extend(chunks[i].as_bytes());
        }

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(3)))
    }

    #[test]
    fn root_large_branch() {
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

        for i in 0..13 {
            proof.extend(chunks[i].as_bytes());
        }

        assert_eq!(helper(&proof, &offsets, 0), Ok(zh(12)))
    }
}
