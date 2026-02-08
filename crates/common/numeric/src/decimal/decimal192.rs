use core::panic;

use crate::prelude::*;
use delegate::delegate;
use enum_iterator::reverse_all;
use radix_common::math::ParseDecimalError;

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
)]
#[display("{}", self.native())]
pub struct Decimal192(ScryptoDecimal192);

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
        self.0
    }

    pub const fn from_native(decimal: ScryptoDecimal192) -> Self {
        Self(decimal)
    }

    pub fn transaction_fee_preset() -> Self {
        25.into()
    }
}

impl Decimal192 {
    /// Will lose precision if the fractional part has more than 18 digits!
    fn from_str_by_truncating(s: impl AsRef<str>) -> Result<Self> {
        let str_value = s.as_ref();
        let parts: Vec<&str> = str_value.split('.').collect();
        let fractional_part = parts[1];
        let processed_s = format!("{}.{}", parts[0], &fractional_part[..18]);

        processed_s
            .parse::<ScryptoDecimal192>()
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }

    pub fn from_str_value(s: impl AsRef<str>) -> Result<Self> {
        let str_value = s.as_ref();
        match str_value.parse::<ScryptoDecimal192>() {
            Ok(decimal) => Ok(Self::from_native(decimal)),
            Err(ParseDecimalError::MoreThanEighteenDecimalPlaces) => {
                Self::from_str_by_truncating(str_value)
            }
            Err(_) => Err(CommonError::DecimalError),
        }
    }
}

impl FromStr for Decimal192 {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_value(s)
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

impl From<f32> for Decimal {
    /// Creates a new `Decimal192` from a f32 float. Will
    /// lose precision if the f32 cannot be losslessly
    /// represented by the underlying Decimal from Scrypto
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert!(Decimal::from(208050.17).to_string() == "208050.17");
    ///
    /// assert!(Decimal::from(f32::MIN_POSITIVE).to_string() == "0");
    ///
    /// ```
    fn from(value: f32) -> Self {
        let str_value = value.to_string();
        Self::from_str_value(str_value).unwrap()
    }
}

impl TryFrom<f64> for Decimal {
    type Error = crate::CommonError;

    /// Creates a new `Decimal192` from a f64 float. Will
    /// fail if the f64 cannot be losslessly represented
    /// by the underlying Decimal from Scrypto.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert!(Decimal::try_from(208050.17).is_ok());
    ///
    /// assert!(Decimal::try_from(f64::MIN_POSITIVE).is_ok());
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let str_value = value.to_string();
        Self::from_str_value(str_value)
    }
}

impl Decimal {
    pub const SCALE: u8 = ScryptoDecimal192::SCALE as u8;
    pub const MAX_PLACES_ENGINEERING_NOTATION: u8 = 4;

    pub fn new(value: String) -> Result<Self> {
        value.parse()
    }

    /// The number `0` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::zero().to_string(), "0");
    /// ```
    ///
    pub fn zero() -> Self {
        Self::from_native(ScryptoDecimal192::zero())
    }

    /// The minimum possible value of `Decimal192`:
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::min().to_string(), "-3138550867693340381917894711603833208051.177722232017256448");
    /// ```
    ///
    pub fn min() -> Self {
        Self::from_native(ScryptoDecimal192::MIN)
    }

    /// The maximum possible value of `Decimal192`
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::max().to_string(), "3138550867693340381917894711603833208051.177722232017256447");
    /// ```
    ///
    pub fn max() -> Self {
        Self::from_native(ScryptoDecimal192::MAX)
    }

    /// The number `1` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::one().to_string(), "1");
    /// ```
    ///
    pub fn one() -> Self {
        Self::from_native(ScryptoDecimal192::one())
    }

    /// The number `2` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::two().to_string(), "2");
    /// ```
    ///
    pub fn two() -> Self {
        Self::from_native(ScryptoDecimal192::from(2))
    }

    /// The number `3` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::three().to_string(), "3");
    /// ```
    ///
    pub fn three() -> Self {
        Self::from_native(ScryptoDecimal192::from(3))
    }

    /// The number `4` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::four().to_string(), "4");
    /// ```
    ///
    pub fn four() -> Self {
        Self::from_native(ScryptoDecimal192::from(4))
    }

    /// The number `5` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::five().to_string(), "5");
    /// ```
    ///
    pub fn five() -> Self {
        Self::from_native(ScryptoDecimal192::from(5))
    }

    /// The number `6` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::six().to_string(), "6");
    /// ```
    ///
    pub fn six() -> Self {
        Self::from_native(ScryptoDecimal192::from(6))
    }

    /// The number `7` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::seven().to_string(), "7");
    /// ```
    ///
    pub fn seven() -> Self {
        Self::from_native(ScryptoDecimal192::from(7))
    }

    /// The number `8` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::eight().to_string(), "8");
    /// ```
    ///
    pub fn eight() -> Self {
        Self::from_native(ScryptoDecimal192::from(8))
    }

    /// The number `9` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::nine().to_string(), "9");
    /// ```
    ///
    pub fn nine() -> Self {
        Self::from_native(ScryptoDecimal192::from(9))
    }

    /// The number `10` as a `Decimal192`.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal192::ten().to_string(), "10");
    /// ```
    ///
    pub fn ten() -> Self {
        Self::from_native(ScryptoDecimal192::from(10))
    }
}

