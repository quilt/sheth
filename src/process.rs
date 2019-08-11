use crate::error::Error;
use crate::state::Db;
use crate::transaction::Transaction;

fn process_transactions<'a>(db: &'a mut Db, transactions: &'a [Transaction]) -> &'a [u8; 32] {
    for tx in transactions {
        if let Err(Error) = tx.verify(db) {
            continue;
        }

        db.inc_nonce(t.from);

        match tx {
            Transaction::Transfer(t) => {}
            Transaction::Deposit(d) => unimplemented!(),
            Transaction::Withdrawal(w) => unimplemented!(),
        }
    }

    unimplemented!();
}

fn transfer(db: &mut Db, tx: Transaction) -> Result<(), Error> {
    db.sub_value(t.from, t.amount)?;
    db.add_value(t.to, t.amount)?;
}
