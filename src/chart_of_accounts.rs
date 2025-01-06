use std::collections::HashMap;
use crate::account::Account;

pub struct ChartOfAccounts {
    accounts: HashMap<String, Account>,
    children: HashMap<String, ChartOfAccounts>,
}

impl ChartOfAccounts {
    pub fn new() -> Self {
        ChartOfAccounts {
            accounts: HashMap::new(),
            children: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.name.clone(), account);
    }

    pub fn add_child(&mut self, name: String, child: ChartOfAccounts) {
        self.children.insert(name, child);
    }

    pub fn get_account(&self, name: &str) -> Option<&Account> {
        self.accounts.get(name).or_else(|| {
            self.children.values().find_map(|child| child.get_account(name))
        })
    }
}
