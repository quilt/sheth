use bigint::U256;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sha2::{Digest, Sha256};
use sheth::account::Account;
use sheth::bls::PublicKey;
use std::collections::HashMap;

/// A tuple consisting of an `Account` and its address.
///
/// The address is important for other stages in the `Blob` generation process since it defines
/// where in the multi-proof the account resides.
#[derive(Clone)]
pub struct AddressedAccount(pub U256, pub Account);

pub fn random_accounts(n: usize, height: usize) -> Vec<AddressedAccount> {
    // TODO: check that the number of accounts can be generated from the tree

    let mut rng = StdRng::seed_from_u64(42);
    let mut map: HashMap<U256, bool> = HashMap::new();

    (0..n).fold(vec![], |mut acc, _| {
        let mut pubkey = [0u8; 48];
        let address = loop {
            rng.fill(&mut pubkey[..]);

            // Hash public key to get address
            let mut address = U256::from(Sha256::digest(&pubkey).as_ref());

            if height < 256 {
                address = address % (U256::one() << height);
            }

            if !map.contains_key(&address) {
                map.insert(address, true);
                break address;
            }
        };

        acc.push(AddressedAccount(
            address,
            Account {
                pubkey: PublicKey::new(pubkey),
                nonce: rng.gen(),
                red_value: rng.gen_range(1, 1000),
                green_value: rng.gen_range(1, 1000),
                blue_value: rng.gen_range(1, 1000),
            },
        ));

        let mut buf = [0u8; 32];
        address.to_big_endian(&mut buf);

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
            assert_eq!(
                address,
                U256::from(Sha256::digest(&account.pubkey.as_bytes()).as_ref())
            );

            assert_ne!(account.pubkey.as_bytes().to_vec(), [0u8; 48].to_vec());
            assert_ne!(account.nonce, 0);
            assert_ne!(account.red_value, 0);
            assert_ne!(account.green_value, 0);
            assert_ne!(account.blue_value, 0);
        }
    }
}
