use chrono::NaiveDate;
use uuid::Uuid;
use rust_decimal::Decimal;
use rusty_money::{Money};
use rusty_money::iso::Currency;

pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub currency: Currency,
    pub account_number: u32,
    pub entries: Vec<Entry>,
}

impl Account {
    pub fn new(name: String, currency: Currency, account_number: u32) -> Self {
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

    pub fn get_balance(&self, date: NaiveDate) -> Money<'_, Currency> {
        let mut balance = Money::from_major(0, &self.currency);
        for entry in &self.entries {
            if entry.posting_date <= date {
                balance += entry.debit.clone();
                balance -= entry.credit.clone();
            }
        }
        balance
    }
}

pub struct Entry {
    pub posting_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub debit: Money<'static, Currency>,
    pub credit: Money<'static, Currency>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}
