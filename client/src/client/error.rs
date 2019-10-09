/// An enum of errors that can occur while running a client.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Error {
    /// Unable to parse command string into a new command
    CommandUnknown(String),

    /// The amount could not be parsed into an integer
    AmountInvalid(String),

    /// The address could not be parsed into a 32 byte value
    AddressInvalid(String),

    /// The arguments did not match the command's syntax
    ArgumentsIncorrect(String),

    /// The transaction was not able to be processed
    TransactionFailed(String),

    /// The address did not match a known account
    AddressUnknown(String),
}
