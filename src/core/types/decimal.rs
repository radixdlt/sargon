use crate::prelude::*;
use radix_engine_common::math::Decimal as ScryptoDecimal;

// FIXME: Use RET's type!
#[derive(
    Clone,
    Debug,
    Eq,
    Default,
    SerializeDisplay,
    DeserializeFromStr,
    uniffi::Record,
)]
pub struct Decimal192 {
    base10_string: String,
}

impl PartialEq for Decimal192 {
    fn eq(&self, other: &Self) -> bool {
        self.native().eq(&other.native())
    }
}
impl std::hash::Hash for Decimal192 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.native().hash(state);
    }
}
impl PartialOrd for Decimal192 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Decimal192 {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = &self.native();
        let rhs = &other.native();
        if lhs.eq(rhs) {
            Ordering::Equal
        } else if lhs.le(rhs) {
            return Ordering::Less;
        } else {
            assert!(lhs.gt(rhs), "!(LHS == RHS || LHS < RHS), thus we expected LHS > RHS, but it was not. Most likely the implementation of RET's Decimal192 has changed, maybe to involve NaN?");
            return Ordering::Greater;
        }
    }
}
impl Decimal192 {
    fn native(&self) -> ScryptoDecimal {
        ScryptoDecimal::from_str(&self.base10_string).unwrap()
    }
    fn from_native(decimal: ScryptoDecimal) -> Self {
        Self {
            base10_string: decimal.to_string(),
        }
    }
}

impl Decimal192 {
    pub fn try_from_str(s: &str) -> Result<Self> {
        Self::new(s.to_string())
    }
}

impl std::fmt::Display for Decimal192 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.native())
    }
}

impl FromStr for Decimal192 {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<ScryptoDecimal>()
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

impl Decimal192 {
    pub fn new(value: String) -> Result<Self> {
        value.parse()
    }

    pub fn zero() -> Self {
        Self::from_native(ScryptoDecimal::zero())
    }

    pub fn one() -> Self {
        Self::from_native(ScryptoDecimal::one())
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

impl TryInto<Decimal192> for &str {
    type Error = crate::CommonError;

    fn try_into(self) -> Result<Decimal192, Self::Error> {
        Decimal192::try_from_str(self)
    }
}

impl TryFrom<&[u8]> for Decimal192 {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        ScryptoDecimal::try_from(slice)
            .map(Self::from_native)
            .map_err(|_| CommonError::DecimalError)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn eq() {
        assert_eq!(Decimal192::zero(), Decimal192::zero());
        assert_eq!(Decimal192::one(), Decimal192::one());
        assert_eq!(Decimal192::zero(), Decimal192::try_from_str("0").unwrap());
        assert_eq!(Decimal192::one(), Decimal192::try_from_str("1").unwrap());
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
        assert!(Decimal192::try_from_str("-1").unwrap().is_negative());
        assert!(!Decimal192::zero().is_negative());
        assert!(!Decimal192::one().is_negative());
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
    fn from_str() {
        let a = Decimal192::try_from_str(
            "3138550867693340381917894711603833208051.177722232017256447",
        )
        .unwrap();
        let b = Decimal192::try_from_str(
            "3036550867693340381917894711603833208050.177722232017256447",
        )
        .unwrap();
        assert!(a > b);
    }

    #[test]
    fn try_from_invalid_str() {
        assert_eq!(
            Decimal192::try_from_str("apabanan"),
            Err(CommonError::DecimalError)
        );
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
}
