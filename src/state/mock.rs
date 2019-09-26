use crate::account::Account;
use crate::address::Address;
use crate::error::Error;
use crate::hash::H256;
use crate::state::State;
use bigint::U256;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

pub struct MockState {
    accounts: BTreeMap<Address, Account>,
}

impl MockState {
    pub fn new(accounts: BTreeMap<Address, Account>) -> MockState {
        MockState { accounts }
    }
}

impl State for MockState {
    fn root(&mut self) -> Result<H256, Error> {
        let mut s = DefaultHasher::new();
        self.accounts.hash(&mut s);
        let hash = U256::from(s.finish());
        Ok(hash.into())
    }

    fn value(&self, address: Address) -> Result<u64, Error> {
        let value = self
            .accounts
            .get(&address)
            .ok_or(Error::StateIncomplete(address.into()))?
            .value;

        Ok(value)
    }

    fn nonce(&self, address: Address) -> Result<u64, Error> {
        let nonce = self
            .accounts
            .get(&address)
            .ok_or(Error::StateIncomplete(address.into()))?
            .nonce;

        Ok(nonce)
    }

    fn add_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        let mut account = self
            .accounts
            .get(&address)
            .ok_or(Error::StateIncomplete(address.into()))?
            .clone();

        account.value += amount;
        self.accounts.insert(address, account.clone());

        Ok(account.value)
    }

    fn sub_value(&mut self, address: Address, amount: u64) -> Result<u64, Error> {
        let mut account = self
            .accounts
            .get(&address)
            .ok_or(Error::StateIncomplete(address.into()))?
            .clone();

        account.value -= amount;
        self.accounts.insert(address, account.clone());

        Ok(account.value)
    }

    fn inc_nonce(&mut self, address: Address) -> Result<u64, Error> {
        let mut account = self
            .accounts
            .get(&address)
            .ok_or(Error::StateIncomplete(address.into()))?
            .clone();

        account.nonce += 1;
        self.accounts.insert(address, account.clone());

        Ok(account.nonce)
    }
}
