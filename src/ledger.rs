use chrono::NaiveDate;
use std::collections::HashMap;
use crate::account::Account;

pub struct Ledger {
    accounts: HashMap<String, Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.name.clone(), account);
    }

    pub fn get_account(&self, name: &str) -> Option<&Account> {
        self.accounts.get(name)
    }

    pub fn get_balance(&self, name: &str, date: NaiveDate) -> Option<f64> {
        self.accounts.get(name).map(|account| account.get_balance(date))
    }
}
