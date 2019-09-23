mod accounts;
mod blob;
mod proof;
mod transactions;

// use sheth::process::process_transactions;
use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};
use std::env;

fn main() {
    let args = env::args().collect();
    let config = blob::Configuration::init(&args);
    let mut blob = blob::generate(config);
    let mut mem = InMemoryBackend::new(&mut blob.proof, config.tree_height);

    let pre_state = mem.root().unwrap();
    assert_eq!(process_transactions(&mut mem, &blob.transactions), Ok(()));
    let post_state = mem.root().unwrap();

    if args.len() == 5 && args[4] == "--scout" {
        println!("beacon_state:");
        println!("  execution_scripts:");
        println!("    - scout/sheth.wasm");
        println!("shard_pre_state:");
        println!("  exec_env_states:");
        println!("    - \"{}\"", hex::encode(pre_state));
        println!("shard_blocks:");
        println!("  - env: 0");
        println!("    data: \"{}\"", hex::encode(blob.to_bytes()));
        println!("shard_post_state:");
        println!("  exec_env_states:");
        println!("    - \"{}\"", hex::encode(post_state));
    } else {
        println!(
            "{} {} {}",
            hex::encode(pre_state),
            hex::encode(&blob.to_bytes()),
            hex::encode(post_state)
        );
    }

    let s = blob.to_bytes();
    // assert_eq!(blob.transactions
    assert_eq!(blob.proof, &s[(4 + blob.transactions.len() * 176)..]);
}
