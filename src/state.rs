use crate::error::Error;

/// The state is represented as a Sparse Merkle Tree.
/// FixedVector[Account,
///
///            root
///           /    \
///         ...    ...    <= intermediate nodes
///         / \    / \
///        0   1  n n+1   <= account roots
pub struct SerializedProof {
    indexes: Vec<u128>,
    chunks: Vec<Vec<u8>>,
}

/// Interface for interacting with the state's Sparse Merkle Tree.
pub trait Backend {
    /// Loads a serialized proof into storage.
    fn load(&mut self, p: &SerializedProof) -> Result<(), Error>;

    /// Calculates the root before making changes to the structure and after in one pass.
    fn roots(&mut self) -> Result<([u8; 32], [u8; 32]), Error>;

    /// Increase the value of an account at `address`.
    fn add_value(&mut self, amount: u64, address: &[u8; 32]) -> Result<u64, Error>;

    /// Decrease the value of an account at `address`.
    fn sub_value(&mut self, amount: u64, address: &[u8; 32]) -> Result<u64, Error>;

    /// Increment the `nonce` of the account at `address` by `1`.
    fn inc_nonce(&mut self, address: &[u8; 32]) -> Result<u64, Error>;
}

pub struct InMemoryBackend {
    db: std::collections::HashMap<u128, [u8; 32]>,
}

impl Backend for InMemoryBackend {
    fn load(&mut self, proof: &SerializedProof) -> Result<(), Error> {
        let buf = [0u8; 32];

        for (i, index) in proof.indexes.iter().enumerate() {
            buf.copy_from_slice(&proof.chunks[i][0..32]);
            self.db.insert(*index, buf);
        }

        Ok(())
    }

    fn roots(&mut self) -> Result<([u8; 32], [u8; 32]), Error> {
        unimplemented!()
    }

    fn add_value(&mut self, amount: u64, address: &[u8; 32]) -> Result<u64, Error> {
        unimplemented!()
    }

    fn sub_value(&mut self, amount: u64, address: &[u8; 32]) -> Result<u64, Error> {
        unimplemented!()
    }

    fn inc_nonce(&mut self, address: &[u8; 32]) -> Result<u64, Error> {
        unimplemented!()
    }
}
