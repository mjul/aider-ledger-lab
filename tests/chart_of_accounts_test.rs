#[cfg(test)]
mod tests {
    use super::*;

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
                number: "1010".to_string(),
                name: "Salg af varer og ydelser".to_string(),
            },
            account_type: AccountType::Revenue,
        });

        // Add the rest of the accounts to the nettoomsætning group...

        coa.add_child(nettoomsætning);

        // Add the rest of the groups and accounts to the coa...

        assert_eq!(coa.name(), "RESULTATOPGØRELSE");
        assert_eq!(coa.get_account(&["Nettoomsætning", "Salg af varer og ydelser"]).unwrap().info.number, "1010");
        // Add more assertions to verify the structure of the chart of accounts...
    }
}
