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

    #[test]
    fn test_chart_of_accounts_tree() {
        let mut coa = ChartOfAccounts::Group {
            headline: Some("RESULTATOPGØRELSE".to_string()),
            footer: None,
            children: Vec::new(),
        };

        let mut nettoomsætning = ChartOfAccounts::Group {
            headline: Some("Nettoomsætning".to_string()),
            footer: None,
            children: Vec::new(),
        };

        nettoomsætning.add_child(ChartOfAccounts::Account {
            info: AccountInfo {
                id: Default::default(),
                name: "Salg af varer og ydelser".to_string(),
                currency: rusty_money::iso::DKK.clone(),
                account_number: 0,
            },
            account_type: AccountType::Revenue,
        });
        coa.add_child(nettoomsætning);
    }
}
