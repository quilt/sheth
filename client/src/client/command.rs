use super::error::Error;
use bigint::U256;

pub enum Command {
    Balance(BalanceCmd),
    Send(SendCmd),
    Accounts,
    Exit,
}

pub trait ExecutableCommand {
    fn execute(&self) -> Result<(), Error>;
}

pub struct BalanceCmd {
    pub(crate) address: U256,
}

impl ExecutableCommand for BalanceCmd {
    fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct SendCmd {
    pub(crate) from: U256,
    pub(crate) to: U256,
    pub(crate) amount: u64,
}

impl ExecutableCommand for SendCmd {
    fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}