impl Add for Decimal {
    type Output = Self;
    /// Addition: `self + rhs`
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(Decimal::one().add(Decimal::two()), Decimal::three());
    /// ```
    ///
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.native() + rhs.native())
    }
}
impl Sub for Decimal {
    type Output = Self;
    /// Subtraction: `self - rhs`
    ///
    /// ```
    /// use numeric::prelude::*;
    /// type SUT = Decimal;
    ///
    /// assert_eq!(SUT::three().sub(SUT::two()), SUT::one());
    /// ```
    ///
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.native() - rhs.native())
    }
}
impl Mul for Decimal {
    type Output = Self;
    /// Multiplication: `self * rhs`
    ///
    /// ```
    /// use numeric::prelude::*;
    /// type SUT = Decimal;
    ///
    /// assert_eq!(SUT::two().mul(SUT::three()), SUT::six());
    /// ```
    ///
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.native() * rhs.native())
    }
}
impl ScryptoCheckedMul for Decimal {
    type Output = Self;

    #[inline]
    fn checked_mul(self, other: Self) -> Option<Self> {
        self.native().checked_mul(other.native()).map(Self::from)
    }
}
impl Div for Decimal {
    type Output = Self;
    /// Division: `self / rhs`
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal;
    ///
    /// assert_eq!(SUT::eight().div(SUT::four()), SUT::two());
    /// ```
    ///
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.native() / rhs.native())
    }
}

impl Neg for Decimal {
    type Output = Self;

    /// Negates `self`
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal;
    ///
    /// assert_eq!(SUT::five().neg().to_string(), "-5");
    /// ```
    ///
    #[inline]
    fn neg(self) -> Self::Output {
        self.native().neg().into()
    }
}

impl Decimal {
    delegate! {
            to self.native() {

                /// Whether this decimal is zero
                ///
                /// ```
    /// use numeric::prelude::*;
                /// #[allow(clippy::upper_case_acronyms)]
                /// type SUT = Decimal;
                ///
                /// assert!(SUT::zero().is_zero());
                /// assert!(!SUT::one().is_zero());
                /// assert!(!SUT::one().neg().is_zero());
                /// ```
                ///
                pub fn is_zero(&self) -> bool;

                /// Whether this decimal is positive.
                ///
                /// ```
    /// use numeric::prelude::*;
                /// #[allow(clippy::upper_case_acronyms)]
                /// type SUT = Decimal;
                ///
                /// assert!(SUT::one().is_positive());
                /// assert!(!SUT::zero().is_positive());
                /// assert!(!SUT::one().neg().is_positive());
                /// ```
                ///
                pub fn is_positive(&self) -> bool;

                /// Whether this decimal is negative.
                ///
                /// ```
    /// use numeric::prelude::*;
                /// #[allow(clippy::upper_case_acronyms)]
                /// type SUT = Decimal;
                ///
                /// assert!(SUT::one().neg().is_negative());
                /// assert!(!SUT::one().is_negative());
                /// assert!(!SUT::zero().is_negative());
                /// ```
                ///
                pub fn is_negative(&self) -> bool;
            }
        }
}

impl Decimal {
    /// Creates the Decimal `10^exponent`, returns `None` if overflows.
    #[inline]
    pub(crate) fn checked_powi(&self, exp: i64) -> Option<Self> {
        self.native().checked_powi(exp).map(|n| n.into())
    }

    /// Creates the Decimal `10^exponent`
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// assert_eq!(SUT::pow(2).to_string(), "100");
    /// assert_eq!(SUT::pow(3).to_string(), "1000");
    /// ```
    ///
    #[inline]
    pub fn pow(exponent: u8) -> Self {
        Self::from(10)
            .checked_powi(exponent as i64)
            .expect("Too large exponent, 10^39 is max.")
    }

    /// Returns the absolute value, if `self` is `Decimal::min()` then `Decimal::max()`
    /// is returned, since `Decimal::min().abs()` would overflow.
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// assert_eq!(SUT::two().neg().abs(), SUT::two());
    /// assert_eq!(SUT::three().abs(), SUT::three());
    /// assert_eq!(SUT::max().abs(), SUT::max());
    /// ```
    ///
    #[inline]
    pub fn abs(&self) -> Self {
        if self == &Self::min() {
            Self::max()
        } else {
            self.native()
                .checked_abs()
                .expect("Should never fail")
                .into()
        }
    }

    /// `max(self, 0)`, which is often called "clamping to zero"
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// assert_eq!(SUT::one().neg().clamped_to_zero(), SUT::zero());
    /// assert_eq!(SUT::two().clamped_to_zero(), SUT::two());
    /// ```
    ///
    pub fn clamped_to_zero(self) -> Self {
        if self.is_negative() {
            Self::zero()
        } else {
            self
        }
    }

