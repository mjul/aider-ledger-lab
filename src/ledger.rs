use chrono::NaiveDate;
use crate::account::Account;
use crate::chart_of_accounts::ChartOfAccounts;

pub struct Ledger {
    root: ChartOfAccounts,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            root: ChartOfAccounts::new(),
        }
    }

    pub fn add_account(&mut self, path: Vec<String>, account: Account) {
        let mut node = &mut self.root;
        for name in path {
            node = node.children.entry(name).or_insert(ChartOfAccounts::new());
        }
        node.add_account(account);
    }

    pub fn get_account(&self, path: Vec<String>) -> Option<&Account> {
        let mut node = &self.root;
        for name in path {
            node = node.children.get(&name)?;
        }
        node.get_account(&path.last().unwrap())
    }

    pub fn get_balance(&self, path: Vec<String>, date: NaiveDate) -> Option<f64> {
        self.get_account(path).map(|account| account.get_balance(date))
    }
}
