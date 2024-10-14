use crate::prelude::*;
use sargon::Decimal192 as InternalDecimal192;

/// `Decimal192` represents a 192 bit representation of a fixed-scale decimal number.
///
/// The finite set of values are of the form `m / 10^18`, where `m` is
/// an integer such that `-2^(192 - 1) <= m < 2^(192 - 1)`.
///
/// Fractional part: ~60 bits/18 digits
/// Integer part   : 132 bits /40 digits
/// Max            :  3138550867693340381917894711603833208051.177722232017256447
/// Min            : -3138550867693340381917894711603833208051.177722232017256448
///
/// Unless otherwise specified, all operations will panic if underflow/overflow.
///
/// Powering it is the [Scrypto Decimal type, see docs][scrypto].
///
/// Note: This type cannot be called `Decimal`, since it results in naming collision
/// in the Swift land (clash with `Foundation.Decimal`) instead we have created a
/// type alias `Decimal = Decimal192` which we use in Rust land.
///
/// [scrypto]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/math/decimal.rs#L42
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Decimal192 {
    secret_magic: String,
}

pub type Decimal = Decimal192;

impl Decimal192 {
    pub fn into_internal(&self) -> InternalDecimal192 {
        self.clone().into()
    }
}

impl From<InternalDecimal192> for Decimal192 {
    fn from(value: InternalDecimal192) -> Self {
        Self {
            secret_magic: value.to_string(),
        }
    }
}

impl Into<InternalDecimal192> for Decimal192 {
    fn into(self) -> InternalDecimal192 {
        // This is safe because the Decimal192 can be created only InternalDecimal192 which is already valid.
        // Here the conversion back happens.
        self.secret_magic.parse::<InternalDecimal192>().unwrap()
    }
}

/// Tries to creates a new `Decimal192` from a String, throws a `CommonError`
/// if the `string` was not a valid Decimal192.
#[uniffi::export]
pub fn new_decimal_from_string(string: String) -> Result<Decimal192> {
    string.parse::<InternalDecimal192>().into_result()
}

/// Tries to creates a new `Decimal192` from a formatted String for
/// a specific locale.
#[uniffi::export]
pub fn new_decimal_from_formatted_string(
    formatted_string: String,
    locale: LocaleConfig,
) -> Result<Decimal192> {
    InternalDecimal192::new_with_formatted_string(
        formatted_string,
        locale.into(),
    )
    .into_result()
}

/// The standard transaction fee
#[uniffi::export]
pub fn transaction_fee_preset() -> Decimal192 {
    InternalDecimal192::transaction_fee_preset().into()
}

/// Creates a new `Decimal192` from a u32 integer.
#[uniffi::export]
pub fn new_decimal_from_u32(value: u32) -> Decimal192 {
    InternalDecimal192::from(value).into()
}

/// Creates a new `Decimal192` from a u64 integer.
#[uniffi::export]
pub fn new_decimal_from_u64(value: u64) -> Decimal192 {
    InternalDecimal192::from(value).into()
}

/// Creates a new `Decimal192` from a i32 integer.
#[uniffi::export]
pub fn new_decimal_from_i32(value: i32) -> Decimal192 {
    InternalDecimal192::from(value).into()
}

/// Creates a new `Decimal192` from a i64 integer.
#[uniffi::export]
pub fn new_decimal_from_i64(value: i64) -> Decimal192 {
    InternalDecimal192::from(value).into()
}

#[uniffi::export]
pub fn decimal_formatted(
    decimal: &Decimal192,
    locale: LocaleConfig,
    total_places: u8,
    use_grouping_separator: bool,
) -> String {
    decimal.into_internal().formatted(
        locale.into(),
        total_places,
        use_grouping_separator,
    )
}

