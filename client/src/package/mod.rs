pub mod blob;

use sheth::process::process_transactions;
use sheth::state::{Backend, InMemoryBackend};

pub fn build(accounts: usize, transactions: usize, height: usize, scout: bool) -> String {
    let initial_blob = blob::generate(accounts, transactions, height);
    let mut blob = blob::generate(accounts, transactions, height);

    let mut mem = InMemoryBackend::new(&mut blob.proof, height);

    let pre_state = mem.root().unwrap();
    assert_eq!(process_transactions(&mut mem, &blob.transactions), Ok(()));
    let post_state = mem.root().unwrap();

    if scout {
        format!(
            "\
        beacon_state:
            execution_scripts:
                - scout/sheth.wasm
            shard_pre_state:
              exec_env_states:
                - \"{}\",
            shard_blocks:
              - env: 0
                data: \"{}\",
            shard_post_state:
              exec_env_states:
                - \"{}\"",
            hex::encode(pre_state),
            hex::encode(initial_blob.to_bytes()),
            hex::encode(post_state)
        )
    } else {
        format!(
            "{} {} {}",
            hex::encode(pre_state),
            hex::encode(post_state),
            hex::encode(initial_blob.to_bytes()),
        )
    }
}
