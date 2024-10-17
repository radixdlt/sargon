use crate::prelude::*;

/// Tries to creates a new `Decimal192` from a String, throws a `CommonError`
/// if the `string` was not a valid Decimal192.
#[uniffi::export]
pub fn new_decimal_from_string(string: String) -> Result<Decimal192> {
    Decimal192::new(string)
}

/// Tries to creates a new `Decimal192` from a formatted String for
/// a specific locale.
#[uniffi::export]
pub fn new_decimal_from_formatted_string(
    formatted_string: String,
    locale: LocaleConfig,
) -> Result<Decimal192> {
    Decimal192::new_with_formatted_string(formatted_string, locale)
}

/// The standard transaction fee
#[uniffi::export]
pub fn transaction_fee_preset() -> Decimal192 {
    Decimal192::transaction_fee_preset()
}

/// Creates a new `Decimal192` from a u32 integer.
#[uniffi::export]
pub fn new_decimal_from_u32(value: u32) -> Decimal192 {
    value.into()
}

/// Creates a new `Decimal192` from a u64 integer.
#[uniffi::export]
pub fn new_decimal_from_u64(value: u64) -> Decimal192 {
    value.into()
}

/// Creates a new `Decimal192` from a i32 integer.
#[uniffi::export]
pub fn new_decimal_from_i32(value: i32) -> Decimal192 {
    value.into()
}

/// Creates a new `Decimal192` from a i64 integer.
#[uniffi::export]
pub fn new_decimal_from_i64(value: i64) -> Decimal192 {
    value.into()
}

#[uniffi::export]
pub fn decimal_formatted(
    decimal: &Decimal192,
    locale: LocaleConfig,
    total_places: u8,
    use_grouping_separator: bool,
) -> String {
    decimal.formatted(locale, total_places, use_grouping_separator)
}

/// A human readable, locale respecting string. Does not perform any rounding or truncation.
#[uniffi::export]
pub fn decimal_formatted_plain(
    decimal: &Decimal192,
    locale: LocaleConfig,
    use_grouping_separator: bool,
) -> String {
    decimal.formatted_plain(locale, use_grouping_separator)
}

/// Creates a new `Decimal192` from a f32 float. Will
/// fail if the f32 cannot be losslessly represented
/// by the underlying Decimal from Scrypto.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert!(new_decimal_from_f32(208050.17).to_string() == "208050.17");
///
/// assert!(new_decimal_from_f32(f32::MIN_POSITIVE).to_string() == "0");
/// ```
#[uniffi::export]
pub fn new_decimal_from_f32(value: f32) -> Decimal192 {
    value.into()
}

/// Creates a new `Decimal192` from a f64 float. Will
/// fail if the f64 cannot be losslessly represented
/// by the underlying Decimal from Scrypto.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert!(new_decimal_from_f64(208050.17).is_ok());
///
/// assert!(new_decimal_from_f64(f64::MIN_POSITIVE).is_ok());
/// ```
#[uniffi::export]
pub fn new_decimal_from_f64(value: f64) -> Result<Decimal192> {
    value.try_into()
}

/// The minimum possible value of `Decimal192`, being:
/// `-3138550867693340381917894711603833208051.177722232017256448`
#[uniffi::export]
pub fn decimal_min() -> Decimal192 {
    Decimal192::min()
}

/// The maximum possible value of `Decimal192`, being:
/// `3138550867693340381917894711603833208051.177722232017256447`
#[uniffi::export]
pub fn decimal_max() -> Decimal192 {
    Decimal192::max()
}

/// Creates the Decimal192 `10^exponent`
#[uniffi::export]
pub fn new_decimal_exponent(exponent: u8) -> Decimal192 {
    Decimal192::pow(exponent)
}

/// `decimal.to_string()`
#[uniffi::export]
pub fn decimal_to_string(decimal: &Decimal192) -> String {
    decimal.to_string()
}

/// `lhs < rhs`
#[uniffi::export]
pub fn decimal_less_than(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs < rhs
}

/// `lhs <= rhs`
#[uniffi::export]
pub fn decimal_less_than_or_equal(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs <= rhs
}

/// `lhs > rhs`
#[uniffi::export]
pub fn decimal_greater_than(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs > rhs
}

