mod command;
mod error;
mod parse;

use command::{BalanceCmd, Command, ExecutableCommand, SendCmd};
use error::Error;

use crate::package::blob;
use dialoguer::{theme::CustomPromptCharacterTheme, Input};
use parse::parse_command;
use std::io;
use std::io::prelude::*;

pub fn start(accounts: usize, tree_depth: usize) {
    println!("Starting sheth client");
    print!("Initializing database ... ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let _ = blob::generate(accounts, 0, tree_depth);
    println!("Ok.\n");

    let theme = CustomPromptCharacterTheme::new('>');

    loop {
        let command_str: String = Input::with_theme(&theme)
            .with_prompt("")
            .interact()
            .unwrap();

        let command = parse_command(command_str);

        let result = match command {
            Ok(Command::Balance(b)) => b.execute(),
            Ok(Command::Send(s)) => s.execute(),
            Ok(Command::Exit) => {
                ::std::process::exit(0);
            }
            Ok(Command::Accounts) => Ok(()),
            Err(e) => Err(e),
        };

        if let Err(e) = result {
            println!("{:?}", e);
        }
    }
}
