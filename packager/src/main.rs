mod accounts;
mod blob;
mod proof;
mod transactions;

// use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};
use std::env;

fn main() {
    let config = blob::Configuration::init(env::args().collect());
    let mut blob = blob::generate(config);
    let mut mem = InMemoryBackend::new(&mut blob, config.tree_height);

    let pre_state_root = mem.root().unwrap();
    // assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));
    // let post_state_root = mem.root().unwrap();

    println!("beacon_state:");
    println!("  execution_scripts:");
    println!("    - scout/sheth.wasm");
    println!("shard_pre_state:");
    println!("  exec_env_states:");
    println!("    - \"{}\"", hex::encode(&pre_state_root));
    println!("shard_blocks:");
    println!("  - env: 0");
    println!("    data: \"{}\"", hex::encode(blob));
    println!("shard_post_state:");
    println!("  exec_env_states:");
    // println!("    - \"{}\"", hex::encode(roots.1));
}