    /// Rounds this number to the specified decimal places, or if
    /// None, rounds to `Decimal192::SCALE` places, using the
    /// rounding mode `ToNearestMidpointAwayFromZero`.
    ///
    /// Note:
    /// Rounding with mode `ToNearestMidpointAwayFromZero` will fail for `Decimal192::max()`,
    /// and in fact it will fail for numbers very close to `Decimal192::max()`.
    /// The max value is a "integer_part_of_max" followed by the decimal part:
    /// `.177722232017256447`
    /// If the rounding fails using `ToNearestMidpointAwayFromZero`, we fallback
    /// to rounding using `ToZero` which never fails.
    ///
    /// ```
    /// use numeric::prelude::*;
    ///
    /// assert_eq!(
    ///     "3138550867693340381917894711603833208051.149".parse::<Decimal192>().unwrap().round(2),
    ///     "3138550867693340381917894711603833208051.15".parse::<Decimal192>().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     "3138550867693340381917894711603833208051.1499".parse::<Decimal192>().unwrap().round(3),
    ///     "3138550867693340381917894711603833208051.15".parse::<Decimal192>().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     "3138550867693340381917894711603833208051.1499".parse::<Decimal192>().unwrap().round(2),
    ///     "3138550867693340381917894711603833208051.15".parse::<Decimal192>().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     "3138550867693340381917894711603833208051.151".parse::<Decimal192>().unwrap().round(2),
    ///     "3138550867693340381917894711603833208051.15".parse::<Decimal192>().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     "3138550867693340381917894711603833208051.1519".parse::<Decimal192>().unwrap().round(3),
    ///     "3138550867693340381917894711603833208051.152".parse::<Decimal192>().unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     Decimal192::max().round(12),
    ///     "3138550867693340381917894711603833208051.177722232017".parse::<Decimal192>().unwrap()
    /// );
    /// assert_eq!(
    ///     Decimal192::max().round(Decimal192::SCALE),
    ///     Decimal192::max()
    /// );
    /// assert_eq!(
    ///     Decimal192::max().round(3),
    ///     "3138550867693340381917894711603833208051.177".parse::<Decimal192>().unwrap()
    /// );
    /// ```
    ///
    /// Why not ALWAYS use `ToNearestMidpointAwayFromZero`? Well maybe we should!
    /// But in this initial release of Sargon I wanted to be using a rounding
    /// behavior as close as wallets use today (at least iOS uses today).
    ///
    /// # Panics
    /// - Panic if the number of decimal places is not within [0..SCALE(=18)]
    pub fn round(&self, decimal_places: impl Into<Option<u8>>) -> Self {
        let decimal_places = decimal_places.into().unwrap_or(Decimal192::SCALE);

        self.round_with_mode(
            decimal_places,
            RoundingMode::ToNearestMidpointAwayFromZero,
        )
        .unwrap_or(
            self.round_with_mode(decimal_places, RoundingMode::ToZero)
                .unwrap_or_else(|_| unreachable!()),
        )
    }

    /// Rounds this number to the specified decimal places.
    ///
    /// # Panics
    /// - Panic if the number of decimal places is not within [0..SCALE(=18)]
    pub fn round_with_mode(
        &self,
        decimal_places: u8,
        rounding_mode: RoundingMode,
    ) -> Result<Self> {
        self.native()
            .checked_round(decimal_places as i32, rounding_mode.into())
            .ok_or(CommonError::DecimalError)
            .map(Self::from)
    }
}

impl HasSampleValues for Decimal192 {
    fn sample() -> Self {
        "123.456".parse().unwrap()
    }

    fn sample_other() -> Self {
        "789.012".parse().unwrap()
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
        // Some locals use pretty specific grouping separators: `"\u{a0}"` and `"\u{202f}"`,
        // but in unit tests we might use _normal_ space (`"U+0020"`), so we remove
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

/// Million, Billion or Trillion, helper for Decimal192 formatting.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
)]
#[repr(u8)]
pub(crate) enum Multiplier {
    Million = 6,
    Billion = 9,
    Trillion = 12,
}

impl Multiplier {
    /// The exponent of this multiplier
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    /// The exponent of a `Multiplier`, i.e. `6` for `Million`.
    pub(crate) fn value(&self) -> Decimal192 {
        Decimal192::pow(self.discriminant())
    }

    /// Symbol of a `Multiplier`, i.e. 'M' for `Million`.
    pub(crate) fn suffix(&self) -> char {
        match self {
            Self::Million => 'M',
            Self::Billion => 'B',
            Self::Trillion => 'T',
        }
        .to_owned()
    }
}

/// Format decimal helper - counts '0' at end of `s`.
fn trailing_zero_count_of(s: impl AsRef<str>) -> usize {
    let str = s.as_ref();
    str.chars()
        .rev()
        .enumerate()
        .find(|x| x.1 != '0')
        .map(|x| x.0)
        .unwrap_or(str.len())
}

/// Format decimal helper - inserts `separator` at every `3` places.
fn insert_grouping_separator_into(s: &mut String, separator: String) {
    let digits = s.len();
    let zeroes_per_thousand = 3;
    if digits <= zeroes_per_thousand {
        return;
    }
    let number_of_separators_to_insert = (digits - 1) / zeroes_per_thousand;
    if number_of_separators_to_insert == 0 {
        return;
    }
    for i in 1..=number_of_separators_to_insert {
        let idx = digits - zeroes_per_thousand * i;
        s.insert_str(idx, &separator);
    }
}

