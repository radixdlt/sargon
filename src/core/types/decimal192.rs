use crate::prelude::*;
use delegate::delegate;
use radix_engine_common::math::{
    Decimal as ScryptoDecimal192, RoundingMode as ScryptoRoundingMode,
};

/// UniFFI conversion for InnerDecimal using String as builtin.
impl crate::UniffiCustomTypeConverter for ScryptoDecimal192 {
    type Builtin = String;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        val.parse::<Self>().map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

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
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.native())]
pub struct Decimal192 {
    secret_magic: ScryptoDecimal192, // Do NOT doc! breaks build script
}

/// Internally (in Rust land) we would like to call `Decimal192` just `Decimal`.
/// Reusing the naming convention set by Scrypto.
pub type Decimal = Decimal192;

impl From<Decimal> for ScryptoDecimal192 {
    fn from(value: Decimal) -> Self {
        value.native()
    }
}
impl From<ScryptoDecimal192> for Decimal {
    fn from(value: ScryptoDecimal192) -> Self {
        Self::from_native(value)
    }
}
impl Decimal {
    fn native(&self) -> ScryptoDecimal192 {
        self.secret_magic
    }

    fn from_native(decimal: ScryptoDecimal192) -> Self {
        Decimal {
            secret_magic: decimal,
        }
    }
}

impl FromStr for Decimal192 {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<ScryptoDecimal192>()
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

macro_rules! forward_from_for_num {
    ($num_type: ty) => {
        impl From<$num_type> for Decimal {
            fn from(value: $num_type) -> Self {
                ScryptoDecimal192::from(value).into()
            }
        }
    };
}
forward_from_for_num!(u32);
forward_from_for_num!(u64);
forward_from_for_num!(i32);
forward_from_for_num!(i64);

impl TryFrom<f32> for Decimal {
    type Error = crate::CommonError;

    /// Creates a new `Decimal192` from a f32 float. Will
    /// fail if the f32 cannot be losslessly represented
    /// by the underlying Decimal from Scrypto.
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// assert!(Decimal::try_from(208050.17).is_ok());
    ///
    /// assert_eq!(
    ///     Decimal::try_from(f32::MIN_POSITIVE),
    ///     Err(CommonError::DecimalOverflow { bad_value: f32::MIN_POSITIVE.to_string() })
    /// );
    /// ```
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let str_value = value.to_string();

        str_value
            .parse::<Self>()
            .map_err(|_| CommonError::DecimalOverflow {
                bad_value: str_value,
            })
    }
}

impl Decimal {
    pub const SCALE: u32 = ScryptoDecimal192::SCALE;

    pub fn new(value: String) -> Result<Self> {
        value.parse()
    }

    pub fn zero() -> Self {
        Self::from_native(ScryptoDecimal192::zero())
    }

    pub fn min() -> Self {
        Self::from_native(ScryptoDecimal192::MIN)
    }

    pub fn max() -> Self {
        Self::from_native(ScryptoDecimal192::MAX)
    }

    pub fn one() -> Self {
        Self::from_native(ScryptoDecimal192::one())
    }

    pub fn two() -> Self {
        Self::from_native(ScryptoDecimal192::from(2))
    }

    pub fn three() -> Self {
        Self::from_native(ScryptoDecimal192::from(3))
    }
}

impl Add for Decimal {
    type Output = Self;
    /// self + rhs
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.native() + rhs.native())
    }
}
impl Sub for Decimal {
    type Output = Self;
    /// self - rhs
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.native() - rhs.native())
    }
}
impl Mul for Decimal {
    type Output = Self;
    /// self * rhs
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.native() * rhs.native())
    }
}
impl Div for Decimal {
    type Output = Self;
    /// self / rhs
    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.native() / rhs.native())
    }
}

impl Neg for Decimal {
    type Output = Self;

    /// `-self`
    fn neg(self) -> Self::Output {
        self.native().neg().into()
    }
}

impl Decimal {
    delegate! {
        to self.native() {

            /// Whether this decimal is zero.
            pub fn is_zero(&self) -> bool;

            /// Whether this decimal is positive.
            pub fn is_positive(&self) -> bool;

            /// Whether this decimal is negative.
            pub fn is_negative(&self) -> bool;
        }
    }
}

