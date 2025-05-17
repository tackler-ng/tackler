/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Scale;
use rust_decimal::{Decimal, RoundingStrategy};

/// Format number with scale setting
///
/// The number is rounded based on `Scale.max` and field min length is
/// based on `width`, which is used as minimum width (use zero to disable filling).
/// The number is right side aligned on the field.
///
/// Used rounding strategy is "midpoint away from zero"
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn format_with_scale(width: usize, v: &Decimal, scale: &Scale) -> String {
    let prec = scale.get_precision(v);
    format!(
        "{:>width$.prec$}",
        v.round_dp_with_strategy(prec as u32, RoundingStrategy::MidpointAwayFromZero)
    )
}

#[cfg(test)]
mod tests {
    use crate::config::Scale;
    use crate::math::format::format_with_scale;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::str::FromStr;

    const SC1_0: Decimal = dec!(1);
    const SC1_1: Decimal = dec!(1.1);
    const SC1_2: Decimal = dec!(1.12);
    const SC1_3: Decimal = dec!(1.123);
    const SC1_6: Decimal = dec!(1.123456);
    const SC1_8_TR7: Decimal = dec!(1.12345677);
    const SC1_8_TR5: Decimal = dec!(1.12345675);
    const SC1_8_TR4: Decimal = dec!(1.12345674);

    const SC10_2: Decimal = dec!(1234567890.12);
    const SC14_4: Decimal = dec!(12345678901234.1234);

    const SC18_2: Decimal = dec!(123456789012345678.12);
    const SC18_9_REF: &str = "123456789012345678.123456789";
    const SC18_9: Decimal = dec!(123456789012345678.123456789);

    // test: 52a72e6e-0d5d-4620-af1c-c6edf0143d82
    #[test]
    fn test_format_positive_values() {
        let scale = Scale::default();

        assert_eq!(format_with_scale(0, &SC1_0, &scale), "1.00".to_string());
        assert_eq!(format_with_scale(0, &SC1_1, &scale), "1.10".to_string());
        assert_eq!(format_with_scale(0, &SC1_2, &scale), "1.12".to_string());
        assert_eq!(format_with_scale(0, &SC1_3, &scale), "1.123".to_string());
        assert_eq!(format_with_scale(0, &SC1_6, &scale), "1.123456".to_string());
        assert_eq!(
            format_with_scale(0, &SC10_2, &scale),
            "1234567890.12".to_string()
        );

        assert_eq!(format_with_scale(1, &SC1_0, &scale), "1.00".to_string());
        assert_eq!(format_with_scale(4, &SC1_0, &scale), "1.00".to_string());
        assert_eq!(format_with_scale(5, &SC1_1, &scale), " 1.10".to_string());
        assert_eq!(format_with_scale(5, &SC1_2, &scale), " 1.12".to_string());
        assert_eq!(format_with_scale(6, &SC1_3, &scale), " 1.123".to_string());
        assert_eq!(format_with_scale(6, &SC1_0, &scale), "  1.00".to_string());

        assert_eq!(
            format_with_scale(10, &SC1_6, &scale),
            "  1.123456".to_ascii_lowercase()
        );
        assert_eq!(
            format_with_scale(15, &SC10_2, &scale),
            "  1234567890.12".to_string()
        );
    }

    // test: 8fcfae80-7a06-49dc-b449-7cfb0cf49c2d
    #[test]
    fn test_format_negative_values() {
        let scale = Scale::default();

        assert_eq!(format_with_scale(0, &-SC1_0, &scale), "-1.00".to_string());
        assert_eq!(format_with_scale(0, &-SC1_1, &scale), "-1.10".to_string());
        assert_eq!(format_with_scale(0, &-SC1_2, &scale), "-1.12".to_string());
        assert_eq!(format_with_scale(0, &-SC1_3, &scale), "-1.123".to_string());
        assert_eq!(
            format_with_scale(0, &-SC1_6, &scale),
            "-1.123456".to_string()
        );
        assert_eq!(
            format_with_scale(0, &-SC10_2, &scale),
            "-1234567890.12".to_string()
        );

        assert_eq!(format_with_scale(1, &-SC1_0, &scale), "-1.00".to_string());
        assert_eq!(format_with_scale(5, &-SC1_0, &scale), "-1.00".to_string());
        assert_eq!(format_with_scale(6, &-SC1_0, &scale), " -1.00".to_string());
        assert_eq!(format_with_scale(5, &-SC1_1, &scale), "-1.10".to_string());
        assert_eq!(format_with_scale(5, &-SC1_2, &scale), "-1.12".to_string());
        assert_eq!(format_with_scale(6, &-SC1_3, &scale), "-1.123".to_string());

        assert_eq!(
            format_with_scale(10, &-SC1_6, &scale),
            " -1.123456".to_string()
        );
        assert_eq!(
            format_with_scale(15, &-SC10_2, &scale),
            " -1234567890.12".to_string()
        );
    }

    // test: be4cec3b-b025-4dbd-9331-e78896843f04
    #[test]
    fn truncate_values_correctly() {
        let scale = Scale::default();
        assert_eq!(
            format_with_scale(0, &SC1_8_TR7, &scale),
            "1.1234568".to_string()
        );
        assert_eq!(
            format_with_scale(0, &SC1_8_TR5, &scale),
            "1.1234568".to_string()
        );
        assert_eq!(
            format_with_scale(0, &SC1_8_TR4, &scale),
            "1.1234567".to_string()
        );

        assert_eq!(
            format_with_scale(10, &SC1_8_TR7, &scale),
            " 1.1234568".to_string()
        );
        assert_eq!(
            format_with_scale(10, &SC1_8_TR5, &scale),
            " 1.1234568".to_string()
        );
        assert_eq!(
            format_with_scale(10, &SC1_8_TR4, &scale),
            " 1.1234567".to_string()
        );
    }

