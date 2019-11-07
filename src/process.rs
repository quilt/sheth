use crate::error::Error;
use crate::state::State;
use crate::transaction::{Transaction, Transfer};

pub fn process_transactions<'a, T: State>(
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

fn transfer<'a, T: State>(db: &mut T, tx: &Transfer) -> Result<(), Error> {
    db.sub_value(tx.color, tx.from, tx.amount)?;
    db.add_value(tx.color, tx.to, tx.amount)?;

    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::account::Account;
    use crate::address::Address;
    use crate::bls::PublicKey;
    use crate::state::MockState;
    use crate::state::TokenColor;
    use crate::transaction::{Transaction, Transfer};
    use bigint::U256;
    use std::collections::BTreeMap;

    fn build_state() -> MockState {
        let mut accounts: BTreeMap<Address, Account> = BTreeMap::new();
        accounts.insert(
            0.into(),
            Account {
                pubkey: PublicKey::zero(),
                nonce: 0,
                red_value: 5,
                blue_value: 0,
                green_value: 0,
            },
        );
        accounts.insert(
            1.into(),
            Account {
                pubkey: PublicKey::zero(),
                nonce: 0,
                red_value: 2,
                blue_value: 0,
                green_value: 0,
            },
        );

        MockState::new(accounts)
    }

    #[test]
    fn two_accounts() {
        let transactions = vec![
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 0,
                amount: 2,
                color: TokenColor::Red,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(1).into(),
                from: U256::from(0).into(),
                nonce: 1,
                amount: 3,
                color: TokenColor::Red,
                signature: [0; 96],
            }),
            Transaction::Transfer(Transfer {
                to: U256::from(0).into(),
                from: U256::from(1).into(),
                nonce: 0,
                amount: 5,
                color: TokenColor::Red,
                signature: [0; 96],
            }),
        ];

        let mut mem = build_state();

        let pre_root = mem.root().unwrap();
        assert_eq!(process_transactions(&mut mem, &transactions), Ok(()));
        let post_root = mem.root().unwrap();

        assert_eq!(
            "0000000000000000000000000000000000000000000000009dc492172af59378",
            hex::encode(pre_root)
        );

        assert_eq!(
            "000000000000000000000000000000000000000000000000c8709c5e456c7ef6",
            hex::encode(post_root)
        );
    }
}