/// `lhs >= rhs`
#[uniffi::export]
pub fn decimal_greater_than_or_equal(
    lhs: &Decimal192,
    rhs: &Decimal192,
) -> bool {
    lhs >= rhs
}

/// Whether this decimal is zero.
#[uniffi::export]
pub fn decimal_is_zero(decimal: &Decimal192) -> bool {
    decimal.is_zero()
}

/// Whether this decimal is positive.
#[uniffi::export]
pub fn decimal_is_positive(decimal: &Decimal192) -> bool {
    decimal.is_positive()
}

/// Whether this decimal is negative.
#[uniffi::export]
pub fn decimal_is_negative(decimal: &Decimal192) -> bool {
    decimal.is_negative()
}

/// `lhs + rhs``
#[uniffi::export]
pub fn decimal_add(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    lhs + rhs
}

/// `lhs - rhs``
#[uniffi::export]
pub fn decimal_sub(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    lhs - rhs
}

/// `lhs * rhs``
#[uniffi::export]
pub fn decimal_mul(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    lhs * rhs
}

/// `lhs / rhs``
#[uniffi::export]
pub fn decimal_div(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    lhs / rhs
}

/// Negates the `decimal`
#[uniffi::export]
pub fn decimal_neg(decimal: &Decimal192) -> Decimal192 {
    decimal.neg()
}

/// Returns `decimal.abs()`, panics if `decimal` is `Decimal192::MIN`
#[uniffi::export]
pub fn decimal_abs(decimal: &Decimal192) -> Decimal192 {
    decimal.abs()
}

/// Clamps `decimal` to zero, i.e. `max(decimal, 0)`
#[uniffi::export]
pub fn decimal_clamped_to_zero(decimal: &Decimal192) -> Decimal192 {
    decimal.clamped_to_zero()
}

