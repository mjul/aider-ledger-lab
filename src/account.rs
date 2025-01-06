use chrono::NaiveDate;

pub struct Account {
    pub name: String,
    pub currency: String,
    pub account_number: u32,
    pub entries: Vec<Entry>,
}

impl Account {
    pub fn new(name: String, currency: String, account_number: u32) -> Self {
        Account {
            name,
            currency,
            account_number,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_balance(&self, date: NaiveDate) -> f64 {
        self.entries
            .iter()
            .filter(|entry| entry.posting_date <= date)
            .map(|entry| entry.amount)
            .sum()
    }
}

pub struct Entry {
    pub posting_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub amount: f64,
}
