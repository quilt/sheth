use crate::account::Address;
use crate::error::Error;
use crate::hash::hash;
use crate::u264::U264;
use arrayref::{array_mut_ref, array_ref};
use bigint::U256;

pub type Hash256 = [u8; 32];

/// The state is represented as a Sparse Merkle Tree.
/// FixedVector[Account,
///
///            root
///           /    \
///         ...    ...    <= intermediate nodes
///         / \    / \
///        0   1  n n+1   <= account roots
///
///  Proof structure:
///
///  First 4 bytes defines the number of chunks.

/// Interface for interacting with the state's Sparse Merkle Tree.
pub trait Backend {
    fn new(height: usize) -> Self;
    /// Loads a serialized proof into storage.
    fn load(&mut self, proof: &[u8]) -> Result<(), Error>;

    /// Calculates the root before making changes to the structure and after in one pass.
    fn roots(&mut self) -> Result<(Hash256, Hash256), Error>;

    /// Increase the value of an account at `address`.
    fn add_value(&mut self, amount: u64, address: U256) -> Result<u64, Error>;

    /// Decrease the value of an account at `address`.
    fn sub_value(&mut self, amount: u64, address: U256) -> Result<u64, Error>;

    /// Increment the `nonce` of the account at `address` by `1`.
    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error>;
}

pub struct InMemoryBackend {
    pub db: std::collections::HashMap<U264, (Hash256, Option<Hash256>)>,
    pub height: usize,
}

impl Backend for InMemoryBackend {
    fn new(height: usize) -> Self {
        Self {
            db: std::collections::HashMap::new(),
            height,
        }
    }

    fn load(&mut self, input: &[u8]) -> Result<(), Error> {
        let n = array_ref![input, 0, 4];
        let n = u32::from_le_bytes(*n);
        let n = n as usize;

        let mut index_buf = [0u8; 33];
        let mut chunk_buf = [0u8; 32];

        const INITIAL_OFFSET: usize = core::mem::size_of::<u32>();
        const PAIR_SIZE: usize = 32 + 33;

        for i in 0..n {
            let begin = (i * PAIR_SIZE) + INITIAL_OFFSET;
            let end = ((i + 1) * PAIR_SIZE) + INITIAL_OFFSET;
            index_buf.copy_from_slice(&input[begin..begin + 33]);
            chunk_buf.copy_from_slice(&input[begin + 33..end]);
            let index = unsafe { std::mem::transmute::<[u8; 33], U264>(index_buf) };
            self.db.insert(index, (chunk_buf, None));
        }

        Ok(())
    }

    fn roots(&mut self) -> Result<(Hash256, Hash256), Error> {
        let mut buf = [0u8; 128];
        let mut indexes: Vec<U264> = self.db.keys().clone().map(|x| x.to_owned()).collect();
        indexes.sort_by(|a, b| (b).cmp(a));

        let mut position = 0;
        while position < indexes.len() {
            let left = indexes[position] & (!U264::zero() - 1.into());
            let right = left + 1.into();
            let parent = left >> 1;

            if self.db.contains_key(&left)
                && self.db.contains_key(&right)
                && !self.db.contains_key(&parent)
            {
                let left = self.db.get(&left).ok_or(Error::ChunkNotLoaded)?;
                let right = self.db.get(&right).ok_or(Error::ChunkNotLoaded)?;

                // Grab the unmodified chunks
                let left0 = left.0;
                let right0 = right.0;

                // Grab the modified chunks (if they exists, otherwise fall back to unmodified)
                let left1 = left.1.unwrap_or(left.0);
                let right1 = right.1.unwrap_or(right.0);

                // Copy chunks into hashing buffer
                buf[0..32].copy_from_slice(&left0);
                buf[32..64].copy_from_slice(&right0);
                buf[64..96].copy_from_slice(&left1);
                buf[96..128].copy_from_slice(&right1);

                // Hash chunks
                hash(array_mut_ref![buf, 0, 64]);
                hash(array_mut_ref![buf, 64, 64]);

                println!(
                    "({}, {}) | ({}, {}) => ({}, {})",
                    hex::encode(left0),
                    hex::encode(right0),
                    hex::encode(left1),
                    hex::encode(right1),
                    hex::encode(array_ref![buf, 0, 32]),
                    hex::encode(array_ref![buf, 64, 32]),
                );

                // Insert new hashes into db
                self.db.insert(
                    parent,
                    (*array_ref![buf, 0, 32], Some(*array_ref![buf, 64, 32])),
                );

                indexes.push(parent);
            }

            position += 1;
        }

        let root = self.db.get(&U264::one()).unwrap();
        Ok((root.0, root.1.unwrap()))
    }

    fn add_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn sub_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        // `nonce_index = (first_leaf + account) * 4 + 2`
        let key = (((U264::one() << self.height) + address.into()) << 2) + 1.into();
        println!("ay key: {:?}", key);

        let val = match self.db.get(&key) {
            // If there is a modified chunk, use that. Otherwise use the original value.
            Some(n) => (n.0, n.1.unwrap_or(n.0)),
            None => return Err(Error::ChunkNotLoaded),
        };

        let nonce = u64::from_le_bytes(*array_ref![val.1, 0, 8]);

        let (nonce, overflow) = nonce.overflowing_add(1);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut nonce_buf = [0u8; 32];
        nonce_buf[0..8].copy_from_slice(&nonce.to_le_bytes());

        self.db.insert(key, (val.0, Some(nonce_buf)));

        Ok(nonce)
    }
}
