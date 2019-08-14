use crate::hash::hash;
use crate::state::Hash256;
use crate::u264::U264;
use arrayref::array_ref;
use bigint::U256;

///  Merkle tree schema
///
///       root
///     /      \
///  pubkey [nonce, value]
pub struct Account {
    pub pubkey: [u8; 48],
    pub nonce: u64,
    pub value: u64,
}

impl Account {
    pub fn root(&self) -> Hash256 {
        let mut buf = [0u8; 64];

        // Calculate account root
        buf[0..48].copy_from_slice(&self.pubkey);
        hash(&mut buf);

        // Put in nonce and value
        buf[32..40].copy_from_slice(&self.nonce.to_le_bytes());
        buf[40..48].copy_from_slice(&self.value.to_le_bytes());
        buf[48..64].copy_from_slice(&[0u8; 16]);

        hash(&mut buf);

        *array_ref![buf, 0, 32]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Address(U256);

impl From<usize> for Address {
    fn from(n: usize) -> Address {
        Address(n.into())
    }
}

impl From<Address> for U264 {
    fn from(address: Address) -> U264 {
        U264::from(address.0)
    }
}
