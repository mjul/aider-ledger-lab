use std::collections::HashMap;
use crate::account::Account;

#[derive(Debug)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug)]
pub enum ChartOfAccounts {
    Group {
        headline: Option<String>,
        footer: Option<String>,
        children: HashMap<String, ChartOfAccounts>,
    },
    Account {
        name: String,
        account_type: AccountType,
    },
}

impl ChartOfAccounts {
    pub fn add_child(&mut self, child: ChartOfAccounts) {
        match self {
            ChartOfAccounts::Group { children, .. } => {
                children.insert(child.name().to_string(), child);
            }
            _ => panic!("Cannot add child to a non-group account"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChartOfAccounts::Group { headline, .. } => headline.as_ref().map_or("", |s| s),
            ChartOfAccounts::Account { name, .. } => name,
        }
    }

    pub fn get_account(&self, path: &[String]) -> Option<&Account> {
        if path.is_empty() {
            return None;
        }

        let name = &path[0];
        match self {
            ChartOfAccounts::Group { children, .. } => {
                if path.len() == 1 {
                    None
                } else {
                    children.get(name).and_then(|child| child.get_account(&path[1..]))
                }
            }
            ChartOfAccounts::Account { name: account_name, .. } => {
                if name == account_name && path.len() == 1 {
                    Some(&Account::new(account_name.to_string(), Currency {}, 0))
                } else {
                    None
                }
            }
        }
    }
}
