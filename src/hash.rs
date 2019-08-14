use sha2::{Digest, Sha256};

pub fn hash(buf: &mut [u8; 64]) {
    let mut tmp = [0u8; 32];
    tmp.copy_from_slice(Sha256::digest(buf).as_ref());
    buf[0..32].copy_from_slice(&tmp);
}

pub fn zh(mut depth: usize, buf: &mut [u8; 64]) {
    let mut tmp = [0u8; 32];
    buf[0..32].copy_from_slice(&tmp);

    while depth > 0 {
        tmp.copy_from_slice(&buf[0..32]);
        buf[32..64].copy_from_slice(&tmp);
        hash(buf);
        depth -= 1;
    }
}