impl Decimal {
    pub fn checked_powi(&self, exp: i64) -> Option<Self> {
        self.native().checked_powi(exp).map(|n| n.into())
    }

    /// Creates the Decimal `10^exponent`
    pub fn pow(exponent: u8) -> Self {
        Self::from(10)
            .checked_powi(exponent as i64)
            .expect("Too large exponent, 10^39 is max.")
    }

    /// `abs(self)`
    /// Panics if Self is Self::MIN.
    pub fn abs(&self) -> Self {
        self.native().checked_abs().expect("Expected clients of Sargon to not use so large negative numbers (Self::MIN).").into()
    }

    /// `max(self, 0)`, which is often called
    /// "clamping to zero"
    pub fn clamped_to_zero(self) -> Self {
        if self.is_negative() {
            Self::zero()
        } else {
            self
        }
    }

    /// Rounds this number to the specified decimal places.
    ///
    /// # Panics
    /// - Panic if the number of decimal places is not within [0..SCALE(=18)]
    pub fn round(
        &self,
        decimal_places: i32,
        rounding_mode: RoundingMode,
    ) -> Result<Self> {
        self.native()
            .checked_round(decimal_places, rounding_mode.into())
            .ok_or(CommonError::DecimalError)
            .map(Self::from)
    }
}

impl Decimal192 {
    pub const MACHINE_READABLE_DECIMAL_SEPARATOR: &'static str = ".";

    /// Parse a local respecting string
    pub fn new_with_formatted_string(
        formatted_string: impl AsRef<str>,
        locale: LocaleConfig,
    ) -> Result<Self> {
        let formatted_string = formatted_string.as_ref().to_owned();
        // Pad with a leading zero, to make numbers with leading decimal separator parsable
        let mut string = format!("0{}", formatted_string);

        // If the locale recognizes a grouping separator, we strip that from the string
        if let Some(grouping_separator) = locale.grouping_separator {
            string = string.replace(&grouping_separator, "");
        }
        // `num` crate defines some pretty specific grouping separators: `"\u{a0}"` and `"\u{202f}"` for
        // for some locales, but in unit tests we might use _normal_ space (`"U+0020"`), so we remove
        // those (being a bit lenient...).
        string = string.replace(' ', "");

        // If the locale recognizes a decimal separator that is different from the machine readable one, we replace it with that
        if let Some(decimal_separator) = locale.decimal_separator {
            if decimal_separator != Self::MACHINE_READABLE_DECIMAL_SEPARATOR {
                // If `decimal_separator != Self::MACHINE_READABLE_DECIMAL_SEPARATOR`,
                // but if the string contains it, it might have been used incorrectly as
                // a grouping separator. i.e. often "." is used in Swedish as a grouping
                // separator, even though a space is the canonical one. So BEFORE
                // we replace occurrences of decimal separator with "."
                // (`Self::MACHINE_READABLE_DECIMAL_SEPARATOR`), we replace
                // occurrences of `Self::MACHINE_READABLE_DECIMAL_SEPARATOR` with "".
                string = string
                    .replace(Self::MACHINE_READABLE_DECIMAL_SEPARATOR, "");

                string = string.replace(
                    &decimal_separator,
                    Self::MACHINE_READABLE_DECIMAL_SEPARATOR,
                );
            }
        }

        string.parse::<Self>()
    }
}

impl TryInto<Decimal192> for &str {
    type Error = crate::CommonError;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        self.parse::<Decimal>()
    }
}

impl TryFrom<&[u8]> for Decimal192 {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        ScryptoDecimal192::try_from(slice)
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

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
    decimal_places: i32,
    rounding_mode: RoundingMode,
) -> Result<Decimal192> {
    decimal.round(decimal_places, rounding_mode)
}

#[cfg(test)]
mod test_decimal {

    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Decimal;

    #[test]
    fn eq() {
        assert_eq!(Decimal::zero(), Decimal::zero());
        assert_eq!(Decimal::one(), Decimal::one());
        assert_eq!(Decimal::zero(), "0".parse().unwrap());
        assert_eq!(Decimal::one(), "1".parse().unwrap());
    }