/// Format decimal helper - splits string after `after` if pos, else at 0.
fn split_str(s: impl AsRef<str>, after: i8) -> (String, String) {
    let mut s = s.as_ref().to_owned();
    if after <= 0 {
        ("".to_owned(), s)
    } else {
        (s.drain(0..after as usize).collect(), s)
    }
}

// Helper for formatting
impl Decimal192 {
    pub(crate) fn multiplier(&self) -> Option<Multiplier> {
        let abs = self.abs();
        reverse_all::<Multiplier>().find(|x| x.value() <= abs)
    }

    /// The digits of the number, without separators or sign. The scale is fixed at 18, meaning the last 18 digits correspond to the decimal part.
    pub fn digits(&self) -> String {
        self.abs().0.attos().to_string() // mantissa
    }

    /// Rounds `self`` to `n` places, counting both the integer and decimal parts,
    /// as well as any leading zeros.
    pub(crate) fn rounded_to_total_places(&self, n: u8) -> Self {
        let total_places = n;
        let digits = self.digits();
        // If we only have decimals, we will still count the 0 before the separator as an integer
        let integer_count =
            std::cmp::max(digits.len() as i8 - Self::SCALE as i8, 1) as u8;

        if integer_count > total_places {
            let scale = Self::pow(integer_count - total_places);
            let base = *self / scale;
            let base_rounded = base.round(0);

            if let Some(val) = base_rounded.checked_mul(scale) {
                val
            } else {
                let base_rounded_safe = base
                    .round_with_mode(0, RoundingMode::ToZero)
                    .expect("Rounding to Zero should never fail.");
                base_rounded_safe * scale
            }
        } else {
            // The remaining digits are decimals and we keep up to totalPlaces of them
            let decimals_to_keep = total_places - integer_count;
            self.round(decimals_to_keep)
        }
    }
}

impl Decimal192 {
    /// A human readable, locale respecting string. Does not perform any rounding or truncation.
    pub fn formatted_plain(
        &self,
        locale: LocaleConfig,
        use_grouping_separator: bool,
    ) -> String {
        if self.is_zero() {
            return 0.to_string();
        }

        let sign = if self.is_negative() { "-" } else { "" };
        let decimal_separator =
            locale.decimal_separator.unwrap_or(".".to_owned());
        let digits = self.digits();
        let integer_count = digits.len() as i8 - Self::SCALE as i8;

        let trailing_zero_count = trailing_zero_count_of(digits.clone());

        let (mut integer_part, mut decimal_part) =
            split_str(digits, integer_count);

        if integer_count <= 0 {
            // If we don't have any integers, we just use "0"
            integer_part = "0".to_owned();
        } else if use_grouping_separator {
            if let Some(grouping_separator) = locale.grouping_separator {
                insert_grouping_separator_into(
                    &mut integer_part,
                    grouping_separator,
                )
            }
        }
        if trailing_zero_count >= Self::SCALE as usize {
            // No non-zero decimals, we only have an integer part
            format!("{}{}", sign, integer_part)
        } else {
            let zeros_to_pad = std::cmp::max(-integer_count, 0) as usize;
            let zeroes = "0".repeat(zeros_to_pad);
            decimal_part = decimal_part
                .drain(0..decimal_part.len() - trailing_zero_count)
                .collect();

            format!(
                "{}{}{}{}{}",
                sign, integer_part, decimal_separator, zeroes, decimal_part
            )
        }
    }

    /// Formats decimal using engineering notation: `5e20`.
    ///
    /// If no `None` is passed to `total_places`, then
    /// `Self::MAX_PLACES_ENGINEERING_NOTATION` (4) will
    /// be used.
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// assert_eq!(SUT::max().formatted_engineering_notation(LocaleConfig::default(), None), "3.138e39");
    /// assert_eq!(SUT::min().formatted_engineering_notation(LocaleConfig::default(), None), "-3.138e39");
    /// assert_eq!(SUT::MAX_PLACES_ENGINEERING_NOTATION, 4);
    /// ```
    ///
    /// # Panics
    /// Panics if `self.abs()` is smaller than 1
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// // Positive small
    /// let mut result = std::panic::catch_unwind(|| SUT::try_from(0.9).unwrap().formatted_engineering_notation(LocaleConfig::default(), None));
    /// assert!(result.is_err());
    ///
    /// // Negative small
    /// result = std::panic::catch_unwind(|| SUT::try_from(-0.9).unwrap().formatted_engineering_notation(LocaleConfig::default(), None));
    /// assert!(result.is_err());
    /// ```
    ///
    pub fn formatted_engineering_notation(
        &self,
        locale: LocaleConfig,
        total_places: impl Into<Option<u8>>,
    ) -> String {
        if self.abs() < Self::one() {
            panic!("You MUST NOT use 'formatted_engineering_notation' on a number which abs() is smaller than 1.");
        }

        let total_places = total_places
            .into()
            .unwrap_or(Self::MAX_PLACES_ENGINEERING_NOTATION);
        let rounded = self.rounded_to_total_places(total_places);
        let digit_count = rounded.digits().len() as u8;

        let integer_count = digit_count - Self::SCALE;
        let exponent = integer_count - 1;
        let scaled = rounded / Self::pow(exponent);
        format!("{}e{}", scaled.formatted_plain(locale, false), exponent)
    }

