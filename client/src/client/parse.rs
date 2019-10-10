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

#[cfg(test)]
mod test {
    use super::*;

    const ADDRESS: &str = "b94F5eA0ba39494cE839613fffBA74279579268A742795792683424567896523";
    const PARSED_ADDRESS: [u8; 32] = [
        185, 79, 94, 160, 186, 57, 73, 76, 232, 57, 97, 63, 255, 186, 116, 39, 149, 121, 38, 138,
        116, 39, 149, 121, 38, 131, 66, 69, 103, 137, 101, 35,
    ];

    fn create_correct_balance_arguments<'a>() -> Vec<&'a str> {
        let mut arguments = vec![];
        arguments.push(ADDRESS);
        arguments
    }

    fn create_correct_balance() -> BalanceCmd {
        let address = U256::from(PARSED_ADDRESS);
        BalanceCmd { address }
    }

    fn create_correct_transfer() -> TransferCmd {
        let from_to = U256::from(PARSED_ADDRESS);
        TransferCmd {
            from: from_to,
            to: from_to,
            amount: 34,
        }
    }

    fn create_correct_transfer_arguments<'a>() -> Vec<&'a str> {
        let mut arguments = vec![];
        arguments.push(ADDRESS);
        arguments.push(ADDRESS);
        arguments.push("34");
        arguments
    }

    macro_rules! assert_arguments_incorrect {
        ($arguments_expr: expr, $parser_expr: expr, $error_string_expr: expr) => {
            assert_eq!(
                $parser_expr($arguments_expr).unwrap_err(),
                Error::ArgumentsIncorrect($error_string_expr.to_string())
            )
        };
    }

    macro_rules! assert_address_ok {
        ($address_expr: expr, $expected_expr: expr) => {
            assert_eq!(
                parse_address($address_expr).unwrap(),
                U256::from(*$expected_expr)
            );
        };
    }

    macro_rules! assert_address_invalid {
        ($address_expr: expr) => {
            assert_eq!(
                parse_address($address_expr).unwrap_err(),
                Error::AddressInvalid($address_expr.to_string())
            );
        };
    }

    #[test]
    fn parse_command_ok() {
        let correct_balance = Command::Balance(create_correct_balance());

        let mut long_arguments = vec!["balance"];
        long_arguments.append(&mut create_correct_balance_arguments());
        assert_eq!(
            parse_command(long_arguments.join(" ")).unwrap(),
            correct_balance
        );

        let mut short_arguments = vec!["b"];
        short_arguments.append(&mut create_correct_balance_arguments());
        assert_eq!(
            parse_command(short_arguments.join(" ")).unwrap(),
            correct_balance
        );

        let correct_transfer = Command::Transfer(create_correct_transfer());

        let mut long_arguments = vec!["transfer"];
        long_arguments.append(&mut create_correct_transfer_arguments());
        assert_eq!(
            parse_command(long_arguments.join(" ")).unwrap(),
            correct_transfer
        );

        let mut short_arguments = vec!["t"];
        short_arguments.append(&mut create_correct_transfer_arguments());
        assert_eq!(
            parse_command(short_arguments.join(" ")).unwrap(),
            correct_transfer
        );

        let correct_accounts = Command::Accounts(AccountsCmd {});

        assert_eq!(
            parse_command("accounts".to_string()).unwrap(),
            correct_accounts
        );

        assert_eq!(parse_command("a".to_string()).unwrap(), correct_accounts);

        let correct_exit = Command::Exit;

        assert_eq!(parse_command("exit".to_string()).unwrap(), correct_exit);

        assert_eq!(parse_command("e".to_string()).unwrap(), correct_exit);
    }

    #[test]
    fn parse_command_ko() {
        assert_eq!(
            parse_command("x".to_string()).unwrap_err(),
            Error::CommandUnknown("x".to_string())
        );
    }

    #[test]
    fn parse_balance_correct_arguments_ok() {
        assert_eq!(
            create_correct_balance(),
            parse_balance(create_correct_balance_arguments()).unwrap()
        );
    }

    #[test]
    fn parse_balance_wrong_arguments_ko() {
        let short_arguments = vec![];
        assert_arguments_incorrect!(short_arguments, parse_balance, "");

        let mut long_arguments = create_correct_balance_arguments();
        long_arguments.push(ADDRESS);
        assert_arguments_incorrect!(
            long_arguments,
            parse_balance,
            [ADDRESS, " ", ADDRESS].concat()
        );
    }

    #[test]
    fn parse_transfer_correct_arguments_ok() {
        assert_eq!(
            create_correct_transfer(),
            parse_transfer(create_correct_transfer_arguments()).unwrap()
        );
    }

    #[test]
    fn parse_transfer_wrong_arguments_ko() {
        let arguments = vec![];
        assert_arguments_incorrect!(arguments, parse_transfer, "");

        let mut short_arguments = create_correct_transfer_arguments();
        short_arguments.pop();
        short_arguments.pop();
        assert_arguments_incorrect!(short_arguments, parse_transfer, ADDRESS);

        let mut short_arguments = create_correct_transfer_arguments();
        short_arguments.pop();
        assert_arguments_incorrect!(
            short_arguments,
            parse_transfer,
            [ADDRESS, " ", ADDRESS].concat()
        );

        let mut wrong_arguments = create_correct_transfer_arguments();
        wrong_arguments[2] = "a";
        assert_eq!(
            Error::AmountInvalid("a".to_string()),
            parse_transfer(wrong_arguments).unwrap_err()
        );

        let mut long_arguments = create_correct_transfer_arguments();
        long_arguments.push("a");
        let check_long_arguments = long_arguments.clone();
        assert_arguments_incorrect!(
            long_arguments,
            parse_transfer,
            check_long_arguments.join(" ")
        );
    }

    #[test]
    fn parse_accounts_no_arguments_ok() {
        assert_eq!(AccountsCmd(), parse_accounts(vec!()).unwrap());
    }

    #[test]
    fn parse_accounts_with_arguments_ko() {
        let mut arguments = vec![];
        arguments.push("mine");
        let check_arguments = arguments.clone();
        assert_arguments_incorrect!(arguments, parse_accounts, check_arguments.join(" "));
    }

    #[test]
    fn parse_address_32b_prefixed_by_0x_ok() {
        let mut address = "0x".to_string();
        address.push_str(ADDRESS);
        assert_address_ok!(&address, &PARSED_ADDRESS);
    }

    #[test]
    fn parse_address_32b_not_prefixed_ok() {
        assert_address_ok!(ADDRESS, &PARSED_ADDRESS);
    }

    #[test]
    fn parse_address_wrong_length_ko() {
        let short_address = &ADDRESS[..31];

        assert_address_invalid!(short_address);

        let mut long_address = ADDRESS.to_string();
        long_address.push('3');

        assert_address_invalid!(&long_address);
    }
}