    #[test]
    fn scrypto_decimal_roundtrip() {
        let scrypto: ScryptoDecimal192 = "2.718281828".parse().unwrap();
        let sut: SUT = scrypto.into();
        let scrypto_again: ScryptoDecimal192 = sut.into();
        assert_eq!(scrypto, scrypto_again);
    }

    #[test]
    fn inequality() {
        assert_ne!(Decimal192::one(), Decimal192::zero());
    }

    #[test]
    fn is_zero() {
        assert!(Decimal192::zero().is_zero());
        assert!(!Decimal192::one().is_zero());
    }

    #[test]
    fn is_positive() {
        assert!(!Decimal192::zero().is_positive());
        assert!(Decimal192::one().is_positive());
    }

    #[test]
    fn is_negative() {
        assert!("-1".parse::<Decimal>().unwrap().is_negative());
        assert!(!Decimal::zero().is_negative());
        assert!(!Decimal::one().is_negative());
    }

    #[test]
    fn not_less() {
        assert!(Decimal192::zero() >= Decimal192::zero());
        assert!(Decimal192::one() >= Decimal192::one());
        assert!(Decimal192::one() >= Decimal192::zero());
    }

    #[test]
    fn less() {
        assert!(Decimal192::zero() < Decimal192::one());
    }

    #[test]
    fn leq() {
        assert!(Decimal192::zero() <= Decimal192::zero());
        assert!(Decimal192::one() <= Decimal192::one());

        assert!(Decimal192::one() > Decimal192::zero());
    }

    #[test]
    fn not_greater_than() {
        assert!(Decimal192::zero() <= Decimal192::zero());
        assert!(Decimal192::one() <= Decimal192::one());
        assert!(Decimal192::zero() <= Decimal192::one());
    }

    #[test]
    fn geq() {
        assert!(Decimal192::zero() >= Decimal192::zero());
        assert!(Decimal192::one() >= Decimal192::one());

        assert!(Decimal192::zero() < Decimal192::one());
    }

    #[test]
    fn greater() {
        assert!(Decimal192::one() > Decimal192::zero());
    }

    #[test]
    fn add_two_large() {
        let a: Decimal = "958947355801916604025588861116008628224.01234"
            .parse()
            .unwrap();
        let b: Decimal = "58947355801916604025588861116008628224.04321"
            .parse()
            .unwrap();
        let c: Decimal = "1017894711603833208051177722232017256448.05555"
            .parse()
            .unwrap();
        assert_eq!(a + b, c);
    }

    #[test]
    fn from_str() {
        let a: Decimal =
            "3138550867693340381917894711603833208051.177722232017256447"
                .parse()
                .unwrap();
        let b: Decimal =
            "3036550867693340381917894711603833208050.177722232017256447"
                .parse()
                .unwrap();
        assert!(a > b);
    }

    #[test]
    fn try_from_invalid_str() {
        assert_eq!("foobar".parse::<Decimal>(), Err(CommonError::DecimalError));
    }

    #[test]
    fn try_from_invalid_bytes() {
        assert_eq!(
            Decimal192::try_from(generate_32_bytes().as_slice()),
            Err(CommonError::DecimalError)
        );
    }

    #[test]
    fn try_from_valid_bytes() {
        assert!(Decimal192::try_from(generate_bytes::<24>().as_slice()).is_ok());
    }

    #[test]
    fn display() {
        let s = "3138550867693340381917894711603833208051.177722232017256447";
        let a: Decimal192 = s.try_into().unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: Decimal192 =
            "3138550867693340381917894711603833208051.177722232017256447"
                .try_into()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!(
                "3138550867693340381917894711603833208051.177722232017256447"
            ),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("3.1415"));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                Decimal192::try_from(generate_bytes::<24>().as_slice()).unwrap()
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn many_zeros() {
        let s = "0.000000000000000123";
        let d: Decimal = s.parse().unwrap();
        assert_eq!(d.to_string(), s);
    }

    #[test]
    fn arithmetic() {
        assert_eq!(
            SUT::two() + SUT::three(),
            SUT::two() * SUT::three() - SUT::one()
        );
    }

    #[test]
    fn neg() {
        assert_eq!(SUT::two() - SUT::three(), -SUT::one());
    }

