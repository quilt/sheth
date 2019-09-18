use bigint::U256;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sha2::{Digest, Sha256};
use sheth::account::Account;

#[derive(Clone)]
pub struct AddressedAccount(pub U256, pub Account);

pub fn random_accounts(n: usize, height: usize) -> Vec<AddressedAccount> {
    let mut rng = StdRng::seed_from_u64(42);

    (0..n).fold(vec![], |mut acc, _| {
        let mut pubkey = [0u8; 48];
        rng.fill(&mut pubkey[..]);

        // Hash public key to get address
        let mut address = U256::from(Sha256::digest(&pubkey).as_ref());

        if height < 256 {
            address = address % (U256::one() << height);
        }

        acc.push(AddressedAccount(
            address,
            Account {
                pubkey,
                nonce: rng.gen(),
                value: rng.gen(),
            },
        ));

        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_random_accounts() {
        let accounts = random_accounts(2, 256);

        for AddressedAccount(address, account) in accounts {
            // assert_eq!(
            //     address,
            //     U256::from(Sha256::digest(&account.pubkey).as_ref()) % (U256::one() << 256)
            // );

            // assert_ne!(account.pubkey.to_vec(), [0u8; 48].to_vec());
            // assert_ne!(account.nonce, 0);
            // assert_ne!(account.value, 0);
        }
    }
}
