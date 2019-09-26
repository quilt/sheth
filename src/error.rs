use crate::u264::U264;

#[derive(Debug, PartialEq)]
pub enum Error {
    SignatureInvalid,
    NonceInvalid,
    BalanceInsufficient,
    StateIncomplete(U264),
    Overflow,
}
