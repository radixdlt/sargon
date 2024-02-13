use crate::prelude::*;
use delegate::delegate;
use radix_engine_common::math::Decimal as ScryptoDecimal192;

/// UniFFI conversion for InnerDecimal using String as builtin.
impl crate::UniffiCustomTypeConverter for InnerDecimal {
    type Builtin = String;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::try_from(val).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

/// A 192 bit precision Decimal wrapping Scrypto's `Decimal` type, but
/// giving it UniFFI conversion.
///
/// The purpose of this "Inner" Decimal is ensuring that the `Decimal` type
/// is not exported as a `String` in FFI land (Swift/Kotlin). The current
/// design ensure that `Decimal` is converted into a Swift `struct` / Kotlin
/// `data class` that **has** a `inner: String`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InnerDecimal(pub(crate) ScryptoDecimal192);
impl TryFrom<String> for InnerDecimal {
    type Error = crate::CommonError;

    fn try_from(value: String) -> Result<Self> {
        ScryptoDecimal192::from_str(&value)
            .map(InnerDecimal)
            .map_err(|_| CommonError::DecimalError)
    }
}
impl From<InnerDecimal> for String {
    fn from(value: InnerDecimal) -> Self {
        value.0.to_string()
    }
}

/// `Decimal` represents a 192 bit representation of a fixed-scale decimal number.
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
/// Powering it is the [Scrypto Decimal type, see docs][scrypto]
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
pub struct Decimal {
    /// @Kotlin / Swift developer: Do not use this property/field. Instead use all the provided
    /// methods on the `Decimal` type.
    __inner: InnerDecimal, // Strange field name to try as much as possible hide it in FFI land.
}

// impl From<InnerDecimal> for Decimal {
//     fn from(value: InnerDecimal) -> Self {
//         Decimal { __inner: value }
//     }
// }

// impl From<Decimal> for InnerDecimal {
//     fn from(value: Decimal) -> Self {
//         value.__inner
//     }
// }

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
        self.__inner.0
    }

    fn from_native(decimal: ScryptoDecimal192) -> Self {
        Decimal {
            __inner: InnerDecimal(decimal),
        }
    }
}

impl FromStr for Decimal {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<ScryptoDecimal192>()
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

impl From<u32> for Decimal {
    fn from(value: u32) -> Self {
        ScryptoDecimal192::from(value).into()
    }
}
impl From<u64> for Decimal {
    fn from(value: u64) -> Self {
        ScryptoDecimal192::from(value).into()
    }
}
impl From<i32> for Decimal {
    fn from(value: i32) -> Self {
        ScryptoDecimal192::from(value).into()
    }
}
impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        ScryptoDecimal192::from(value).into()
    }
}

impl Decimal {
    pub fn new(value: String) -> Result<Self> {
        value.parse()
    }

