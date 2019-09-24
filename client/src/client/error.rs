#[derive(Debug)]
pub enum Error {
    CommandUnknown(String),
    AmountInvalid(String),
    AddressInvalid(String),
    ArgumentsIncorrect(String),
}
