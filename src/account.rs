use crate::hash::hash;
use crate::state::Hash256;
use crate::u264::U264;
use arrayref::array_ref;
use bigint::U256;

///  Account merkle tree schema:
///
///       root
///     /      \
///  pubkey  other_root
///           /      \
///         nonce   value
#[derive(Clone)]
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

        // hash nonce + pubkey
        buf[32..40].copy_from_slice(&self.nonce.to_le_bytes());
        buf[40..64].copy_from_slice(&[0u8; 24]);
        hash(&mut buf);

        // hash value + padding
        let mut buf2 = [0u8; 64];
        buf2[0..8].copy_from_slice(&self.value.to_le_bytes());
        hash(&mut buf2);

        // hash 8 + 9
        buf[32..64].copy_from_slice(&buf2[0..32]);
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

impl From<U256> for Address {
    fn from(n: U256) -> Address {
        Address(n)
    }
}

impl From<[u8; 32]> for Address {
    fn from(arr: [u8; 32]) -> Address {
        Address(arr.into())
    }
}

impl From<Address> for U264 {
    fn from(address: Address) -> U264 {
        U264::from(address.0)
    }
}

impl From<Address> for [u8; 32] {
    fn from(a: Address) -> [u8; 32] {
        a.0.into()
    }
}
