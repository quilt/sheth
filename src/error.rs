use crate::U264;

#[derive(Debug, PartialEq)]
pub enum Error {
    SignatureInvalid,
    NonceInvalid,
    BalanceInsufficient,
    ChunkNotLoaded(U264),
    Overflow,
}
