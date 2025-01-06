use chrono::NaiveDate;
use uuid::Uuid;
use rust_decimal::Decimal;
use rusty_money::{Money};
use rusty_money::iso::Currency;

#[derive(Debug)]
pub struct AccountInfo {
    pub id: Uuid,
    pub name: String,
    pub currency: Currency,
    pub account_number: u32,
}

impl AccountInfo {
    pub fn new(name: String, currency: Currency, account_number: u32) -> Self {
        AccountInfo {
            id: Uuid::new_v4(),
            name,
            currency,
            account_number,
        }
    }
}

pub struct Account {
    pub info: AccountInfo,
    pub entries: Vec<Entry>,
}

impl Account {
    pub fn new(info: AccountInfo) -> Self {
        Account {
            info,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_balance(&self, date: NaiveDate) -> Money<'_, Currency> {
        let mut balance = Money::from_major(0, &self.info.currency);
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
