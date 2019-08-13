use crate::account::Address;
use crate::error::Error;
use crate::u264::U264;
use arrayref::array_ref;
use u256::U256;

pub type Hash = [u8; 32];

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
    /// Loads a serialized proof into storage.
    fn load(&mut self, proof: &[u8]) -> Result<(), Error>;

    /// Calculates the root before making changes to the structure and after in one pass.
    fn roots(&mut self) -> Result<(Hash, Hash), Error>;

    /// Increase the value of an account at `address`.
    fn add_value(&mut self, amount: u64, address: U256) -> Result<u64, Error>;

    /// Decrease the value of an account at `address`.
    fn sub_value(&mut self, amount: u64, address: U256) -> Result<u64, Error>;

    /// Increment the `nonce` of the account at `address` by `1`.
    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error>;
}

pub struct InMemoryBackend {
    db: std::collections::HashMap<U264, Hash>,
}

impl Backend for InMemoryBackend {
    fn load(&mut self, input: &[u8]) -> Result<(), Error> {
        let n = array_ref![input, 0, 4];
        let n = u32::from_le_bytes(*n);
        let n = n as usize;

        let mut index_buf = [0u8; 33];
        let mut chunk_buf = [0u8; 32];

        for i in 0..n {
            index_buf.copy_from_slice(&input[..(i * 65) + 33]);
            chunk_buf.copy_from_slice(&input[(i * 65) + 33..(i + 1) * 65]);
            let index = unsafe { std::mem::transmute::<[u8; 33], U264>(index_buf) };
            self.db.insert(index, chunk_buf);
        }

        Ok(())
    }

    fn roots(&mut self) -> Result<(Hash, Hash), Error> {
        unimplemented!()
    }

    fn add_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn sub_value(&mut self, amount: u64, address: U256) -> Result<u64, Error> {
        unimplemented!()
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        let key = U264::from(address);

        let nonce = if let Some(n) = self.db.get(&key) {
            &n[0..8]
        } else {
            return Err(Error::ChunkNotLoaded);
        };

        let nonce = u64::from_le_bytes(*array_ref![nonce, 0, 8]);
        let (nonce, overflow) = nonce.overflowing_add(1);

        if overflow {
            return Err(Error::Overflow);
        }

        let mut nonce_buf = [0u8; 32];
        nonce_buf[0..8].copy_from_slice(&nonce.to_le_bytes());

        self.db.insert(key, nonce_buf);

        Ok(nonce)
    }
}
