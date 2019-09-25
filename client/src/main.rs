#[macro_use]
extern crate clap;

mod accounts;
mod client;
mod package;
mod proof;
mod transactions;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("sheth-client")
        .version("0.0.1")
        .author("Matt G. <git@garnett.dev>")
        .about("Builds transaction packages for the Sheth EE")
        .subcommand(
            SubCommand::with_name("package")
                .about("Builds a random transaction package")
                .version("0.0.1")
                .arg(
                    Arg::with_name("accounts")
                        .required(true)
                        .help("number of accounts that will be represented in the proof"),
                )
                .arg(
                    Arg::with_name("transactions")
                        .required(true)
                        .help("number of transactions to be generated"),
                )
                .arg(
                    Arg::with_name("height")
                        .long("height")
                        .short("d")
                        .takes_value(true)
                        .default_value("256")
                        .help("defines the height of sparse state structure"),
                )
                .arg(
                    Arg::with_name("scout")
                        .long("scout")
                        .help("When set, the output will be in the format of a Scout YAML file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Starts a Sheth client")
                .arg(
                    Arg::with_name("accounts")
                        .required(true)
                        .help("number of accounts that will be represented in the proof"),
                )
                .arg(
                    Arg::with_name("height")
                        .long("height")
                        .short("d")
                        .takes_value(true)
                        .default_value("256")
                        .help("defines the height of sparse state structure"),
                ),
        )
        .get_matches();

    // Run packager
    if let Some(matches) = matches.subcommand_matches("package") {
        let accounts = value_t!(matches.value_of("accounts"), usize).unwrap_or_else(|e| e.exit());
        let txs = value_t!(matches.value_of("transactions"), usize).unwrap_or_else(|e| e.exit());
        let height = value_t!(matches.value_of("height"), usize).unwrap_or_else(|e| e.exit());
        let scout = matches.is_present("scout");

        let output = package::build(accounts, txs, height, scout);
        println!("{}", output);
    }

    // Start client
    if let Some(matches) = matches.subcommand_matches("start") {
        let accounts = value_t!(matches.value_of("accounts"), usize).unwrap_or_else(|e| e.exit());
        let height = value_t!(matches.value_of("height"), usize).unwrap_or_else(|e| e.exit());

        client::start(accounts, height);
    }
}