/// A human readable, locale respecting string. Does not perform any rounding or truncation.
#[uniffi::export]
pub fn decimal_formatted_plain(
    decimal: &Decimal192,
    locale: LocaleConfig,
    use_grouping_separator: bool,
) -> String {
    decimal
        .into_internal()
        .formatted_plain(locale.into(), use_grouping_separator)
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
    InternalDecimal192::from(value).into()
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
    InternalDecimal192::try_from(value).into_result()
}

/// The minimum possible value of `Decimal192`, being:
/// `-3138550867693340381917894711603833208051.177722232017256448`
#[uniffi::export]
pub fn decimal_min() -> Decimal192 {
    InternalDecimal192::min().into()
}

/// The maximum possible value of `Decimal192`, being:
/// `3138550867693340381917894711603833208051.177722232017256447`
#[uniffi::export]
pub fn decimal_max() -> Decimal192 {
    InternalDecimal192::max().into()
}

/// Creates the Decimal192 `10^exponent`
#[uniffi::export]
pub fn new_decimal_exponent(exponent: u8) -> Decimal192 {
    InternalDecimal192::pow(exponent).into()
}

/// `decimal.to_string()`
#[uniffi::export]
pub fn decimal_to_string(decimal: &Decimal192) -> String {
    decimal.into_internal().to_string()
}

/// `lhs < rhs`
#[uniffi::export]
pub fn decimal_less_than(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs.into_internal() < rhs.into_internal()
}

/// `lhs <= rhs`
#[uniffi::export]
pub fn decimal_less_than_or_equal(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs.into_internal() <= rhs.into_internal()
}

/// `lhs > rhs`
#[uniffi::export]
pub fn decimal_greater_than(lhs: &Decimal192, rhs: &Decimal192) -> bool {
    lhs.into_internal() > rhs.into_internal()
}

/// `lhs >= rhs`
#[uniffi::export]
pub fn decimal_greater_than_or_equal(
    lhs: &Decimal192,
    rhs: &Decimal192,
) -> bool {
    lhs.into_internal() >= rhs.into_internal()
}

/// Whether this decimal is zero.
#[uniffi::export]
pub fn decimal_is_zero(decimal: &Decimal192) -> bool {
    decimal.into_internal().is_zero()
}

/// Whether this decimal is positive.
#[uniffi::export]
pub fn decimal_is_positive(decimal: &Decimal192) -> bool {
    decimal.into_internal().is_positive()
}

/// Whether this decimal is negative.
#[uniffi::export]
pub fn decimal_is_negative(decimal: &Decimal192) -> bool {
    decimal.into_internal().is_negative()
}

/// `lhs + rhs``
#[uniffi::export]
pub fn decimal_add(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    (lhs.into_internal() + rhs.into_internal()).into()
}

/// `lhs - rhs``
#[uniffi::export]
pub fn decimal_sub(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    (lhs.into_internal() - rhs.into_internal()).into()
}

/// `lhs * rhs``
#[uniffi::export]
pub fn decimal_mul(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    (lhs.into_internal() * rhs.into_internal()).into()
}

/// `lhs / rhs``
#[uniffi::export]
pub fn decimal_div(lhs: Decimal192, rhs: Decimal192) -> Decimal192 {
    (lhs.into_internal() / rhs.into_internal()).into()
}

/// Negates the `decimal`
#[uniffi::export]
pub fn decimal_neg(decimal: &Decimal192) -> Decimal192 {
    decimal.into_internal().neg().into()
}

/// Returns `decimal.abs()`, panics if `decimal` is `Decimal192::MIN`
#[uniffi::export]
pub fn decimal_abs(decimal: &Decimal192) -> Decimal192 {
    decimal.into_internal().abs().into()
}

/// Clamps `decimal` to zero, i.e. `max(decimal, 0)`
#[uniffi::export]
pub fn decimal_clamped_to_zero(decimal: &Decimal192) -> Decimal192 {
    decimal.into_internal().clamped_to_zero().into()
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
    decimal
        .into_internal()
        .round_with_mode(decimal_places, rounding_mode.into())
        .into_result()
}
