use crate::proof::accounts::random_accounts;
use crate::proof::offsets::calculate as calculate_offsets;
use crate::proof::uncompressed::generate as generate_uncompressed_proof;

pub fn generate(num_accounts: u64, height: usize) -> Vec<u8> {
    let accounts = random_accounts(num_accounts);
    let proof = generate_uncompressed_proof(accounts, height);
    let offsets = calculate_offsets(proof.indexes);

    let ret = offsets.iter().fold(vec![], |mut acc, x| {
        acc.extend(&x.to_le_bytes());
        acc
    });

    proof.values.iter().fold(ret, |mut acc, x| {
        acc.extend(x.as_bytes());
        acc
    })
}
