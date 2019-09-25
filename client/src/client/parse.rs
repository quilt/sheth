use super::command::{AccountsCmd, BalanceCmd, Command, TransferCmd};
use super::error::Error;
use arrayref::array_ref;
use bigint::U256;

/// Parse a given string and return the resulting `Command` or `Error`.
pub fn parse_command(command: String) -> Result<Command, Error> {
    let command: Vec<&str> = command.split_whitespace().collect();

    match command[0] {
        "balance" | "b" => Ok(Command::Balance(parse_balance(command[1..].to_vec())?)),
        "transfer" | "t" => Ok(Command::Transfer(parse_transfer(command[1..].to_vec())?)),
        "accounts" | "a" => Ok(Command::Accounts(parse_accounts(command[1..].to_vec())?)),
        "exit" | "e" => Ok(Command::Exit),
        _ => Err(Error::CommandUnknown(command[0].to_string())),
    }
}

/// Parse the arguments for the balance command to determine the address to look up.
pub fn parse_balance(balance_args: Vec<&str>) -> Result<BalanceCmd, Error> {
    if balance_args.len() != 1 {
        return Err(Error::ArgumentsIncorrect(balance_args.join(" ")));
    }

    let address = parse_address(balance_args[0])?;

    Ok(BalanceCmd { address })
}

/// Parse the arguments for the transfer command to determine the values of the transaction that
/// will be built.
pub fn parse_transfer(transfer_args: Vec<&str>) -> Result<TransferCmd, Error> {
    if transfer_args.len() != 3 {
        return Err(Error::ArgumentsIncorrect(transfer_args.join(" ")));
    }

    let from = parse_address(transfer_args[0])?;
    let to = parse_address(transfer_args[1])?;
    let amount = transfer_args[2]
        .parse::<u64>()
        .map_err(|_| Error::AmountInvalid(transfer_args[2].to_string()))?;

    Ok(TransferCmd { from, to, amount })
}

/// Ensure that no arguments were given to the accounts command.
pub fn parse_accounts(accounts_args: Vec<&str>) -> Result<AccountsCmd, Error> {
    if accounts_args.len() > 0 {
        return Err(Error::ArgumentsIncorrect(accounts_args.join(" ")));
    }

    Ok(AccountsCmd())
}

/// Parse a hex string into a U256 value or return `Error`.
pub fn parse_address(s: &str) -> Result<U256, Error> {
    // If the address is prepended by `"0x"`, strip it.
    let s = if s.len() > 2 && &s[0..2] == "0x" {
        &s[2..]
    } else {
        &s
    };

    let bytes = hex::decode(s).map_err(|_| Error::AddressInvalid(s.to_string()))?;

    if bytes.len() != 32 {
        Err(Error::AddressInvalid(s.to_string()))
    } else {
        Ok(U256::from(array_ref![bytes, 0, 32]))
    }
}
