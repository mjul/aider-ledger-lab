use chrono::NaiveDate;
use uuid::Uuid;
use decimal::Decimal;

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

    pub fn get_balance(&self, date: NaiveDate) -> MoneyAmount {
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
    pub debit: MoneyAmount,
    pub credit: MoneyAmount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoneyAmount(Decimal);

impl MoneyAmount {
    pub fn new(amount: Decimal) -> Self {
        MoneyAmount(amount)
    }
}

impl std::ops::Add for MoneyAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MoneyAmount(self.0 + other.0)
    }
}

impl std::ops::Sub for MoneyAmount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        MoneyAmount(self.0 - other.0)
    }
}
