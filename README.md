# aider-ledger-lab
This lab is a test-drive of aider.chat for LLM-assisted coding.

## Overview
Welcome to the aider-ledger-lab repository! This project is a simple model of a Rust library for a general ledger, designed for use in accounting. The ledger is structured into accounts, which are organized into a tree called a chart of accounts.

## Accounting Terms
- **General Ledger**: A comprehensive record of all financial transactions of a business.
- **Chart of Accounts**: A hierarchical list of all the accounts used in the general ledger.
- **Account**: A specific category of transactions in a company's accounting records.
- **Debit**: An increase to an asset account or a decrease to a liability account in a company's books.
- **Credit**: A decrease to an asset account or an increase to a liability account in a company's books.
- **Trial Balance**: A summary of a company's accounts at a specific point in time. It lists all accounts with their debit and credit balances.

## Features
- The chart of accounts describes the accounts used for the general ledger and is a tree with individual accounts and logical accounts for sums and grouping.
- Logical accounts (non-leaf nodes) can have an optional group headline and group sum.
- Account sums are always separated for debit and credit columns.
- You can get a balance for the whole period of the account since creation or a trial balance on a specific time.
- All account entries have a posting date and an effective date.
- Monetary values are represented as decimals. Each account has a single associated currency and accepts only entries in this currency.
