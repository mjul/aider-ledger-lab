# aider-ledger-lab
This lab is a test-drive of aider.chat for LLM-assisted coding.

## Overview
Welcome to the aider-ledger-lab repository! This project is a simple model of a Rust library for a general ledger, designed for use in accounting. The ledger is structured into accounts, which are organized into a tree called a chart of accounts.

## Features
### Chart of Accounts
- The chart of accounts describes the accounts used for the general ledger.
- The chart of accounts is a tree with individual accounts and logical accounts for sums and grouping.

# Account
- Logical accounts (non-leaf nodes) have an optional group headline and group sum in the headline or as a separate line in the balance below the group.
- The account sums are always separated for debit and credit columns.
- You can get a balance for the whole period of the account since creation or a trial balance on a specific time.
- All account entries have a posting date and an effective date.
- Monetary values are represented as decimals. In the accounts, every account has a single associated currency and accepts only entries in this currency.