    pub fn zero() -> Self {
        Self::from_native(ScryptoDecimal192::zero())
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
        Into::into(self.native() + rhs.native())
    }
}
impl Sub for Decimal {
    type Output = Self;
    /// self - rhs
    fn sub(self, rhs: Self) -> Self::Output {
        Into::into(self.native() - rhs.native())
    }
}
impl Mul for Decimal {
    type Output = Self;
    /// self * rhs
    fn mul(self, rhs: Self) -> Self::Output {
        Into::into(self.native() * rhs.native())
    }
}
impl Div for Decimal {
    type Output = Self;
    /// self / rhs
    fn div(self, rhs: Self) -> Self::Output {
        Into::into(self.native() / rhs.native())
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

/// Tries to creates a new `Decimal` from a String, throws a `CommonError`
/// if the `string` was not a valid Decimal.
#[uniffi::export]
pub fn new_decimal_from_string(string: String) -> Result<Decimal> {
    Decimal::new(string)
}

/// Creates a new `Decimal` from a u32 integer.
#[uniffi::export]
pub fn new_decimal_from_u32(value: u32) -> Decimal {
    value.into()
}

/// Creates a new `Decimal` from a u64 integer.
#[uniffi::export]
pub fn new_decimal_from_u64(value: u64) -> Decimal {
    value.into()
}

/// Creates a new `Decimal` from a i32 integer.
#[uniffi::export]
pub fn new_decimal_from_i32(value: i32) -> Decimal {
    value.into()
}

/// Creates a new `Decimal` from a i64 integer.
#[uniffi::export]
pub fn new_decimal_from_i64(value: i64) -> Decimal {
    value.into()
}

/// Whether this decimal is zero.
#[uniffi::export]
pub fn decimal_is_zero(decimal: &Decimal) -> bool {
    decimal.is_zero()
}

/// Whether this decimal is positive.
#[uniffi::export]
pub fn decimal_is_positive(decimal: &Decimal) -> bool {
    decimal.is_positive()
}

/// Whether this decimal is negative.
#[uniffi::export]
pub fn decimal_is_negative(decimal: &Decimal) -> bool {
    decimal.is_negative()
}

/// `lhs + rhs``
#[uniffi::export]
pub fn decimal_add(lhs: Decimal, rhs: Decimal) -> Decimal {
    lhs + rhs
}

/// `lhs - rhs``
#[uniffi::export]
pub fn decimal_sub(lhs: Decimal, rhs: Decimal) -> Decimal {
    lhs - rhs
}

/// `lhs * rhs``
#[uniffi::export]
pub fn decimal_mul(lhs: Decimal, rhs: Decimal) -> Decimal {
    lhs * rhs
}

/// `lhs / rhs``
#[uniffi::export]
pub fn decimal_div(lhs: Decimal, rhs: Decimal) -> Decimal {
    lhs / rhs
}

/// Negates the `decimal`
#[uniffi::export]
pub fn decimal_neg(decimal: &Decimal) -> Decimal {
    decimal.neg()
}

impl TryInto<Decimal> for &str {
    type Error = crate::CommonError;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        self.parse::<Decimal>()
    }
}

impl TryFrom<&[u8]> for Decimal {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        ScryptoDecimal192::try_from(slice)
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn eq() {
        assert_eq!(Decimal::zero(), Decimal::zero());
        assert_eq!(Decimal::one(), Decimal::one());
        assert_eq!(Decimal::zero(), "0".parse().unwrap());
        assert_eq!(Decimal::one(), "1".parse().unwrap());
    }

    #[test]
    fn inequality() {
        assert_ne!(Decimal::one(), Decimal::zero());
    }

    #[test]
    fn is_zero() {
        assert!(Decimal::zero().is_zero());
        assert!(!Decimal::one().is_zero());
    }

    #[test]
    fn is_positive() {
        assert!(!Decimal::zero().is_positive());
        assert!(Decimal::one().is_positive());
    }

    #[test]
    fn is_negative() {
        assert!("-1".parse::<Decimal>().unwrap().is_negative());
        assert!(!Decimal::zero().is_negative());
        assert!(!Decimal::one().is_negative());
    }

    #[test]
    fn not_less() {
        assert!(Decimal::zero() >= Decimal::zero());
        assert!(Decimal::one() >= Decimal::one());
        assert!(Decimal::one() >= Decimal::zero());
    }

    #[test]
    fn less() {
        assert!(Decimal::zero() < Decimal::one());
    }

    #[test]
    fn leq() {
        assert!(Decimal::zero() <= Decimal::zero());
        assert!(Decimal::one() <= Decimal::one());

        assert!(Decimal::one() > Decimal::zero());
    }

    #[test]
    fn not_greater_than() {
        assert!(Decimal::zero() <= Decimal::zero());
        assert!(Decimal::one() <= Decimal::one());
        assert!(Decimal::zero() <= Decimal::one());
    }

    #[test]
    fn geq() {
        assert!(Decimal::zero() >= Decimal::zero());
        assert!(Decimal::one() >= Decimal::one());

        assert!(Decimal::zero() < Decimal::one());
    }

    #[test]
    fn greater() {
        assert!(Decimal::one() > Decimal::zero());
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
            Decimal::try_from(generate_32_bytes().as_slice()),
            Err(CommonError::DecimalError)
        );
    }

    #[test]
    fn try_from_valid_bytes() {
        assert!(Decimal::try_from(generate_bytes::<24>().as_slice()).is_ok());
    }

    #[test]
    fn display() {
        let s = "3138550867693340381917894711603833208051.177722232017256447";
        let a: Decimal = s.try_into().unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: Decimal =
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
                Decimal::try_from(generate_bytes::<24>().as_slice()).unwrap()
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

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
}
