use crate::proof::accounts::random_accounts;
use crate::proof::offsets::calculate as calculate_offsets;
use crate::proof::uncompressed::generate as generate_uncompressed_proof;
use arrayref::array_ref;
use sheth::state::{Backend, InMemoryBackend};

pub fn generate(num_accounts: usize, height: usize) -> Vec<u8> {
    let accounts = random_accounts(num_accounts, height);
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

#[cfg(test)]
mod test {
    use super::*;

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
            6, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 218, 109, 128, 123, 247, 149,
            16, 97, 70, 229, 130, 39, 117, 217, 20, 176, 39, 122, 101, 36, 15, 101, 14, 212, 200,
            167, 202, 119, 130, 78, 90, 223, 120, 72, 181, 215, 17, 188, 152, 131, 153, 99, 23,
            163, 249, 201, 2, 105, 213, 103, 113, 0, 93, 84, 10, 25, 24, 73, 57, 201, 232, 208,
            219, 42, 85, 242, 146, 169, 167, 93, 196, 41, 170, 134, 245, 251, 132, 117, 101, 88, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197, 33, 10, 45, 228, 168, 212, 211, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180, 32, 123, 235,
            13, 65, 144, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        let root = vec![
            244, 66, 119, 245, 49, 148, 83, 125, 119, 234, 65, 205, 91, 124, 143, 211, 180, 8, 160,
            148, 47, 110, 220, 9, 163, 254, 180, 238, 62, 88, 141, 72,
        ];

        assert_eq!(generate(1, 1), proof);
        let mut mem = InMemoryBackend::new(&mut proof, 1);
        assert_eq!(mem.root(), Ok(*array_ref![root, 0, 32]));
    }
}
