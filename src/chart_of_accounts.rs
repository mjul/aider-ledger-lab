use std::collections::HashMap;
use crate::account::{Account, AccountInfo};

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
        children: Vec<ChartOfAccounts>,
    },
    Account {
        info: AccountInfo,
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
            ChartOfAccounts::Account { info, .. } => &info.name,
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
            ChartOfAccounts::Account { info, .. } => {
                if name == &info.name && path.len() == 1 {
                    Some(&Account::new(info.clone()))
                } else {
                    None
                }
            }
        }
    }
}
