use bigint::U512;
use rand::{random, thread_rng, RngCore};
use sha2::{Digest, Sha256};
use sheth::account::Account;

pub struct AddressedAccount(pub U512, pub Account);

pub fn random_accounts(n: u64) -> Vec<AddressedAccount> {
    (0..n).fold(vec![], |mut acc, x| {
        let mut pubkey = [0u8; 48];
        thread_rng().fill_bytes(&mut pubkey);

        // Hash public key to get address
        let address = U512::from(Sha256::digest(&pubkey).as_ref());

        acc.push(AddressedAccount(
            address,
            Account {
                pubkey,
                nonce: random(),
                value: random(),
            },
        ));

        acc
    })
}
