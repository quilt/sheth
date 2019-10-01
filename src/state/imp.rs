use crate::account::{calc_nonce_index, calc_value_index};
use crate::address::Address;
use crate::error::Error;
use crate::hash::{hash, H256};
use crate::state::State;
use crate::u264::U264;
use arrayref::{array_mut_ref, array_ref};
use imp::Imp;

// #[cfg(feature = "std")]
// pub fn as_bytes(&self) -> Vec<u8> {
//     let mut ret: Vec<u8> = vec![];
//     ret.extend(
//         (((self.offsets.len() + 8) / 8) as u64)
//             .to_le_bytes()
//             .to_vec(),
//     );
//     ret.extend(self.offsets);
//     ret.extend(&*self.db);
//     ret
// }

impl<'a> State for Imp<'a, U264> {
    fn root(&mut self) -> Result<[u8; 32], Error> {
        let offsets = unsafe {
            core::slice::from_raw_parts(self.offsets.as_ptr() as *const u64, self.offsets.len() / 8)
        };

        fn helper(proof: &[u8], offsets: &[u64], offset: u64) -> Result<H256, Error> {
            if offsets.len() == 0 {
                return Ok(*array_ref![proof, (offset * 32) as usize, 32]);
            }

            let mut left = *array_ref![proof, (offset * 32) as usize, 32];
            let mut right = *array_ref![proof, ((offset + 1) * 32) as usize, 32];

            if offsets[0] != 1 {
                left = helper(proof, &offsets[1..offsets[0] as usize], offset)?;
            }

            if offsets.len() != 1 {
                right = helper(
                    proof,
                    &offsets[offsets[0] as usize..],
                    offsets[0] as u64 + offset,
                )?;
            }

            // Copy chunks into hashing buffer
            let mut buf = [0u8; 64];
            buf[0..32].copy_from_slice(&left);
            buf[32..64].copy_from_slice(&right);

            // Hash chunks
            hash(array_mut_ref![buf, 0, 64]);

            Ok(*array_ref![buf, 0, 32])
        }

        helper(self.db, offsets, 0)
    }

    fn value(&self, address: Address) -> Result<u64, Error> {
        let index = calc_value_index(address, self.height);
        let chunk = self.get(index);
        Ok(u64::from_le_bytes(*array_ref![&chunk, 0, 8]))
    }

    fn nonce(&self, address: Address) -> Result<u64, Error> {
        let index = calc_nonce_index(address, self.height);
        let chunk = self.get(index);
        Ok(u64::from_le_bytes(*array_ref![&chunk, 0, 8]))
    }

    fn add_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        let index = calc_value_index(address, self.height);
        let chunk = self.get(index);

        let value = u64::from_le_bytes(*array_ref![&chunk, 0, 8]);

        let (value, overflow) = value.overflowing_add(amount);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&value.to_le_bytes());
        self.update(index, buf);

        Ok(value)
    }

    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        let index = calc_value_index(address, self.height);
        let chunk = self.get(index);

        let value = u64::from_le_bytes(*array_ref![chunk, 0, 8]);

        let (value, overflow) = value.overflowing_sub(amount);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&value.to_le_bytes());
        self.update(index, buf);

        Ok(value)
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        let index = calc_nonce_index(address, self.height);
        let chunk = self.get(index);

        let nonce = u64::from_le_bytes(*array_ref![chunk, 0, 8]);

        let (nonce, overflow) = nonce.overflowing_add(1);
        if overflow {
            return Err(Error::Overflow);
        }

        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&nonce.to_le_bytes());
        self.update(index, buf);

        Ok(nonce)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn zh(depth: usize) -> H256 {
        let mut buf = [0u8; 64];
        crate::hash::zh(depth, &mut buf);
        *array_ref![buf, 0, 32]
    }

    fn h256(n: u8) -> H256 {
        let mut ret = [0u8; 32];
        ret[0] = n;
        ret
    }

    fn get_proof() -> Vec<u8> {
        // indexes = [16, 17, 9, 10, 11, 3]
        let offsets: Vec<u8> = vec![6, 5, 3, 2, 1, 1].iter().fold(vec![], |mut acc, x| {
            let x = *x as u64;
            acc.extend(&x.to_le_bytes());
            acc
        });

        let proof: Vec<u8> = vec![h256(0), h256(0), h256(1), h256(1), zh(0), zh(0)]
            .iter()
            .fold(vec![], |mut acc, x| {
                acc.extend(x);
                acc
            });

        let mut ret = offsets;
        ret.extend(proof);

        ret
    }

    #[test]
    fn add_value() {
        let mut proof = get_proof();
        let mut mem = Imp::new(&mut proof, 1);

        assert_eq!(mem.add_value(0.into(), 1), Ok(2));
        assert_eq!(mem.get((10 << 1).into()), h256(2));
    }

    #[test]
    fn sub_value() {
        let mut proof = get_proof();
        let mut mem = Imp::new(&mut proof, 1);

        assert_eq!(mem.sub_value(0.into(), 1), Ok(0));
        assert_eq!(mem.get((10 << 1).into()), h256(0));
    }

    #[test]
    fn inc_nonce() {
        let mut proof = get_proof();
        let mut mem = Imp::new(&mut proof, 1);

        assert_eq!(mem.inc_nonce(0.into()), Ok(2));
        assert_eq!(mem.get((9 << 1).into()), h256(2));
    }
}
