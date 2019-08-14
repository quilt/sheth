#[derive(Debug)]
pub enum Error {
    SignatureInvalid,
    NonceInvalid,
    BalanceInsufficient,
    ChunkNotLoaded,
    Overflow,
}
