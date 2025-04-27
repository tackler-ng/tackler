# Tackler-NG: Tackler CLI Application

[![Build Status](https://github.com/tackler-ng/tackler/actions/workflows/ci.yml/badge.svg)](https://github.com/tackler-ng/tackler/actions)
[![Github Releases](https://img.shields.io/github/v/release/tackler-ng/tackler?include_prereleases&color=%230868da)](https://github.com/tackler-ng/tackler/releases)
[![crates.io](https://tackler.e257.fi/img/badge-crates.svg)](https://crates.io/crates/tackler)
[![Tackler Docs](https://img.shields.io/badge/tackler-documentation-%23ffcb00)](https://tackler.e257.fi/docs)
[![GitHub Discussions](https://img.shields.io/github/discussions/tackler-ng/tackler)](https://github.com/tackler-ng/tackler/discussions)
[![Chat on Matrix](https://tackler.e257.fi/img/badge-matrix.svg)](https://matrix.to/#/#tackler:matrix.org)


[Tackler](https://tackler.e257.fi/) is fast (1), reliable bookkeeping tool
with native GIT SCM  support for plain text accounting, written in Rust. 

````bash
cargo install --locked tackler
tackler new demo
tackler --config demo/conf/tackler.toml
````
This will produce balance and register reports for the demo journal.

````text
Balance Report
--------------
                 0.00    17.50  Expenses
                 0.00    12.00  Expenses:Food
                12.00    12.00  Expenses:Food:Fast-Food
                 0.00     5.50  Expenses:Sweets
                 2.50     2.50  Expenses:Sweets:Candy
                 3.00     3.00  Expenses:Sweets:Ice·Cream
=====================
                17.50

Register Report
---------------
...
````

1) Tackler has excellent performance, it can process 700_000 transactions per second on modern laptop. 
See [Tackler Performance](https://tackler.e257.fi/docs/performance/) for details.


## Project Status

Tackler-NG is in [feature](https://tackler.e257.fi/features/) parity with and beyond of the old Scala code base.
It's basis of all Tackler development.

**NOTE: Tackler-NG is tested with 423 [tracked test vectors](https://github.com/tackler-ng/tackler-t3db)**

All Tackler CLI functionality is supported, including 
[Tackler Journal Format](https://tackler.e257.fi/docs/journal/format/), 
transaction storages ([FS backend](https://tackler.e257.fi/docs/usage/#storage-selector), 
[Gitoxide](https://github.com/GitoxideLabs/gitoxide/) based [Git backend](https://tackler.e257.fi/docs/journal/git-storage/)), 
all reports 
([Balance](https://tackler.e257.fi/docs/report-balance/), 
[Balance Group](https://tackler.e257.fi/docs/report-balance-group/), 
[Register](https://tackler.e257.fi/docs/report-register/))
and all exports 
([Equity](https://tackler.e257.fi/docs/export-equity/), 
[Identity](https://tackler.e257.fi/docs/export-equity/)).

Other notable features are:

* [Commodities](https://tackler.e257.fi/docs/commodities/), [Currencies and Shares](https://tackler.e257.fi/docs/currencies/)
* [Market Value of Commodities and Shares](https://tackler.e257.fi/docs/price/), including different valuation (Mark-to-Market) methods:
  * [Current Market Value](https://tackler.e257.fi/docs/price/current-market-value/)
  * [Historic Market Value](https://tackler.e257.fi/docs/price/historic-market-value/)
  * [Variable Market Value](https://tackler.e257.fi/docs/price/variable-market-value/)
* [Transaction Filters](https://tackler.e257.fi/docs/txn-filters/) for powerfull selectors of accounting data
* Real transaction [timestamps up to nanosecond](https://tackler.e257.fi/docs/journal/format/#timestamps) resolution and with timezone information
  * Plain dates are also supported
  * Also Civil date times without zone information
* Accounting based on [Geo Location](https://tackler.e257.fi/docs/gis/txn-geo-location/) and [Transaction GIS Filters](https://tackler.e257.fi/docs/gis/txn-geo-filters/)
* Optional [JSON Output](https://tackler.e257.fi/docs/output-formats/)

See `tackler --help` and [Tackler Configuration](https://github.com/tackler-ng/tackler/blob/main/examples/tackler.toml) how to use tackler-ng.


## Installation

You can install tackler binary directly by cargo:

````bash
# Latest released version
cargo install --locked tackler

# Latest development version
cargo install --locked --git https://github.com/tackler-ng/tackler tackler
````

Or build it from the source.

### Build the Source Code

The `main` branch should [build and pass](https://github.com/tackler-ng/tackler/actions/workflows/ci.yml) 
all tests all the time.

You have to clone the tackler source code with git submodules, 
as [test vectors](https://github.com/tackler-ng/tackler-tests) are located in a separate repository.

````bash
git clone --recurse-submodules https://github.com/tackler-ng/tackler
````

Then build the tackler binary - if you have [`just`](https://github.com/casey/just) installed,
building tackler is just:

````bash
cd tackler
just release-build
````

Or with plain cargo command:

````bash
cd tackler
cargo build --release --locked --bin tackler
````

Tackler binary will be located at `target/release/tackler`

## Examples

Check out Tackler's [repository for full list of examples](https://github.com/tackler-ng/tackler/blob/main/examples/readme.adoc).

These examples need tackler source code and test vectors,
so make sure you have cloned it out with submodules.
If not, update the test suite submodule:

```bash
git submodule init
git submodule update
```


### Simple example


This setup doesn't have any checks enabled and it uses plain filesystem as transaction storage.

#### Journal

````
2024-03-20 'Lucky Day!
   Assets:Bank:Acme_Inc  100
   Income:Lottery

2024-06-20 'Sweet'n Sour Candies
   Expenses:Sweets:Candy  2.50
   Assets:Cash

2024-09-22 'Hot dogs
   Expenses:Food:FastFood  12
   Assets:Visa:4012_8888_8888_1881

2024-12-21 'Strawberry ice cream
   Expenses:Sweets:Ice·Cream  3
   Assets:Cash
````


#### Command

````bash
target/release/tackler --config examples/simple.toml
````

#### Output

````
Balance Report
--------------
                 0.00    17.50  Expenses
                 0.00    12.00  Expenses:Food
                12.00    12.00  Expenses:Food:FastFood
                 0.00     5.50  Expenses:Sweets
                 2.50     2.50  Expenses:Sweets:Candy
                 3.00     3.00  Expenses:Sweets:Ice·Cream
=====================
                17.50
````

## Let's play for real

Following examples use bare git repository as transaction storage, 
and also strict and audit mode is activated by configuration.

The triplet of git commit id, Txn Set Checksum and 
Account Selector Checksum provides auditable (cryptographic)
proof of transactions used by reports.

### Use Git repository as Transaction storage

#### Reports with Txn Checksum

````bash
target/release/tackler \
    --config examples/audit.toml \
````

#### Output

````
Git Storage
      reference : set-1e1
      directory : txns
      extension : txn
         commit : ed6e4b10de2daea8d143569c473d14a9b09c3270
         author : tackler <accounting@example.com>
           date : 2016-11-25 09:36:00 +0000
        subject : set-1e1: 2016-11-25

Txn Set Checksum
        SHA-256 : 4a0eb2f8836447a025030a87136c047b4a737031162f593cb00f390c6ba113a3
       Set size : 10

**********************************************************************************
Account Selector Checksum
        SHA-256 : df4714ff3f5bd031a8256df95863a8085f740b436f1b47febb45c9eb9aaa9e9e


Balance Report
--------------
                -6.00   a:ay2016:am02
               -14.00   a:ay2016:am03
               -19.00   a:ay2016:am04
               -26.00   a:ay2016:am05
                -1.00   a:ay2016:am07
                -7.00   a:ay2016:am08
               -13.00   a:ay2016:am09
               -19.00   a:ay2016:am10
               -25.00   a:ay2016:am11
               -31.00   a:ay2016:am12
=====================
              -161.00
##################################################################################
````

#### Report with 100_000 Transactions

There is git ref 'set-1e5' inside the example audit -repository.

````bash
target/release/tackler \
    --config examples/audit.toml \
    --input.git.ref set-1e5
````

#### Output

````
Git Storage
      reference : set-1e5
      directory : txns
      extension : txn
         commit : 4648a2994b41ed341b544a148b3060fd2d267d79
         author : tackler <accounting@example.com>
           date : 2016-12-31 17:41:24 +0000
        subject : set-1e5: 2016-12-31

Txn Set Checksum
        SHA-256 : 2f4bc22df78502182aa27037d8d0f72462adb018be3e768399e0b803fa75baa7
       Set size : 100000

**********************************************************************************
Account Selector Checksum
        SHA-256 : df4714ff3f5bd031a8256df95863a8085f740b436f1b47febb45c9eb9aaa9e9e

Balance Report
--------------
               -135600.00   a:ay2016:am01
               -118950.00   a:ay2016:am02
               -135631.00   a:ay2016:am03
               -127137.00   a:ay2016:am04
               -135616.00   a:ay2016:am05
               -127154.00   a:ay2016:am06
               -135600.00   a:ay2016:am07
               -135603.00   a:ay2016:am08
               -127140.00   a:ay2016:am09
               -135619.00   a:ay2016:am10
               -127126.00   a:ay2016:am11
               -133433.00   a:ay2016:am12
=========================
              -1574609.01
##################################################################################
````

### Transaction Filters

#### Filter definition

````bash
target/release/tackler \
    --config examples/audit.toml \
    --input.git.ref set-1e5 \
    --api-filter-def '{"txnFilter":{"TxnFilterPostingAccount":{"regex":"a:ay2016:am12"}}}'
````

The transaction filter definition could be given also 
as Base64 ascii armored string:

````
--api-filter-def \
base64:eyJ0eG5GaWx0ZXIiOnsiVHhuRmlsdGVyUG9zdGluZ0FjY291bnQiOnsicmVnZXgiOiJhOmF5MjAxNjphbTEyIn19fQ==
````


#### Output

````
Git Storage
      reference : set-1e5
      directory : txns
      extension : txn
         commit : 4648a2994b41ed341b544a148b3060fd2d267d79
         author : tackler <accounting@example.com>
           date : 2016-12-31 17:41:24 +0000
        subject : set-1e5: 2016-12-31

Txn Set Checksum
        SHA-256 : 8ff80f72dc8b36fd6272319d08b99c54d8cf47baff47eda3df68b69e7b7b66bc
       Set size : 8405

Filter
  Posting Account: "a:ay2016:am12"

**********************************************************************************
Account Selector Checksum
        SHA-256 : df4714ff3f5bd031a8256df95863a8085f740b436f1b47febb45c9eb9aaa9e9e

Balance Report
--------------
              -133433.00   a:ay2016:am12
========================
              -133433.00
##################################################################################
````

## Further info

* [Tackler Journal Format](https://tackler.e257.fi/docs/journal/format/)
* [Txn Filters with Shell Script](https://tackler.e257.fi/docs/usage/#txn-filters-shell)
* [Tackler-NG repository](https://github.com/tackler-ng/tackler)
* [Tackler website](https://tackler.e257.fi/)
* [Plain Text Accounting](https://plaintextaccounting.org/)


## Tackler components on Crates.io

* Tackler CLI application: [tackler](https://crates.io/crates/tackler)
* Tackler Client API: [tackler-api](https://crates.io/crates/tackler-api)
* Tackler Server API: [tackler-core](https://crates.io/crates/tackler-core)
* Tackler Rusty Services: [tackler-rs](https://crates.io/crates/tackler-rs)
