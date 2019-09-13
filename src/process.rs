use crate::error::Error;
use crate::state::Backend;
use crate::transaction::{Transaction, Transfer};

pub fn process_transactions<'a, T: Backend<'a>>(
    db: &'a mut T,
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

fn transfer<'a, T: Backend<'a>>(db: &mut T, tx: &Transfer) -> Result<(), Error> {
    db.sub_value(tx.from, tx.amount)?;
    db.add_value(tx.to, tx.amount)?;

    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::state::InMemoryBackend;
    use crate::transaction::{Transaction, Transfer};
    use bigint::U256;

    static PROOF: [u8; 240] = [
        6, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 218, 109, 128, 123, 247, 149, 16, 97,
        70, 229, 130, 39, 117, 217, 20, 176, 39, 122, 101, 36, 15, 101, 14, 212, 200, 167, 202,
        119, 130, 78, 90, 223, 120, 72, 181, 215, 17, 188, 152, 131, 153, 99, 23, 163, 249, 201, 2,
        105, 213, 103, 113, 0, 93, 84, 10, 25, 24, 73, 57, 201, 232, 208, 219, 42, 85, 242, 146,
        169, 167, 93, 196, 41, 170, 134, 245, 251, 132, 117, 101, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 197, 33, 10, 45, 228, 168, 212, 211, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180, 32, 123, 235, 13, 65, 144, 114, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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

        // let mut mem = InMemoryBackend::new(2);

        // assert_eq!(mem.load(&PROOF), Ok(()));
        // assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

        // let roots = mem.roots().unwrap();

        // assert_eq!(
        //     "cd324543aae22d6e8ae6050276279d78d5f1c0da78551f938f8afefd6a3dd0d7",
        //     hex::encode(roots.0)
        // );

        // assert_eq!(
        //     "0c1cde372ab576268597d34a22e202ade92eb3e086f9ef0087a981ea4e531dbd",
        //     hex::encode(roots.1)
        // );
    }
}
