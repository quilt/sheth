pub mod blob;

use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};

pub fn build(accounts: usize, transactions: usize, height: usize, scout: bool) {
    let initial_blob = blob::generate(accounts, transactions, height);
    let mut blob = blob::generate(accounts, transactions, height);

    let mut mem = InMemoryBackend::new(&mut blob.proof, height);

    let pre_state = mem.root().unwrap();
    assert_eq!(process_transactions(&mut mem, &blob.transactions), Ok(()));
    let post_state = mem.root().unwrap();

    if scout {
        println!("beacon_state:");
        println!("  execution_scripts:");
        println!("    - scout/sheth.wasm");
        println!("shard_pre_state:");
        println!("  exec_env_states:");
        println!("    - \"{}\"", hex::encode(pre_state));
        println!("shard_blocks:");
        println!("  - env: 0");
        println!("    data: \"{}\"", hex::encode(initial_blob.to_bytes()));
        println!("shard_post_state:");
        println!("  exec_env_states:");
        println!("    - \"{}\"", hex::encode(post_state));
    } else {
        println!(
            "{} {} {}",
            hex::encode(pre_state),
            hex::encode(post_state),
            hex::encode(initial_blob.to_bytes()),
        );
    }
}
