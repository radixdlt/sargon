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

/// Formats decimal using engineering notation: `5e20`.
///
/// If no `None` is passed to `total_places`, then
/// `Self::MAX_PLACES_ENGINEERING_NOTATION` (4) will
/// be used.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
/// #[allow(clippy::upper_case_acronyms)]
/// type SUT = Decimal192;
///
/// assert_eq!(SUT::max().formatted_engineering_notation(LocaleConfig::default(), None), "3.138e39");
/// assert_eq!(SUT::min().formatted_engineering_notation(LocaleConfig::default(), None), "-3.138e39");
/// assert_eq!(SUT::MAX_PLACES_ENGINEERING_NOTATION, 4);
/// ```
///
#[uniffi::export]
pub fn decimal_formatted_engineering_notation(
    decimal: &Decimal192,
    locale: LocaleConfig,
    total_places: Option<u8>,
) -> String {
    decimal.formatted_engineering_notation(locale, total_places)
}

/// Creates a new `Decimal192` from a f32 float. Will
/// fail if the f32 cannot be losslessly represented
/// by the underlying Decimal from Scrypto.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert!(new_decimal_from_f32(208050.17).is_ok());
///
/// assert_eq!(
///     new_decimal_from_f32(f32::MIN_POSITIVE),
///     Err(CommonError::DecimalOverflow { bad_value: f32::MIN_POSITIVE.to_string() })
/// );
/// ```
#[uniffi::export]
pub fn new_decimal_from_f32(value: f32) -> Result<Decimal192> {
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
