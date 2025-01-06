# aider-ledger-lab

Welcome to the aider-ledger-lab repository! This project is a simple model of a Rust library for a general ledger, designed for use in accounting. The ledger is structured into accounts, which are organized into a tree called a chart of accounts.

The chart of accounts includes individual accounts and logical accounts. Logical accounts (non-leaf nodes) have an optional group headline and group sum in the headline or as a separate line in the balance below the group. This allows logical accounts to have an idea of summing the child accounts.

The account sums are always separated for debit and credit columns. You can get a balance for the whole period of the account since creation or a trial balance on a specific time. All account entries have a posting date and an effective date. The library uses the chrono library for date handling.

Monetary values are represented as decimals. In the accounts, every account has a single associated currency and accepts only entries in this currency.
