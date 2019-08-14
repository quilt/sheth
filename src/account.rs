use crate::hash::hash;
use crate::state::Hash256;
use crate::u264::U264;
use arrayref::array_ref;
use bigint::U256;

///  Merkle tree schema
///
///       root
///     /      \
///  pubkey  other_root
///           /      \
///         nonce   value
pub struct Account {
    pub pubkey: [u8; 48],
    pub nonce: u64,
    pub value: u64,
}

impl Account {
    // TODO: fix for new structure
    pub fn root(&self) -> Hash256 {
        let mut buf = [0u8; 64];

        // Calculate account root
        buf[0..48].copy_from_slice(&self.pubkey);
        hash(&mut buf);

        println!("16: {:?}", hex::encode(&buf[0..32]));

        // hash nonce + pubkey
        buf[32..40].copy_from_slice(&self.nonce.to_le_bytes());
        buf[40..64].copy_from_slice(&[0u8; 24]);
        hash(&mut buf);

        println!("8: {:?}", hex::encode(&buf[0..32]));

        // hash value + padding
        let mut buf2 = [0u8; 64];
        buf2[0..8].copy_from_slice(&self.value.to_le_bytes());
        hash(&mut buf2);

        println!("9: {:?}", hex::encode(&buf2[0..32]));

        // hash 8 + 9
        buf[32..64].copy_from_slice(&buf2[0..32]);
        hash(&mut buf);

        println!("account root: {:?}", &buf[0..32]);
        println!("account root: {:?}", hex::encode(&buf[0..32]));

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
