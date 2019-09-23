use crate::accounts::random_accounts;
use crate::proof::offsets::calculate as calculate_offsets;
use crate::proof::uncompressed::generate as generate_uncompressed_proof;
use crate::transactions;
use sheth::transaction::Transaction;

#[derive(Clone, Copy, Debug)]
pub struct Configuration {
    pub accounts: usize,
    pub transactions: usize,
    pub tree_height: usize,
}

impl Configuration {
    pub fn init(args: &Vec<String>) -> Self {
        let tree_height = args[1]
            .parse::<usize>()
            .expect("Height should be a number.");

        let accounts = args[2]
            .parse::<usize>()
            .expect("Account count should be a number.");

        let transactions = args[3]
            .parse::<usize>()
            .expect("Transaction count should be a number.");

        Configuration {
            accounts,
            transactions,
            tree_height,
        }
    }
}

pub struct Blob {
    pub proof: Vec<u8>,
    pub transactions: Vec<Transaction>,
}

impl Blob {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = transactions::serialize(&self.transactions);
        ret.extend(&self.proof);
        ret
    }
}

pub fn generate(config: Configuration) -> Blob {
    let accounts = random_accounts(config.accounts, config.tree_height);
    let proof = generate_uncompressed_proof(accounts.clone(), config.tree_height);
    let offsets = calculate_offsets(proof.indexes);
    let transactions = transactions::generate(config.transactions, accounts);

    let mut compressed_proof = offsets.iter().fold(vec![], |mut acc, x| {
        acc.extend(&x.to_le_bytes());
        acc
    });

    compressed_proof = proof.values.iter().fold(compressed_proof, |mut acc, x| {
        acc.extend(x.as_bytes());
        acc
    });

    Blob {
        proof: compressed_proof,
        transactions,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use arrayref::array_ref;
    use sheth::state::{Backend, InMemoryBackend};

    #[test]
    fn generate_small_tree() {
        // Indexes = [16, 17, 9, 10, 11, 3]
        //
        // Manually verified:
        //
        // 12 => h(24, 25) => "fe3e04bdedd9ccc59568328661a965c292c978a6e815542100ce20450042ce5d"
        // 6  => h(12, 13) => "896f8cfbe5dc1a0071d091c9a5587c90caa54fa7ace3b20fee762121fabe8071"
        // 7  => h(14, 15) => "f3c4e41f0b0bdf6006e5d4e9e36ab78e32f3e8988723475be307fdad986b88e3"
        // 3  => h(6, 7)   => "cc86176af0b56c8741cab9fed5311cf0055c31d0441125f27a17f9917d93774b"
        // 1  => h(2, 3)   => "f44277f53194537d77ea41cd5b7c8fd3b408a0942f6edc09a3feb4ee3e588d48"

        let mut proof = vec![
            0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 218, 109, 128,
            123, 247, 149, 16, 97, 70, 229, 130, 39, 117, 217, 20, 176, 39, 122, 101, 36, 15, 101,
            14, 212, 200, 167, 202, 119, 130, 78, 90, 223, 120, 72, 181, 215, 17, 188, 152, 131,
            153, 99, 23, 163, 249, 201, 2, 105, 213, 103, 113, 0, 93, 84, 10, 25, 24, 73, 57, 201,
            232, 208, 219, 42, 85, 242, 146, 169, 167, 93, 196, 41, 170, 134, 245, 251, 132, 117,
            101, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197, 33, 10, 45, 228, 168,
            212, 211, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180,
            32, 123, 235, 13, 65, 144, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let root = vec![
            244, 66, 119, 245, 49, 148, 83, 125, 119, 234, 65, 205, 91, 124, 143, 211, 180, 8, 160,
            148, 47, 110, 220, 9, 163, 254, 180, 238, 62, 88, 141, 72,
        ];

        let config = Configuration {
            accounts: 1,
            transactions: 0,
            tree_height: 1,
        };

        assert_eq!(generate(config).to_bytes(), proof);
        let mut mem = InMemoryBackend::new(&mut proof[4..], 1);
        assert_eq!(mem.root(), Ok(*array_ref![root, 0, 32]));
    }
}
