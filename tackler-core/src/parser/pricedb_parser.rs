/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use itertools::Itertools;
use winnow::{
    Parser,
    combinator::{eof, opt, preceded, repeat_till},
};

use crate::kernel::Settings;
use crate::model::price_entry::PriceDb;
use crate::parser::Stream;

use super::parts::{pricedb::parse_price_entry, txns::multispace0_line_ending};

use crate::tackler;
use std::path::Path;

/// # Errors
/// Returns `Err` in case of invalid pricedb data
pub fn pricedb_from_str(
    input: &mut &str,
    settings: &mut Settings,
) -> Result<PriceDb, tackler::Error> {
    let is = Stream {
        input,
        state: settings,
    };

    preceded(
        opt(multispace0_line_ending),
        repeat_till(1.., parse_price_entry, eof),
    )
    .parse(is)
    .map(|(price_entries, _): (Vec<_>, _)| price_entries.into_iter().sorted().dedup().collect())
    .map_err(|err| err.to_string().into())
}

/// # Errors
/// Returns `Err` in case of invalid pricedb data
pub fn pricedb_from_file(path: &Path, settings: &mut Settings) -> Result<PriceDb, tackler::Error> {
    let pricedb_str = std::fs::read_to_string(path)
        .map_err(|err| format!("Can't open file: '{}' - {}", path.display(), err))?;

    // todo: error log
    pricedb_from_str(&mut pricedb_str.as_str(), settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    fn test_parse_pricedb() {
        #[rustfmt::skip]
        let pok_pricedbs = vec![
            (indoc!(
               "|P 2024-01-09 XAU 2659.645203 USD
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|P 2024-01-09 XAU 2659.645203 USD
                |P 2024-01-09 USD  121.306155 BDT
                |P 2024-01-09 XAG  3652.77663 BDT
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|\t \n\
                | \t  \t
                |P 2024-01-09 XAU 2659.645203 USD
                |P 2024-01-09 USD  121.306155 BDT
                |P 2024-01-09 XAG  3652.77663 BDT
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|P 2024-01-09 XAU 2659.645203 USD
                |P 2024-01-09 USD  121.306155 BDT
                |P 2024-01-09 XAG  3652.77663 BDT
                |\t \n\
                | \t  \t
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|\t \n\
                | \t  \t
                |P 2024-01-09 XAU 2659.645203 USD
                |P 2024-01-09 USD  121.306155 BDT
                |P 2024-01-09 XAG  3652.77663 BDT
                |\t \n\
                | \t  \t
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|\t \n\
                | \t  \t
                |P 2024-01-09 XAU 2659.645203 USD
                |\t \n\
                | \t  \t
                |P 2024-01-09 USD  121.306155 BDT
                |P 2024-01-09 XAG  3652.77663 BDT
                |\t \n\
                | \t  \t
                |"
            ).strip_margin(), 3usize),
        ];

        let mut count = 0;
        let pok_count = pok_pricedbs.len();
        for t in pok_pricedbs {
            let mut settings = Settings::default();

            let res = pricedb_from_str(&mut t.0.as_str(), &mut settings);

            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: {}\n",
                count + 1
            );

            let pricedb = res.unwrap(/*:test:*/);
            assert_eq!(
                pricedb.len(),
                t.1,
                "\nWrong price entry count: Offending test vector item: {}",
                count + 1
            );

            count += 1;
        }
        assert_eq!(count, pok_count);
    }
}