    /// A human readable, locale respecting string, rounded to `total_places`
    /// places, counting all digits.
    ///
    /// ```
    /// use numeric::prelude::*;
    /// #[allow(clippy::upper_case_acronyms)]
    /// type SUT = Decimal192;
    ///
    /// assert_eq!("12345678.975".parse::<SUT>().unwrap().formatted(LocaleConfig::default(), 8, true), "12.345679 M");
    /// ```
    ///
    pub fn formatted(
        &self,
        locale: LocaleConfig,
        total_places: u8,
        use_grouping_separator: bool,
    ) -> String {
        let format = |number: Self| {
            number.formatted_plain(locale.clone(), use_grouping_separator)
        };
        let rounded_to_total_places =
            self.rounded_to_total_places(total_places);

        if let Some(multiplier) = rounded_to_total_places.multiplier() {
            let scaled = rounded_to_total_places / multiplier.value();
            let integer_count = scaled.digits().len() as u8 - Self::SCALE;
            if integer_count > total_places {
                self.formatted_engineering_notation(
                    locale,
                    Self::MAX_PLACES_ENGINEERING_NOTATION,
                )
            } else {
                format!("{} {}", format(scaled), multiplier.suffix())
            }
        } else {
            format(rounded_to_total_places)
        }
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

#[cfg(test)]
mod test_decimal {

    use super::*;

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

    #[should_panic(
        expected = "You MUST NOT use 'formatted_engineering_notation' on a number which abs() is smaller than 1."
    )]
    #[test]
    fn engineering_for_abs_less_than_1_fails_pos() {
        _ = SUT::from(0.9f32).formatted_engineering_notation(
            LocaleConfig::swedish_sweden(),
            None,
        );
    }

