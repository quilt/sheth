use super::command::{AccountsCmd, BalanceCmd, Command, SendCmd};
use super::error::Error;
use arrayref::array_ref;
use bigint::U256;

pub fn parse_command(command: String) -> Result<Command, Error> {
    let command: Vec<&str> = command.split_whitespace().collect();

    if command.len() < 1 {
        // should error
        unimplemented!()
    }

    match command[0] {
        "balance" | "b" => Ok(Command::Balance(parse_balance(command[1..].to_vec())?)),
        "send" | "s" => Ok(Command::Send(parse_send(command[1..].to_vec())?)),
        "accounts" | "a" => Ok(Command::Accounts(parse_accounts(command[1..].to_vec())?)),
        "exit" | "e" => Ok(Command::Exit),
        _ => Err(Error::CommandUnknown(command[0].to_string())),
    }
}

pub fn parse_balance(balance_args: Vec<&str>) -> Result<BalanceCmd, Error> {
    if balance_args.len() != 1 {
        return Err(Error::ArgumentsIncorrect(balance_args.join(" ")));
    }

    let address = parse_address(balance_args[0])?;

    Ok(BalanceCmd { address })
}

pub fn parse_send(send_args: Vec<&str>) -> Result<SendCmd, Error> {
    if send_args.len() != 3 {
        return Err(Error::ArgumentsIncorrect(send_args.join(" ")));
    }

    let from = parse_address(send_args[0])?;
    let to = parse_address(send_args[1])?;
    let amount = send_args[2]
        .parse::<u64>()
        .map_err(|_| Error::AmountInvalid(send_args[2].to_string()))?;

    Ok(SendCmd { from, to, amount })
}

pub fn parse_address(s: &str) -> Result<U256, Error> {
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

pub fn parse_accounts(accounts_args: Vec<&str>) -> Result<AccountsCmd, Error> {
    if accounts_args.len() > 0 {
        return Err(Error::ArgumentsIncorrect(accounts_args.join(" ")));
    }

    Ok(AccountsCmd())
}

mod test {
    use super::*;

    #[test]
    fn parse_address() {}
}
