/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

pub(super) const TXT: &str = r#"#
#Tackler-NG configuration
#
# For full Documentation, see:
# https://tackler.e257.fi/docs/configuration/tackler-toml/
#
[kernel]
###
### Use strict account data validation
### If true, all transactions must use predefined accounts, commodities and tags
### Valid values are <true|false>
strict = false
audit = { mode = false, hash = "SHA-256" }
timestamp = { default-time = 00:00:00, timezone = { name = "UTC" } }

[kernel.input]
storage = "fs"
fs  = { path = "..",      dir = "txns", suffix = "txn" }
git = { repo = "../.git", dir = "txns", suffix = "txn", ref = "main" }

###
### Commodity Price Functionality
###    This is an optional section
[price]
db-path = "../txns/price.db"
###
### Possible values:
###     "none", "last-price", "given-time", "txn-time"
lookup-type = "none"

[transaction]
accounts    = { path = "accounts.toml" }
commodities = { path = "commodities.toml" }
tags        = { path = "tags.toml" }


[report]
report-timezone = "UTC"
scale = { min = 2, max = 2 }
###
### This is a list of accounts (full match regex) to be included in the reports
###
accounts = [ "Expenses(:.*)?", "Assets(:.*)?" ]
###
### Possible values are:
###     "balance", "balance-group", "register"
targets = [ "balance", "register" ]
###
### Reporting commodity
###    This is mandatory setting by configuration or CLI,
###    if Commodity Price functionality is enabled in the reports
###    CLI: --report.commodity CAD
commodity = "CAD"

balance       = { title = "Balance Report", type = "flat" }
balance-group = { title = "Balance Group Report", type = "flat", group-by = "month" }
register      = { title = "Register Report", accounts = [ "Welcome(:.*)?", ]}


[export]
targets = [ ]
equity = { accounts = [ "Assets(:.*)?", ], equity-account = "Equity:Balance" }
"#;