    #[should_panic(
        expected = "You MUST NOT use 'formatted_engineering_notation' on a number which abs() is smaller than 1."
    )]
    #[test]
    fn engineering_for_abs_less_than_1_fails_neg() {
        _ = SUT::from(-0.9f32).formatted_engineering_notation(
            LocaleConfig::swedish_sweden(),
            None,
        );
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
    fn transaction_fee_preset_value() {
        assert_eq!(Decimal192::transaction_fee_preset(), 25.into());
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
        let a: Decimal192 = s.parse().unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: Decimal192 =
            "3138550867693340381917894711603833208051.177722232017256447"
                .parse()
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
    fn from_f64_more_than_18_decimals_is_ok() {
        let test = |f: f64, s: &str| {
            let sut = Decimal192::try_from(f).unwrap();
            assert_eq!(sut.to_string(), s);
        };
        test(0.1, "0.1");
        test(f32::MAX as f64, "340282346638528860000000000000000000000");
        test(f32::MIN as f64, "-340282346638528860000000000000000000000");
        test(123456789.87654321, "123456789.87654321");
        test(4.012_345_678_901_235, "4.012345678901235"); // precision lost
        test(4.012_345_678_901_235, "4.012345678901235"); // Over 18 decimals is OK (precision lost)
    }

    #[test]
    fn from_f32_more_than_18_decimals_is_ok() {
        let test = |f: f32, s: &str| {
            let sut = Decimal192::from(f);
            assert_eq!(sut.to_string(), s);
        };

        test(0.1, "0.1");
        test(f32::MAX, "340282350000000000000000000000000000000");
        test(f32::MIN, "-340282350000000000000000000000000000000");
        test(f32::MIN_POSITIVE, "0");
        test(123456789.87654321, "123456790");
        test(4.012_346, "4.012346"); // precision lost
        test(4.012_346, "4.012346"); // Over 18 decimals is OK (precision lost)
    }

    #[test]
    fn from_str_more_than_18_decimals_is_ok() {
        assert_eq!(
            "4.012345678901234567890"
                .parse::<Decimal192>()
                .unwrap()
                .to_string(),
            "4.012345678901234567"
        ); // Over 18 decimals is OK (precision lost)
    }

    #[test]
    fn from_more_than_one_decimal_point_with_more_than_18_decimals_string() {
        assert_eq!(
            "4.0123456789012345678.123".parse::<Decimal192>(),
            Err(CommonError::DecimalError)
        );
    }

    #[test]
    fn from_negative_string() {
        let sut: SUT = "-3.2".parse().unwrap();
        assert_eq!(sut * sut, "10.24".parse().unwrap());
    }

    #[test]
    fn test_formatted_engineering_notation() {
        let test_ = |x: Decimal, n: u8, expected: &str| {
            let actual = x.formatted_engineering_notation(
                LocaleConfig::english_united_states(),
                n,
            );
            assert_eq!(actual, expected);
        };
        let test = |x: &str, n: u8, expected: &str| {
            test_(x.parse().unwrap(), n, expected)
        };
        test("111222111222111222333.222333", 18, "1.11222111222111222e20");
        test("111222111222111222333.222333", 8, "1.1122211e20");
        test("-1234567890.987654321", 8, "-1.2345679e9");
        test("-1234567890.987654321", 11, "-1.234567891e9");
        test("-1234567890.987654321", 14, "-1.2345678909877e9");
        test_(SUT::max(), SUT::MAX_PLACES_ENGINEERING_NOTATION, "3.138e39");
        test_(
            SUT::min(),
            SUT::MAX_PLACES_ENGINEERING_NOTATION,
            "-3.138e39",
        );
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
        let swedish = LocaleConfig::swedish_sweden();
        let us = LocaleConfig::english_united_states();
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
        test(
            "4.012 345 678 901 234 567 890",
            &us,
            "4.012345678901234567890",
        );
    }

    #[test]
    fn scale_is_18() {
        assert_eq!(SUT::SCALE, 18);
    }

    #[test]
    fn round() {
        let test = |x: SUT, d: u8, y: &str| {
            assert_eq!(x.round(d), y.parse().unwrap());
        };

        let mut x = SUT::max();

        test(
            x,
            18,
            "3138550867693340381917894711603833208051.177722232017256447",
        );
        test(
            x,
            17,
            "3138550867693340381917894711603833208051.17772223201725644",
        );
        test(
            x,
            16,
            "3138550867693340381917894711603833208051.1777222320172564",
        );
        test(
            x,
            15,
            "3138550867693340381917894711603833208051.177722232017256",
        );
        test(
            x,
            14,
            "3138550867693340381917894711603833208051.17772223201725",
        );
        test(
            x,
            13,
            "3138550867693340381917894711603833208051.1777222320172",
        );
        test(
            x,
            12,
            "3138550867693340381917894711603833208051.177722232017",
        );
        test(
            x,
            11,
            "3138550867693340381917894711603833208051.17772223201",
        );
        test(x, 10, "3138550867693340381917894711603833208051.1777222320");
        test(x, 9, "3138550867693340381917894711603833208051.177722232");
        test(x, 8, "3138550867693340381917894711603833208051.17772223");
        test(x, 7, "3138550867693340381917894711603833208051.1777222");
        test(x, 6, "3138550867693340381917894711603833208051.177722");
        test(x, 5, "3138550867693340381917894711603833208051.17772");
        test(x, 4, "3138550867693340381917894711603833208051.1777");
        test(x, 3, "3138550867693340381917894711603833208051.177");
        test(x, 2, "3138550867693340381917894711603833208051.17");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 0, "3138550867693340381917894711603833208051");

        x = "3138550867693340381917894711603833208051.14"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.14");
        test(x, 3, "3138550867693340381917894711603833208051.14");

        x = "3138550867693340381917894711603833208051.148"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.15");
        test(x, 3, "3138550867693340381917894711603833208051.148");
        test(x, 4, "3138550867693340381917894711603833208051.148");

        x = "3138550867693340381917894711603833208051.149"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.15");
        test(x, 3, "3138550867693340381917894711603833208051.149");
        test(x, 4, "3138550867693340381917894711603833208051.149");

        x = "3138550867693340381917894711603833208051.1499"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.15");
        test(x, 3, "3138550867693340381917894711603833208051.15");
        test(x, 4, "3138550867693340381917894711603833208051.1499");
        test(x, 5, "3138550867693340381917894711603833208051.1499");

        x = "3138550867693340381917894711603833208051.15"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.15");
        test(x, 3, "3138550867693340381917894711603833208051.15");
        test(x, 4, "3138550867693340381917894711603833208051.15");
        test(x, 5, "3138550867693340381917894711603833208051.15");

        x = "3138550867693340381917894711603833208051.15999"
            .parse()
            .unwrap();
        test(x, 0, "3138550867693340381917894711603833208051");
        test(x, 1, "3138550867693340381917894711603833208051.1");
        test(x, 2, "3138550867693340381917894711603833208051.16");
        test(x, 3, "3138550867693340381917894711603833208051.16");
        test(x, 4, "3138550867693340381917894711603833208051.16");
        test(x, 5, "3138550867693340381917894711603833208051.15999");
        test(x, 6, "3138550867693340381917894711603833208051.15999");
    }

    /// Low level test, testing helper function used by formatting of decimal
    #[test]
    fn test_insert_grouping_separator_into() {
        let test_w = |s: &str, exp: &str, sep: char| {
            let mut string = s.to_owned();
            insert_grouping_separator_into(&mut string, sep.to_string());
            assert_eq!(string, exp.to_owned())
        };
        let test = |s: &str, exp: &str| test_w(s, exp, ' ');

        test("", "");
        test("1", "1");
        test("22", "22");
        test("333", "333");
        test("4444", "4 444");
        test("123456789", "123 456 789");
        test("12345678987654321", "12 345 678 987 654 321");

        test_w("123456789", "123.456.789", '.');
        test_w("123456789", "123,456,789", ',');
    }

    /// Low level test, testing helper function used by formatting of decimal
    #[test]
    fn test_trailing_zero_count_of() {
        let test =
            |s: &str, exp: usize| assert_eq!(trailing_zero_count_of(s), exp);

        test("", 0);
        test("1", 0);
        test("0", 1);
        test("100", 2);
        test("1001", 0);
        test("90000", 4);
        test("9000.0", 1);
        test("9.000", 3);
    }

    /// Low level test, testing helper function used by formatting of decimal
    #[test]
    fn test_split_str() {
        let test = |s: &str, a: i8, exp: (&str, &str)| {
            let res = split_str(s, a);
            assert_eq!(res.0, exp.0.to_string());
            assert_eq!(res.1, exp.1.to_string());
        };

        test("12345.09876", -2, ("", "12345.09876"));

        test("9.8", 0, ("", "9.8"));
        test("9.8", 1, ("9", ".8"));
        test("9.8", 2, ("9.", "8"));

        test("3.1415", 0, ("", "3.1415"));
        test("3.1415", 1, ("3", ".1415"));
        test("3.1415", 2, ("3.", "1415"));

        test("42.1828", 0, ("", "42.1828"));
        test("42.1828", 1, ("4", "2.1828"));
        test("42.1828", 2, ("42", ".1828"));
        test("42.1828", 3, ("42.", "1828"));
        test("42.1828", 4, ("42.1", "828"));
        test("42.1828", 5, ("42.18", "28"));
        test("42.1828", 6, ("42.182", "8"));
        test("42.1828", 7, ("42.1828", ""));
    }

    /// Low level test, testing helper function used by formatting of decimal
    #[test]
    fn test_digits() {
        let test = |s: &str, e: &str| {
            let x: Decimal192 = s.parse().unwrap();
            assert_eq!(x.digits(), e);
        };
        test("1", "1000000000000000000");
        test("1.2", "1200000000000000000");
        test("123456789.098765432105", "123456789098765432105000000");
        test(
            "123456789.098765432105000098",
            "123456789098765432105000098",
        );
    }

    #[test]
    fn format_grouping_separator() {
        let test = |x: &str, exp: &str| {
            let locale = LocaleConfig::english_united_states();
            let decimal: Decimal192 = x.parse().unwrap();
            let actual = decimal.formatted(locale, 8, true);
            assert_eq!(actual, exp);
        };

        test("123456789", "123.45679 M");
        test("12345678", "12.345678 M");
        test("1234567", "1.234567 M");

        test("123456", "123,456");
        test("12345", "12,345");
        test("1234", "1,234");
        test("123", "123");

        test("123456.4321", "123,456.43");
        test("12345.4321", "12,345.432");
        test("1234.4321", "1,234.4321");
        test("123.4321", "123.4321");
    }

    #[test]
    fn format_decimal() {
        let test_ = |decimal: SUT, exp: &str| {
            let locale = LocaleConfig::english_united_states();
            let actual = decimal.formatted(locale, 8, false);
            assert_eq!(actual, exp);
        };
        let test = |x: &str, exp: &str| test_(x.parse().unwrap(), exp);

        test_(SUT::max(), "3.138e39");
        test("0.009999999999999", "0.01");
        test("12341234", "12.341234 M");
        test("1234123.4", "1.2341234 M");
        test("123456.34", "123456.34");
        test("12345.234", "12345.234");
        test("1234.1234", "1234.1234");
        test("123.41234", "123.41234");
        test("12.341234", "12.341234");
        test("1.2341234", "1.2341234");

        test("0.1234123", "0.1234123");
        test("0.0234123", "0.0234123");
        test("0.0034123", "0.0034123");
        test("0.0004123", "0.0004123");
        test("0.0000123", "0.0000123");
        test("0.0000023", "0.0000023");
        test("0.0000003", "0.0000003");

        test("1234123.44", "1.2341234 M");
        test("123456.344", "123456.34");
        test("12345.2344", "12345.234");
        test("1234.12344", "1234.1234");
        test("123.412344", "123.41234");
        test("12.3412344", "12.341234");
        test("1.23412344", "1.2341234");

        test("0.12341234", "0.1234123");
        test("0.02341234", "0.0234123");
        test("0.00341234", "0.0034123");
        test("0.00041234", "0.0004123");
        test("0.00001234", "0.0000123");
        test("0.00000234", "0.0000023");
        test("0.00000034", "0.0000003");

        test("9999999.99", "10 M");
        test("999999.999", "1 M");
        test("99999.9999", "100000");
        test("9999.99999", "10000");
        test("999.999999", "1000");
        test("99.9999999", "100");
        test("9.99999999", "10");

        test("0.99999999", "1");
        test("0.09999999", "0.1");
        test("0.00999999", "0.01");
        test("0.00099999", "0.001");
        test("0.00009999", "0.0001");
        test("0.00000999", "0.00001");
        test("0.00000099", "0.000001");
        test("0.00000009", "0.0000001");

        test("0.000000009", "0");

        test("12.3456789", "12.345679");

        test("0.123456789", "0.1234568");
        test("0.4321", "0.4321");
        test("0.0000000000001", "0");
        test("0.9999999999999", "1");
        test("1000", "1000");
        test("1000.01", "1000.01");
        test("1000.123456789", "1000.1235");
        test("1000000.1234", "1.0000001 M");
        test("10000000.1234", "10 M");
        test("10000000.5234", "10.000001 M");
        test("999.999999999943", "1000");

        test("-0.123456789", "-0.1234568");
        test("-0.4321", "-0.4321");
        test("-0.0000000000001", "0");
        test("-0.9999999999999", "-1");
        test("-1000", "-1000");
        test("-1000.01", "-1000.01");
        test("-1000.123456789", "-1000.1235");
        test("-1000000.1234", "-1.0000001 M");
        test("-10000000.1234", "-10 M");
        test("-10000000.5234", "-10.000001 M");
        test("-999.999999999943", "-1000");

        // No suffix
        test("1.112221112221112223", "1.1122211");
        test("11.12221112221112223", "11.122211");
        test("111.2221112221112223", "111.22211");
        test("1112.221112221112223", "1112.2211");
        test("11122.21112221112223", "11122.211");
        test("111222.1112221112223", "111222.11");

        // Million
        test("1112221.112221112223332223", "1.1122211 M");
        test("11122211.12221112223332223", "11.122211 M");
        test("111222111.2221112223332223", "111.22211 M");

        // Billion
        test("1112221112.22111222333222333", "1.1122211 B");
        test("11122211122.2111222333222333", "11.122211 B");
        test("111222111222.111222333222333", "111.22211 B");

        // Trillion
        test("1112221112221.11222333222333", "1.1122211 T");
        test("11122211122211.1222333222333", "11.122211 T");
        test("111222111222111.222333222333", "111.22211 T");
        test("1112221112221112.22333222333", "1112.2211 T");
        test("11122211122211122.2333222333", "11122.211 T");
        test("111222111222111222.333222333", "111222.11 T");
        test("1112221112221112223.33222333", "1112221.1 T");
        test("11122211122211122233.3222333", "11122211 T");

        // Too large, we have to use engineering notation
        test("111222111222111222333.222333", "1.112e20");
        test("1112221112221112223332.22333", "1.112e21");
        test("11122211122211122233322.2333", "1.112e22");
        test("111222111222111222333222.333", "1.112e23");
        test("1112221112221112223332223.33", "1.112e24");
        test("11122211122211122233322233.3", "1.112e25");
        test("111222111222111222333222333", "1.112e26");

        test("999999999999999999999.922333", "1e21");
        test("9999999999999999999999.92333", "1e22");
        test("99999999999999999999999.9333", "1e23");
        test("999999999999999999999999.933", "1e24");
        test("9999999999999999999999999.93", "1e25");
        test("99999999999999999999999999.9", "1e26");
        test("999999999999999999999999999", "1e27");

        test("99999994", "99.999994 M");
        test("999999956", "999.99996 M");

        test("9999999462", "9.9999995 B");
        test("100123454", "100.12345 M");
        test("1000123446", "1.0001234 B");
        test("10001234462", "10.001234 B");

        test("100123456", "100.12346 M");
        test("1000123450", "1.0001235 B");
        test("10000123500", "10.000124 B");

        test("9999999900", "9.9999999 B");
        test("9999999900", "9.9999999 B");
        test("9999999900", "9.9999999 B");
        test("9999999500", "9.9999995 B");
        test("9999999400", "9.9999994 B");
        test("9999999000", "9.999999 B");

        test("10000012445.678", "10.000012 B");
        test("10000012445.678", "10.000012 B");
        test("10000012445.678", "10.000012 B");
        test("10000002445.678", "10.000002 B");
        test("10000002445.678", "10.000002 B");

        test("10000012545.678", "10.000013 B");
        test("10000012545.678", "10.000013 B");
        test("10000012545.678", "10.000013 B");
        test("10000002545.678", "10.000003 B");
        test("10000002545.678", "10.000003 B");
        test("10000000055.678", "10 B");

        test("999999999900.00", "1 T");
        test("999999999000.00", "1 T");
        test("999999990000.00", "999.99999 B");
        test("999999950000.00", "999.99995 B");
        test("999999940000.00", "999.99994 B");
        test("999999900000.00", "999.9999 B");

        test("9999999999900.00", "10 T");
        test("9999999999000.00", "10 T");
        test("9999999990000.00", "10 T");
        test("9999999950000.00", "10 T");
        test("9999999940000.00", "9.9999999 T");
        test("9999999900000.00", "9.9999999 T");

        test("10000012445678.9", "10.000012 T");
        test("10000012445678.92", "10.000012 T");
        test("10000012445678.923", "10.000012 T");
        test("10000002445678.9", "10.000002 T");
        test("10000000445678.92", "10 T");
        test("10000000045678.923", "10 T");

        test("10000012545678", "10.000013 T");
        test("10000012545678.2", "10.000013 T");
        test("10000012545678.23", "10.000013 T");
        test("10000002545678", "10.000003 T");
        test("10000002545678.2", "10.000003 T");
        test("10000000055678.23", "10 T");

        test("01434.234", "1434.234");
        test("1434.234", "1434.234");
        test("112.234", "112.234");
        test("12.234", "12.234");
        test("1.234", "1.234");
        test("0.01", "0.01");
        test("0.001", "0.001");
        test("0.00100", "0.001");
        test("0.001000", "0.001");

        test("57896044618.658097719968", "57.896045 B");
        test("1000000000.1", "1 B");
        test("999999999.1", "1 B");
        test("1000000000", "1 B");

        test("1000.1234", "1000.1234");
        test("1000.5", "1000.5");
        test("0.12345674", "0.1234567");
        test("0.12345675", "0.1234568");
        test("0.4321", "0.4321");
        test("0.99999999999999999", "1");
        test("0.00000000000000001", "0");
        test("0", "0");
        test("1", "1");
        test("0.0", "0");
        test("1.0", "1");
    }
}