/// Rounds this number to the specified decimal places.
///
/// # Panics
/// - Panic if the number of decimal places is not within [0..SCALE(=18)]
#[uniffi::export]
pub fn decimal_round(
    decimal: &Decimal192,
    decimal_places: u8,
    rounding_mode: RoundingMode,
) -> Result<Decimal192> {
    decimal.round_with_mode(decimal_places, rounding_mode)
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Decimal192;

    #[test]
    fn to_string() {
        let s = "58947355801916604025588861116008628224.04321";
        let a: Decimal192 = s.parse().unwrap();

        assert_eq!(decimal_to_string(&a), s);
    }

    #[test]
    fn arithmetic() {
        let zero = SUT::zero();
        let one = SUT::one();
        let two = SUT::two();
        let three = SUT::three();
        let four = SUT::four();
        let five = SUT::five();
        let six = SUT::six();
        let seven = SUT::seven();
        let eight = SUT::eight();
        let nine = SUT::nine();
        let ten = SUT::ten();

        assert_eq!(zero, new_decimal_from_i32(0));
        assert_eq!(one, new_decimal_from_i64(1));
        assert_eq!(two, new_decimal_from_u32(2));
        assert_eq!(three, new_decimal_from_u64(3));
        assert_eq!(four, new_decimal_from_string("4".to_string()).unwrap());

        assert_eq!(zero + zero, zero);
        assert_eq!(one + zero, one);
        assert_eq!(one + one, two);
        assert_eq!(one + two, three);
        assert_eq!(two + one, three);
        assert_eq!(two + two, four);
        assert_eq!(two + three, five);
        assert_eq!(three + three, six);
        assert_eq!(three + four, seven);
        assert_eq!(four + four, eight);
        assert_eq!(four + five, nine);
        assert_eq!(five + four, nine);
        assert_eq!(five + five, ten);
        assert_eq!(ten + zero, ten);

        assert_eq!(zero * zero, zero);
        assert_eq!(one * zero, zero);
        assert_eq!(two * zero, zero);
        assert_eq!(three * zero, zero);

        assert_eq!(one * one, one);
        assert_eq!(one * two, two);
        assert_eq!(one * three, three);
        assert_eq!(one * four, four);

        assert_eq!(two * two, four);
        assert_eq!(two * three, six);
        assert_eq!(two * four, eight);
        assert_eq!(two * five, ten);
        assert_eq!(three * three, nine);

        assert_eq!(one / one, one);
        assert_eq!(two / one, two);
        assert_eq!(three / one, three);
        assert_eq!(four / one, four);

        assert_eq!(two / two, one);
        assert_eq!(three / three, one);
        assert_eq!(four / four, one);
        assert_eq!(five / five, one);
        assert_eq!(ten / ten, one);
        assert_eq!(nine / three, three);

        assert_eq!(ten - ten, zero);
        assert_eq!(nine - nine, zero);
        assert_eq!(two - two, zero);
        assert_eq!(zero - zero, zero);
        assert_eq!(seven - two, five);

        assert_eq!(decimal_add(zero, zero), zero);
        assert_eq!(decimal_add(one, zero), one);

        assert_eq!(decimal_add(one, one), two);
        assert_eq!(decimal_add(one, two), three);
        assert_eq!(decimal_add(two, one), three);
        assert_eq!(decimal_add(two, two), four);
        assert_eq!(decimal_add(two, three), five);
        assert_eq!(decimal_add(three, three), six);
        assert_eq!(decimal_add(three, four), seven);
        assert_eq!(decimal_add(three, four), seven);
        assert_eq!(decimal_add(four, four), eight);
        assert_eq!(decimal_add(four, five), nine);
        assert_eq!(decimal_add(five, four), nine);
        assert_eq!(decimal_add(five, five), ten);
        assert_eq!(decimal_add(ten, zero), ten);

        assert_eq!(decimal_mul(zero, zero), zero);
        assert_eq!(decimal_mul(zero, zero), zero);
        assert_eq!(decimal_mul(one, zero), zero);
        assert_eq!(decimal_mul(two, zero), zero);
        assert_eq!(decimal_mul(zero, zero), zero);

        assert_eq!(decimal_mul(one, one), one);
        assert_eq!(decimal_mul(one, two), two);
        assert_eq!(decimal_mul(one, three), three);
        assert_eq!(decimal_mul(one, four), four);

        assert_eq!(decimal_mul(two, two), four);
        assert_eq!(decimal_mul(two, three), six);
        assert_eq!(decimal_mul(two, four), eight);
        assert_eq!(decimal_mul(two, five), ten);
        assert_eq!(decimal_mul(three, three), nine);

        assert_eq!(decimal_div(one, one), one);
        assert_eq!(decimal_div(two, one), two);
        assert_eq!(decimal_div(three, one), three);
        assert_eq!(decimal_div(four, one), four);

        assert_eq!(decimal_div(two, two), one);
        assert_eq!(decimal_div(three, three), one);
        assert_eq!(decimal_div(four, four), one);
        assert_eq!(decimal_div(five, five), one);
        assert_eq!(decimal_div(ten, ten), one);
        assert_eq!(decimal_div(nine, three), three);

        assert_eq!(decimal_sub(ten, ten), zero);
        assert_eq!(decimal_sub(nine, nine), zero);
        assert_eq!(decimal_sub(zero, zero), zero);
        assert_eq!(decimal_sub(seven, two), five);
    }

    #[test]
    fn exponent() {
        assert_eq!(new_decimal_exponent(0).to_string(), "1");
        assert_eq!(new_decimal_exponent(1).to_string(), "10");
        assert_eq!(new_decimal_exponent(2).to_string(), "100");
        assert_eq!(new_decimal_exponent(3).to_string(), "1000");
        assert_eq!(new_decimal_exponent(4).to_string(), "10000");
        assert_eq!(
            new_decimal_exponent(20).to_string(),
            "100000000000000000000"
        );
        assert_eq!(
            new_decimal_exponent(39).to_string(),
            "1000000000000000000000000000000000000000"
        );
    }

    #[test]
    #[should_panic(expected = "Too large exponent, 10^39 is max.")]
    fn exponent_too_large() {
        _ = new_decimal_exponent(40);
    }

    #[test]
    fn decimal_min_abs_is_decimal_max() {
        assert_eq!(SUT::min().abs(), SUT::max())
    }

    #[test]
    fn compare() {
        let zero = new_decimal_from_i32(0);
        let one = new_decimal_from_i64(1);
        let two = new_decimal_from_u32(2);
        let three = new_decimal_from_u64(3);

        assert!(decimal_less_than(&zero, &one));
        assert!(decimal_less_than(&zero, &two));
        assert!(decimal_less_than(&one, &two));
        assert!(!decimal_less_than(&zero, &zero));
        assert!(decimal_less_than_or_equal(&zero, &zero));
        assert!(decimal_less_than_or_equal(&zero, &one));
        assert!(decimal_less_than_or_equal(&zero, &two));
        assert!(decimal_less_than_or_equal(&one, &two));
        assert!(decimal_less_than_or_equal(&two, &three));
        assert!(decimal_less_than_or_equal(&three, &three));

        assert!(!decimal_greater_than(&three, &three));
        assert!(decimal_greater_than_or_equal(&three, &three));
        assert!(decimal_greater_than_or_equal(&three, &two));
        assert!(decimal_greater_than(&three, &two));
        assert!(decimal_greater_than(&three, &one));
        assert!(decimal_greater_than(&two, &one));
        assert!(!decimal_greater_than(&one, &one));
        assert!(decimal_greater_than(&one, &zero));
    }

    #[test]
    fn is_zero() {
        assert!(decimal_is_zero(&SUT::zero()));
        assert!(!decimal_is_zero(&SUT::one()));
    }

    #[test]
    fn is_positive() {
        // `0` is neither positive nor negative
        // https://en.wikipedia.org/wiki/0
        assert!(!decimal_is_positive(&SUT::zero()));

        assert!(decimal_is_positive(&SUT::one()));
        assert!(!decimal_is_positive(&decimal_neg(&SUT::one())));
    }

    #[test]
    fn is_negative() {
        // `0` is neither positive nor negative
        // https://en.wikipedia.org/wiki/0
        assert!(!decimal_is_negative(&SUT::zero()));

        assert!(!decimal_is_negative(&SUT::one()));
        assert!(decimal_is_negative(&decimal_neg(&SUT::one())));
    }

    #[test]
    fn min() {
        assert_eq!(
            decimal_min().to_string(),
            "-3138550867693340381917894711603833208051.177722232017256448"
        );
    }

    #[test]
    fn max() {
        assert_eq!(
            decimal_max().to_string(),
            "3138550867693340381917894711603833208051.177722232017256447"
        );
    }

    #[test]
    fn transaction_fee_preset_value() {
        assert_eq!(
            Decimal192::transaction_fee_preset(),
            transaction_fee_preset()
        );
    }

    #[test]
    fn from_f32() {
        let f: f32 = 208050.17;
        assert_eq!(f.to_string(), "208050.17");
        let sut = new_decimal_from_f32(f);
        assert_eq!(sut.to_string(), "208050.17");
        assert_eq!(
            SUT::from(f32::MAX).to_string(),
            "340282350000000000000000000000000000000"
        );
        assert_eq!(SUT::from(f32::MIN_POSITIVE).to_string(), "0");
    }

    #[test]
    fn from_f64() {
        let f: f64 = 208050.17;
        assert_eq!(f.to_string(), "208050.17");
        let sut = new_decimal_from_f64(f);
        assert_eq!(sut.unwrap().to_string(), "208050.17");
        assert_eq!(
            SUT::try_from(f32::MAX as f64).unwrap().to_string(),
            "340282346638528860000000000000000000000"
        );
        assert_eq!(
            SUT::try_from(f32::MIN_POSITIVE as f64).unwrap().to_string(),
            "0"
        );
    }

    #[test]
    fn rounding() {
        let mut sut: SUT = "3.1".parse().unwrap();
        let mut mode: RoundingMode = RoundingMode::ToPositiveInfinity;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(4)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-3)
        );

        mode = RoundingMode::ToNegativeInfinity;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(3)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-4)
        );

        mode = RoundingMode::ToZero;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(3)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-3)
        );

        mode = RoundingMode::AwayFromZero;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(4)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-4)
        );

        sut = "3.5".parse().unwrap();
        mode = RoundingMode::ToNearestMidpointTowardZero;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(3)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-3)
        );

        mode = RoundingMode::ToNearestMidpointAwayFromZero;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(4)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-4)
        );

        mode = RoundingMode::ToNearestMidpointToEven;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            new_decimal_from_i32(4)
        );
        assert_eq!(
            decimal_round(&-sut, 0, mode).unwrap(),
            new_decimal_from_i32(-4)
        );

        // more decimals
        sut = "2.4595".parse().unwrap();
        mode = RoundingMode::AwayFromZero;
        assert_eq!(
            decimal_round(&sut, 0, mode).unwrap(),
            "3".parse::<SUT>().unwrap()
        );
        assert_eq!(
            decimal_round(&sut, 1, mode).unwrap(),
            "2.5".parse::<SUT>().unwrap()
        );
        assert_eq!(
            decimal_round(&sut, 2, mode).unwrap(),
            "2.46".parse::<SUT>().unwrap()
        );
        assert_eq!(
            decimal_round(&sut, 3, mode).unwrap(),
            "2.46".parse::<SUT>().unwrap()
        );

        let max: SUT =
            "3138550867693340381917894711603833208051.177722232017256447"
                .parse()
                .unwrap();

        assert!(max
            .round_with_mode(0, RoundingMode::ToNearestMidpointAwayFromZero)
            .is_ok());
        assert!(max
            .round_with_mode(0, RoundingMode::ToNearestMidpointTowardZero)
            .is_ok());
        assert!(max.round_with_mode(0, RoundingMode::ToZero).is_ok());

        assert!(max
            .round_with_mode(18, RoundingMode::ToNearestMidpointAwayFromZero)
            .is_ok());
        assert!(max
            .round_with_mode(18, RoundingMode::ToNearestMidpointTowardZero)
            .is_ok());
        assert!(max.round_with_mode(18, RoundingMode::ToZero).is_ok());

        assert!(max.round_with_mode(0, RoundingMode::AwayFromZero).is_err());
    }

    #[test]
    fn abs() {
        let sut = -SUT::one();
        assert_eq!(decimal_abs(&sut), SUT::one());
    }

    #[test]
    fn clamped() {
        assert_eq!(decimal_clamped_to_zero(&-SUT::one()), SUT::zero());
        assert_eq!(decimal_clamped_to_zero(&SUT::one()), SUT::one());
    }

    #[test]
    fn from_formatted_string() {
        let test = |s: &str, l: &LocaleConfig, exp: &str| {
            assert_eq!(
                new_decimal_from_formatted_string(s.to_owned(), l.clone())
                    .unwrap(),
                exp.parse::<Decimal192>().unwrap()
            )
        };
        let fail = |s: &str, l: &LocaleConfig| {
            assert!(new_decimal_from_formatted_string(s.to_owned(), l.clone())
                .is_err())
        };
        let swedish = LocaleConfig::swedish_sweden();
        let us = LocaleConfig::english_united_states();
        test(",005", &swedish, "0.005");
        test(".005", &us, "0.005");
        test("1,001", &swedish, "1.001");
        test("1,001", &us, "1001");

        fail("1,000,000.23", &swedish);
        test("1,000,000.23", &us, "1000000.23");
    }

    #[test]
    fn formatted() {
        assert_eq!(
            decimal_formatted(
                &SUT::max(),
                LocaleConfig::english_united_states(),
                4,
                true
            ),
            "3.138e39"
        );
        assert_eq!(
            decimal_formatted(
                &SUT::from("12345678.975"),
                LocaleConfig::default(),
                8,
                true
            ),
            "12.345679 M"
        );
    }

    #[test]
    fn formatted_plain() {
        assert_eq!(
            decimal_formatted_plain(
                &SUT::from("123456789.042"),
                LocaleConfig::english_united_states(),
                true
            ),
            "123,456,789.042"
        );
        assert_eq!(
            decimal_formatted_plain(
                &SUT::from("123456789.042"),
                LocaleConfig::english_united_states(),
                false
            ),
            "123456789.042"
        );
        assert_eq!(
            decimal_formatted_plain(
                &SUT::from("123456789.042"),
                LocaleConfig::swedish_sweden(),
                true
            ),
            "123\u{a0}456\u{a0}789,042"
        );
        assert_eq!(
            decimal_formatted_plain(
                &SUT::from("123456789.042"),
                LocaleConfig::swedish_sweden(),
                false
            ),
            "123456789,042"
        );
    }
}
