mod command;
mod error;
mod parse;

use crate::package::blob;
use command::Command;
use dialoguer::{theme::CustomPromptCharacterTheme, Input};
use parse::parse_command;
use sheth::state::{Multiproof, State};
use std::io;
use std::io::prelude::*;

pub fn start(accounts: usize, tree_height: usize) {
    println!("Starting sheth client");

    print!("Initializing database ... ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let blob = blob::generate(accounts, 0, tree_height);
    let accounts = blob.accounts;
    let mut proof = blob.proof;
    let mut db = Multiproof::new(&mut proof, tree_height);

    println!("Ok.\n");

    let theme = CustomPromptCharacterTheme::new('>');

    loop {
        let command_str: String = Input::with_theme(&theme)
            .with_prompt("")
            .interact()
            .unwrap();

        let command = parse_command(command_str);

        let result = match command {
            Ok(Command::Balance(b)) => b.execute(&db),
            Ok(Command::Transfer(t)) => t.execute(&mut db),
            Ok(Command::Exit) => std::process::exit(0),
            Ok(Command::Accounts(a)) => a.execute(&accounts),
            Err(e) => Err(e),
        };

        if let Err(e) = result {
            println!("{:?}", e);
        }
    }
}
