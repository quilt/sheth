//  Merkle tree schema
//
//       root
//     /      \
//  pubkey [nonce, value]
pub struct Account {
    pub pubkey: [u8; 32],
    pub nonce: u64,
    pub value: u64,
}
