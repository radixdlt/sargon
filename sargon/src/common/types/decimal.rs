use crate::prelude::*;
use radix_engine_common::math::Decimal as NativeDecimal;
use radix_engine_toolkit_json::models::common::SerializableDecimal;

// FIXME: Use RET's type!
#[derive(Clone, Debug, Eq, uniffi::Record, Default)]
pub struct Decimal {
    base10_string: String,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.native().eq(&other.native())
    }
}
impl std::hash::Hash for Decimal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.native().hash(state);
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = &self.native();
        let rhs = &other.native();
        if lhs.eq(rhs) {
            Ordering::Equal
        } else if lhs.le(rhs) {
            return Ordering::Less;
        } else {
            assert!(lhs.gt(rhs), "!(LHS == RHS || LHS < RHS), thus we expected LHS > RHS, but it was not. Most likely the implementation of RET's Decimal has changed, maybe to involve NaN?");
            return Ordering::Greater;
        }
    }
}
impl Decimal {
    fn native(&self) -> NativeDecimal {
        NativeDecimal::from_str(&self.base10_string).unwrap()
    }
    fn from_native(decimal: NativeDecimal) -> Self {
        Self {
            base10_string: decimal.to_string(),
        }
    }
}

impl Serialize for Decimal {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let dec: SerializableDecimal = self.native().into();
        SerializableDecimal::serialize(&dec, serializer)
    }
}

impl<'de> Deserialize<'de> for Decimal {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
        use std::ops::Deref;

        let s = SerializableDecimal::deserialize(d)?;
        let native: NativeDecimal = *s.deref();
        Ok(Self::from_native(native))
    }
}

impl Decimal {
    pub fn try_from_str(s: &str) -> Result<Self> {
        Self::new(s.to_string())
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.native())
    }
}

impl Decimal {
    pub fn new(value: String) -> Result<Self> {
        value
            .parse::<NativeDecimal>()
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }

    pub fn zero() -> Self {
        Self::from_native(NativeDecimal::zero())
    }

    pub fn one() -> Self {
        Self::from_native(NativeDecimal::one())
    }

    pub fn is_zero(&self) -> bool {
        self.native().is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.native().is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.native().is_negative()
    }
}

impl TryInto<Decimal> for &str {
    type Error = crate::CommonError;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        Decimal::try_from_str(self)
    }
}

impl TryFrom<&[u8]> for Decimal {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        NativeDecimal::try_from(slice)
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
        assert_eq!(Decimal::zero(), Decimal::try_from_str("0").unwrap());
        assert_eq!(Decimal::one(), Decimal::try_from_str("1").unwrap());
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
        assert!(Decimal::try_from_str("-1").unwrap().is_negative());
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
        let a = Decimal::try_from_str(
            "3138550867693340381917894711603833208051.177722232017256447",
        )
        .unwrap();
        let b = Decimal::try_from_str(
            "3036550867693340381917894711603833208050.177722232017256447",
        )
        .unwrap();
        assert!(a > b);
    }

    #[test]
    fn try_from_invalid_str() {
        assert_eq!(
            Decimal::try_from_str("apabanan"),
            Err(CommonError::DecimalError)
        );
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
