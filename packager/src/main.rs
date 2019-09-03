pub mod proof;
pub mod transactions;

use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};

fn main() {
    // Hard coded for now
    let tx_count = 50;
    let account_count = 100;
    let height = 256;
    // ---

    // let transactions = transactions::generate(tx_count, account_count);
    // let proof = proof::generate(tx_count, height);

    // let mut input = transactions::serialize(&transactions);
    // input.extend(proof.clone());

    // let mut mem = InMemoryBackend::new(height);

    // assert_eq!(mem.load(&proof), Ok(()));
    // assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

    // let roots = mem.roots().unwrap();

    // println!("beacon_state:");
    // println!("  execution_scripts:");
    // println!("    - scout/sheth.wasm");
    // println!("shard_pre_state:");
    // println!("  exec_env_states:");
    // println!("    - \"{}\"", hex::encode(roots.0));
    // println!("shard_blocks:");
    // println!("  - env: 0");
    // println!("    data: \"{}\"", hex::encode(input));
    // println!("shard_post_state:");
    // println!("  exec_env_states:");
    // println!("    - \"{}\"", hex::encode(roots.1));
}
