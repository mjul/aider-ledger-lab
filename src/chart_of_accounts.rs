use crate::account::AccountInfo;

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
                children.push(child);
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
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_empty_chart_of_accounts() {
        let coa = ChartOfAccounts::Group {
            headline: None,
            footer: None,
            children: Vec::new(),
        };
    }
}
