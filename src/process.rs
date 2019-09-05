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
