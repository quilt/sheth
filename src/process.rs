use crate::error::Error;
use crate::state::Backend;
use crate::transaction::Transaction;
use u256::U256;

fn process_transactions<T>(db: &mut T, transactions: &[Transaction]) -> U256
where
    T: Backend,
{
    for tx in transactions {
        if let Err(Error) = tx.verify(db) {
            continue;
        }

        db.inc_nonce(tx.from());

        match tx {
            Transaction::Transfer(t) => {}
            Transaction::Deposit(d) => unimplemented!(),
            Transaction::Withdrawal(w) => unimplemented!(),
        }
    }

    unimplemented!();
}

fn transfer<T: Backend>(db: &mut T, tx: Transaction) -> Result<(), Error> {
    // db.sub_value(tx.from, tx.amount)?;
    // db.add_value(tx.to, tx.amount)?;

    Ok(())
}