    // test: 77f9a99e-ef0a-47c4-a8c9-59f3d4478f31
    #[test]
    fn test_format_large_numbers() {
        let scale = Scale::default();
        assert_eq!(
            format_with_scale(0, &SC10_2, &scale),
            "1234567890.12".to_string()
        );
        assert_eq!(
            format_with_scale(0, &SC14_4, &scale),
            "12345678901234.1234".to_string()
        );
        assert_eq!(
            format_with_scale(0, &SC18_2, &scale),
            "123456789012345678.12".to_string()
        );
        assert_eq!(
            format_with_scale(0, &SC18_9, &scale),
            "123456789012345678.1234568".to_string()
        );

        assert_eq!(
            format_with_scale(22, &SC18_2, &scale),
            " 123456789012345678.12".to_string()
        );
        assert_eq!(
            format_with_scale(27, &SC18_9, &scale),
            " 123456789012345678.1234568".to_string()
        );
    }

    /*
     * //
     * // orig: 1cf0c2c7-35a9-42b3-b916-8d3a20a9d428
     * //
     * sc30_130 = TacklerReal(
     * // |3        |2        |1        |        1|        2|        3|        4|        5|        6|        7|        8|        9|       10|       11|       12|       13|
     *   "123456789012345678901234567890.123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
     * it("format, truncate and round with very large numbers (30 digits) with high precision (128 decimals)") {
     *    class LargeScale extends ReportConfiguration {
     *       override val minScale = 2
     *       override val maxScale = 128
     *    }
     *    val largeFrmt = new Frmt("", new LargeScale())
     *    val ref = "123456789012345678901234567890.1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234568"
     *
     *    assert(largeFrmt.scaleFormat(sc30_130) === ref)
     *    assert(largeFrmt.fillFormat(160, sc30_130) === " " + ref)
     * }
     */

    // test: 1cf0c2c7-35a9-42b3-b916-8d3a20a9d428
    #[test]
    fn test_format_large_numbers_with_high_precision() {
        let scale = Scale::try_from(2, 28).unwrap(/*:test:*/);
        let sc24_4_txt = "123456789012345678901234.1234";
        let sc24_4 = Decimal::from_str(
            sc24_4_txt
        ).unwrap(/*:test:*/);

        // Avogadro number is 6.023x10^23
        let sc5_23_txt = "92345.12345678901234567890123";
        let sc5_23 = Decimal::from_str(sc5_23_txt).unwrap(/*:test:*/);

        assert_eq!(format_with_scale(0, &SC18_9, &scale), SC18_9_REF);
        assert_eq!(
            format_with_scale(0, &sc24_4, &scale),
            sc24_4_txt.to_string()
        );
        assert_eq!(
            format_with_scale(0, &sc5_23, &scale),
            sc5_23_txt.to_string()
        );

        assert_eq!(
            format_with_scale(32, &sc24_4, &scale),
            format!("   {sc24_4_txt}")
        );
        assert_eq!(
            format_with_scale(32, &sc5_23, &scale),
            format!("   {sc5_23_txt}")
        );
    }

    // test: f82c5cbc-2f8b-4c81-9732-36e85807b754
    #[test]
    fn verify_rounding_mode_half_up_1_1() {
        let scale = Scale::try_from(0,0).unwrap(/*:test:*/);

        let test_cases = vec![
            (dec!(5.5), "6", " 6"),
            (dec!(2.5), "3", " 3"),
            (dec!(1.6), "2", " 2"),
            (dec!(1.1), "1", " 1"),
            (dec!(1.0), "1", " 1"),
            (dec!(-1.0), "-1", "-1"),
            (dec!(-1.1), "-1", "-1"),
            (dec!(-1.6), "-2", "-2"),
            (dec!(-2.5), "-3", "-3"),
            (dec!(-5.5), "-6", "-6"),
        ];

        for (value, scale_ref, fill_ref) in test_cases {
            assert_eq!(format_with_scale(0, &value, &scale), scale_ref.to_string());
            assert_eq!(format_with_scale(2, &value, &scale), fill_ref.to_string());
        }
    }
    #[test]
    fn verify_rounding_mode_half_up_1_2() {
        let scale = Scale::try_from(2,2).unwrap(/*:test:*/);

        let test_cases = vec![
            (dec!(0.055), "0.06", " 0.06"),
            (dec!(0.025), "0.03", " 0.03"),
            (dec!(0.016), "0.02", " 0.02"),
            (dec!(0.011), "0.01", " 0.01"),
            (dec!(0.010), "0.01", " 0.01"),
            (dec!(-0.010), "-0.01", "-0.01"),
            (dec!(-0.011), "-0.01", "-0.01"),
            (dec!(-0.016), "-0.02", "-0.02"),
            (dec!(-0.025), "-0.03", "-0.03"),
            (dec!(-0.055), "-0.06", "-0.06"),
        ];

        for (value, scale_ref, fill_ref) in test_cases {
            assert_eq!(format_with_scale(0, &value, &scale), scale_ref.to_string());
            assert_eq!(format_with_scale(5, &value, &scale), fill_ref.to_string());
        }
    }
}
