use crate::account::Address;
use crate::error::Error;
use crate::u264::U264;
use arrayref::array_ref;
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
    fn new() -> Self;
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
}

impl Backend for InMemoryBackend {
    fn new() -> Self {
        Self {
            db: std::collections::HashMap::new(),
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
            println!("i:{}, {:?}", i, index);
            self.db.insert(index, (chunk_buf, None));
        }

        Ok(())
    }

    fn roots(&mut self) -> Result<(Hash256, Hash256), Error> {
        unimplemented!()
    }

    fn add_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn sub_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        let key = (U264::from(address) << 1) + 1;

        let val = match self.db.get(&key) {
            // If there is a modified chunk, use that. Otherwise use the original value.
            Some(n) => n.1.unwrap_or(n.0),
            None => return Err(Error::ChunkNotLoaded),
        };

        let nonce = u64::from_le_bytes(*array_ref![val, 0, 8]);

        let (nonce, overflow) = nonce.overflowing_add(1);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut nonce_buf = [0u8; 32];
        nonce_buf[0..8].copy_from_slice(&nonce.to_le_bytes());

        self.db.insert(key, (val, Some(nonce_buf)));

        Ok(nonce)
    }
}
