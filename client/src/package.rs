use composer::blob;

pub fn build(accounts: usize, transactions: usize, height: usize, scout: bool) -> String {
    let (blob, pre_state, post_state) = blob::generate_with_roots(accounts, transactions, height);

    if scout {
        format!(
            "\
beacon_state:
    execution_scripts:
        - scout/sheth.wasm
shard_pre_state:
    exec_env_states:
        - \"{}\"
shard_blocks:
    - env: 0
      data: \"{}\"
shard_post_state:
    exec_env_states:
        - \"{}\"",
            hex::encode(pre_state),
            hex::encode(blob.to_bytes()),
            hex::encode(post_state)
        )
    } else {
        format!(
            "{} {} {}",
            hex::encode(pre_state),
            hex::encode(post_state),
            hex::encode(blob.to_bytes()),
        )
    }
}
