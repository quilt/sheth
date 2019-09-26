#[derive(Clone)]
pub struct PublicKey([u8; 48]);

impl PublicKey {
    pub fn new(bytes: [u8; 48]) -> Self {
        PublicKey(bytes)
    }

    pub fn zero() -> Self {
        PublicKey([0u8; 48])
    }

    pub fn one() -> Self {
        PublicKey([1u8; 48])
    }

    pub fn as_bytes(&self) -> [u8; 48] {
        self.0
    }
}

#[cfg(feature = "std")]
impl std::hash::Hash for PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.0[..], state)
    }
}
