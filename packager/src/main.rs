pub mod proof;
pub mod transactions;

use arrayref::array_ref;
use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let height = args[1]
        .parse::<usize>()
        .expect("Height should be a number.");

    let account_count = args[2]
        .parse::<usize>()
        .expect("Account count should be a number.");

    let tx_count = args[3]
        .parse::<usize>()
        .expect("Transaction count should be a number.");

    println!(
        "Height => {}\nAccounts => {}\nTransactions => {}",
        height, account_count, tx_count
    );
    // let transactions = transactions::serialize(&transactions::generate(tx_count, account_count));
    let proof = proof::generate(account_count, height);

    let mut input = proof;
    // input.extend(transactions.clone());

    let mut mem = InMemoryBackend::new(&mut input, height);
    // assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

    let pre_state_root = mem.root().unwrap();

    println!("beacon_state:");
    println!("  execution_scripts:");
    println!("    - scout/sheth.wasm");
    println!("shard_pre_state:");
    println!("  exec_env_states:");
    println!("    - \"{}\"", hex::encode(&pre_state_root));
    println!("shard_blocks:");
    println!("  - env: 0");
    println!("    data: \"{}\"", hex::encode(input));
    println!("shard_post_state:");
    println!("  exec_env_states:");
    // println!("    - \"{}\"", hex::encode(roots.1));
}
