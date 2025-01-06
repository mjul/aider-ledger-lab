use std::collections::HashMap;
use crate::account::Account;

pub struct ChartOfAccounts {
    name: String,
    accounts: HashMap<String, Account>,
    children: HashMap<String, ChartOfAccounts>,
}

impl ChartOfAccounts {
    pub fn new(name: String) -> Self {
        ChartOfAccounts {
            name,
            accounts: HashMap::new(),
            children: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.name.clone(), account);
    }

    pub fn add_child(&mut self, child: ChartOfAccounts) {
        self.children.insert(child.name.clone(), child);
    }

    pub fn get_account(&self, path: &[String]) -> Option<&Account> {
        if path.is_empty() {
            return None;
        }

        let name = &path[0];
        if path.len() == 1 {
            self.accounts.get(name)
        } else {
            self.children.get(name).and_then(|child| child.get_account(&path[1..]))
        }
    }
}
