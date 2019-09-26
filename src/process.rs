use crate::error::Error;
use crate::state::State;
use crate::transaction::{Transaction, Transfer};

pub fn process_transactions<'a, T: State<'a>>(
    db: &mut T,
    transactions: &[Transaction],
) -> Result<(), Error> {
    for tx in transactions {
        if let Err(_) = tx.verify(db) {
            continue;
        }

        db.inc_nonce(tx.from())?;

        match tx {
            Transaction::Transfer(t) => transfer(db, t)?,
            Transaction::Deposit(_) => unimplemented!(),
            Transaction::Withdrawal(_) => unimplemented!(),
        }
    }

    Ok(())
}

fn transfer<'a, T: State<'a>>(db: &mut T, tx: &Transfer) -> Result<(), Error> {
    db.sub_value(tx.from, tx.amount)?;
    db.add_value(tx.to, tx.amount)?;

    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::state::Multiproof;
    use crate::transaction::{Transaction, Transfer};
    use bigint::U256;

    static PROOF: [u8; 400] = [
        10, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 127, 94, 166, 196, 32, 181, 47,
        184, 196, 119, 113, 87, 177, 5, 167, 18, 67, 162, 37, 35, 153, 161, 46, 69, 228, 55, 93,
        138, 243, 89, 39, 163, 18, 138, 182, 218, 55, 72, 52, 48, 99, 58, 172, 127, 223, 126, 13,
        101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 72, 181, 215, 17,
        188, 152, 131, 153, 99, 23, 163, 249, 201, 2, 105, 213, 103, 113, 0, 93, 84, 10, 25, 24,
        73, 57, 201, 232, 208, 219, 42, 85, 242, 146, 169, 167, 93, 196, 41, 170, 134, 245, 251,
        132, 117, 101, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn two_accounts() {
        let transactions = vec![
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 0,
                amount: 2,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 1,
                amount: 3,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(0).into(),
                from: U256::from(1).into(),
                nonce: 0,
                amount: 5,
                signature: [0; 96],
            }),
        ];

        let mut proof = PROOF;
        let mut mem = Multiproof::new(&mut proof, 1);

        let pre_root = mem.root().unwrap();
        assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));
        let post_root = mem.root().unwrap();

        assert_eq!(
            "e47bd21d034fc39428c84510d9955eb0aa8f6d9e71cb4722af90821eba92a284",
            hex::encode(pre_root)
        );

        assert_eq!(
            "5e1cc14cb4917bf2fe2e3b7aee1a95396cf741a164f98eaa8f06d7eee5926e4f",
            hex::encode(post_root)
        );
    }
}
