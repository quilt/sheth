use crate::error::Error;
use crate::state::Backend;
use crate::transaction::{Transaction, Transfer};
use arrayref::array_ref;

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;

pub fn process_transactions<T: Backend>(db: &mut T, transactions: &[u8]) -> Result<(), Error> {
    let mut i = 0;
    while i < transactions.len() {
        // Hacky :-(
        let tx = Transaction::Transfer(unsafe {
            core::mem::transmute::<&[u8; 176], &Transfer>(array_ref![transactions, i, 176])
        });

        #[cfg(feature = "std")]
        {
            println!("");
            println!("i      : {}", i);
            println!("{}", tx);
        }

        if let Err(_) = tx.verify(db) {
            continue;
        }

        db.inc_nonce(tx.from())?;

        // match tx {
        //     Transaction::Transfer(t) => transfer(db, t)?,
        //     Transaction::Deposit(d) => unimplemented!(),
        //     Transaction::Withdrawal(w) => unimplemented!(),
        // }

        i += 176;
    }

    Ok(())
}

fn transfer<T: Backend>(db: &mut T, tx: &Transfer) -> Result<(), Error> {
    db.sub_value(tx.from(), tx.amount())?;
    db.add_value(tx.to(), tx.amount())?;

    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::account::Address;
    use crate::state::InMemoryBackend;
    use crate::transaction::{Transaction, Transfer};
    use bigint::U256;

    static PROOF: [u8; 719] = [
        11, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 195, 86,
        169, 110, 177, 121, 15, 58, 133, 98, 199, 13, 111, 144, 174, 84, 132, 236, 25, 85, 251,
        220, 228, 101, 75, 111, 42, 177, 146, 186, 203, 153,
    ];

    #[test]
    fn two_accounts() {
        let mut transactions: Vec<u8> = vec![];
        transactions.extend(
            Transfer::new(Address::from(1), Address::from(0), 0, 2, [0; 96])
                .as_bytes()
                .iter(),
        );
        transactions.extend(
            Transfer::new(
                Address::from(1).into(),
                Address::from(0).into(),
                1,
                3,
                [0; 96],
            )
            .as_bytes()
            .iter(),
        );
        transactions.extend(
            Transfer::new(
                Address::from(0).into(),
                Address::from(1).into(),
                0,
                5,
                [0; 96],
            )
            .as_bytes()
            .iter(),
        );

        let mut mem = InMemoryBackend::new(2);

        assert_eq!(mem.load(&PROOF), Ok(()));
        assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));

        let roots = mem.roots().unwrap();

        assert_eq!(
            "cd324543aae22d6e8ae6050276279d78d5f1c0da78551f938f8afefd6a3dd0d7",
            hex::encode(roots.0)
        );

        assert_eq!(
            "0c1cde372ab576268597d34a22e202ade92eb3e086f9ef0087a981ea4e531dbd",
            hex::encode(roots.1)
        );
    }
}