    #[test]
    fn from_negative_string() {
        let sut: SUT = "-3.2".parse().unwrap();
        assert_eq!(sut * sut, "10.24".parse().unwrap());
    }

    #[test]
    fn test_parse_formatted_decimal() {
        let test = |s: &str, l: &LocaleConfig, exp: &str| {
            assert_eq!(
                Decimal192::new_with_formatted_string(s, l.clone()).unwrap(),
                exp.parse::<Decimal192>().unwrap()
            )
        };
        let fail = |s: &str, l: &LocaleConfig| {
            assert!(Decimal192::new_with_formatted_string(s, l.clone()).is_err())
        };
        let swedish = LocaleConfig::swedish();
        let us = LocaleConfig::us();
        test(",005", &swedish, "0.005");
        test(".005", &us, "0.005");
        test("1,001", &swedish, "1.001");
        test("1,001", &us, "1001");
        test("1\u{a0}001,45", &swedish, "1001.45");
        test("1 001,45", &swedish, "1001.45");
        test("1.001,45", &swedish, "1001.45");
        test("1.001,45", &us, "1.00145");

        fail("1.000.000", &us);
        test("1.000.000", &swedish, "1000000");

        fail("1.000.000,23", &us);
        test("1.000.000,23", &swedish, "1000000.23");

        test("1 000 000,23", &us, "100000023");
        test("1 000 000,23", &swedish, "1000000.23");

        test("1 000 000.23", &us, "1000000.23");
        test("1 000 000.23", &swedish, "100000023");

        fail("1,000,000", &swedish);
        test("1,000,000", &us, "1000000");

        fail("1,000,000.23", &swedish);
        test("1,000,000.23", &us, "1000000.23");
    }

    #[test]
    fn scale_is_18() {
        assert_eq!(SUT::SCALE, 18);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

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
        let zero = new_decimal_from_i32(0);
        let one = new_decimal_from_i64(1);
        let two = new_decimal_from_u32(2);
        let three = new_decimal_from_u64(3);
        let four = new_decimal_from_string("4".to_string()).unwrap();
        let five = new_decimal_from_i32(5);
        let six = new_decimal_from_i32(6);
        let seven = new_decimal_from_i32(7);
        let eight = new_decimal_from_i32(8);
        let nine = new_decimal_from_i32(9);
        let ten = new_decimal_from_i32(10);

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
    #[should_panic(
        expected = "Expected clients of Sargon to not use so large negative numbers (Self::MIN)."
    )]
    fn decimal_min_abs() {
        _ = SUT::min().abs()
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
    fn from_f32() {
        let f: f32 = 208050.17;
        assert_eq!(f.to_string(), "208050.17");
        let sut = new_decimal_from_f32(f);
        assert_eq!(sut.unwrap().to_string(), "208050.17");
        assert_eq!(
            SUT::try_from(f32::MAX).unwrap().to_string(),
            "340282350000000000000000000000000000000"
        );
        assert_eq!(
            SUT::try_from(f32::MIN_POSITIVE),
            Err(CommonError::DecimalOverflow {
                bad_value: f32::MIN_POSITIVE.to_string()
            })
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
            .round(0, RoundingMode::ToNearestMidpointAwayFromZero)
            .is_ok());
        assert!(max
            .round(0, RoundingMode::ToNearestMidpointTowardZero)
            .is_ok());
        assert!(max.round(0, RoundingMode::ToZero).is_ok());

        assert!(max
            .round(18, RoundingMode::ToNearestMidpointAwayFromZero)
            .is_ok());
        assert!(max
            .round(18, RoundingMode::ToNearestMidpointTowardZero)
            .is_ok());
        assert!(max.round(18, RoundingMode::ToZero).is_ok());

        assert!(max.round(0, RoundingMode::AwayFromZero).is_err());
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
        let swedish = LocaleConfig::swedish();
        let us = LocaleConfig::us();
        test(",005", &swedish, "0.005");
        test(".005", &us, "0.005");
        test("1,001", &swedish, "1.001");
        test("1,001", &us, "1001");

        fail("1,000,000.23", &swedish);
        test("1,000,000.23", &us, "1000000.23");
    }
}
