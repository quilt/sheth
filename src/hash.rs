use sha2::{Digest, Sha256};

pub type H256 = [u8; 32];

pub fn hash(buf: &mut [u8; 64]) {
    let mut tmp = [0u8; 32];
    tmp.copy_from_slice(Sha256::digest(buf).as_ref());
    buf[0..32].copy_from_slice(&tmp);
}

pub fn zh(mut depth: usize, buf: &mut [u8; 64]) {
    // Hash of an account with a balance of zero.
    let mut tmp = [
        218, 109, 128, 123, 247, 149, 16, 97, 70, 229, 130, 39, 117, 217, 20, 176, 39, 122, 101,
        36, 15, 101, 14, 212, 200, 167, 202, 119, 130, 78, 90, 223,
    ];

    buf[0..32].copy_from_slice(&tmp);
    buf[32..64].copy_from_slice(&tmp);

    while depth > 0 {
        tmp.copy_from_slice(&buf[0..32]);
        buf[32..64].copy_from_slice(&tmp);
        hash(buf);
        depth -= 1;
    }
}
