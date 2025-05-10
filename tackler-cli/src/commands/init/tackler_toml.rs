/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

pub(super) const TXT: &str = r#"#
#Tackler-NG configuration file
#
# For full Documentation, see:
# https://tackler.e257.fi/docs/configuration/tackler-toml/
#
[kernel]
### Use strict account data validation
###   If true, all transactions must use predefined accounts, commodities and tags
###   Valid values are <true|false>
strict = false
audit = { mode = false, hash = "SHA-256" }
timestamp = { default-time = 00:00:00, timezone = { name = "UTC" } }

[kernel.input]
storage = "fs"
fs  = { path = "..",      dir = "txns", ext = "txn" }
git = { repo = "../.git", dir = "txns", ext = "txn", ref = "main" }

### Commodity Price Functionality
###    This is an optional section
[price]
### Price Database path
###   Path to pricedb, if it's relative, it's relative to location of this file
db-path = "../txns/price.db"
### Price lookup type
###   Possible values: "none", "last-price", "given-time", "txn-time"
lookup-type = "none"

[transaction]
accounts    = { path = "accounts.toml" }
commodities = { path = "commodities.toml" }
tags        = { path = "tags.toml" }

[report]
report-timezone = "UTC"
scale = { min = 2, max = 2 }

### Report output formats
###
###   This is a list of output formats. Actual output 
###   is activated with CLI output options (`--output.*`). 
###
###   Valid options are one or both of: "txt", "json"
formats = [ "txt" ]

### Report accounts
###   This is a list of accounts (full match regex)
###   to be included in the reports
###
###   To select all accounts, use an empty array
###      accounts = [ ]
accounts = [ "Assets(:.*)?", "Expenses(:.*)?" ]

### Report targets
###    Possible values are: "balance", "balance-group", "register"
targets = [ "balance", "register" ]

### Reporting commodity
###   If Commodity Price functionality is enabled in the reports then,
###   in that case this is mandatory setting (by configuration or by CLI).
###   CLI: --report.commodity CAD
commodity = "CAD"

###
### Balance and Balance Group Reports
###
###   There are two different kind of Balance family reports:  flat and tree
###    - `tree` reports balance for an account and for all its children accounts
###    - `flat` reports balance only for an account
###
###   You can select between these with `type` option on `balance` and `balance-group`
balance       = { title = "Balance Report", type = "tree" }
balance-group = { title = "Balance Group Report", type = "tree", group-by = "month" }
register      = { title = "Register Report", accounts = [ "Welcome(:.*)?", ]}

[export]
targets = [ ]
equity = { accounts = [ "Assets(:.*)?", ], equity-account = "Equity:Balance" }
"#;
