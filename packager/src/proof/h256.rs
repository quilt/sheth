use arrayref::array_ref;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(arr: &[u8; 32]) -> Self {
        H256(arr.clone())
    }

    pub fn zero() -> Self {
        H256([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

pub fn zh(depth: usize) -> H256 {
    let mut buf = [0u8; 64];
    sheth::hash::zh(depth, &mut buf);
    H256::new(array_ref![buf, 0, 32])
}
