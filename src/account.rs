use chrono::NaiveDate;
use uuid::Uuid;

pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub currency: String,
    pub account_number: u32,
    pub entries: Vec<Entry>,
}

impl Account {
    pub fn new(name: String, currency: String, account_number: u32) -> Self {
        Account {
            id: AccountId::new(),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}
