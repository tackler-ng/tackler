/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use criterion::{Criterion, criterion_group, criterion_main};
use indoc::indoc;
use tackler_core::kernel::Settings;
use tackler_core::parser::string_to_txns;
use tackler_rs::IndocUtils;

fn criterion_benchmark(c: &mut Criterion) {
    let mut settings = Settings::default();

    #[rustfmt::skip]
    let input = 
        indoc!(
            "|2024-12-31T23:58:59.123456789+02:00 (#001) 'bells 'n whistles
             | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
             | # tags: cef, first, second
             | # location: geo:1.111,2.222,3.333
             | ; first txn comment
             | ; second txn comment
             | e:d:f 26 bar·He_50L @ 1.25 EUR ; 32.50 EUR
             | a:b:c
             |
             |").strip_margin();

    c.bench_function("parser", |b| {
        b.iter(|| {
            let res = string_to_txns(&mut input.as_str(), &mut settings);
            assert!(res.is_ok());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
