#[cfg(not(tarpaulin_include))] // WILL MIGRATE SOON
use std::{ops::Deref, str::FromStr};

use crate::CommonError;
use radix_engine_common::math::Decimal as NativeDecimal;
use radix_engine_toolkit_json::models::common::SerializableDecimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

// FIXME: Use RET's type!
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record, Default)]
pub struct Decimal {
    base10_string: String,
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
    /// Serializes this `HDPath` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let dec: SerializableDecimal = self.native().into();
        SerializableDecimal::serialize(&dec, serializer)
    }
}

impl<'de> Deserialize<'de> for Decimal {
    /// Tries to deserializes a JSON string as a bech32 address into an `HDPath`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
        let s = SerializableDecimal::deserialize(d)?;
        let native: NativeDecimal = *s.deref();
        Ok(Self::from_native(native))
    }
}

impl Decimal {
    pub fn try_from_str(s: &str) -> Result<Self, CommonError> {
        Self::new(s.to_string())
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.native())
    }
}

impl Decimal {
    pub fn new(value: String) -> Result<Self, CommonError> {
        value
            .parse::<NativeDecimal>()
            .map(|native| Self::from_native(native))
            .map_err(|_| CommonError::DecimalError)
    }

    pub fn max() -> Self {
        Self::from_native(NativeDecimal::MAX)
    }

    pub fn min() -> Self {
        Self::from_native(NativeDecimal::MIN)
    }

    pub fn zero() -> Self {
        Self::from_native(NativeDecimal::zero())
    }

    pub fn one() -> Self {
        Self::from_native(NativeDecimal::one())
    }

    pub fn add(&self, other: Self) -> Result<Self, CommonError> {
        use radix_engine_common::math::CheckedAdd;
        self.native()
            .checked_add(other.native())
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn sub(&self, other: Self) -> Result<Self, CommonError> {
        use radix_engine_common::math::CheckedSub;
        self.native()
            .checked_sub(other.native())
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn mul(&self, other: Self) -> Result<Self, CommonError> {
        use radix_engine_common::math::CheckedMul;
        self.native()
            .checked_mul(other.native())
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn div(&self, other: Self) -> Result<Self, CommonError> {
        use radix_engine_common::math::CheckedDiv;
        self.native()
            .checked_div(other.native())
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn as_str(&self) -> String {
        self.native().to_string()
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

    pub fn abs(&self) -> Result<Self, CommonError> {
        self.native()
            .checked_abs()
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn floor(&self) -> Result<Self, CommonError> {
        self.native()
            .checked_floor()
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn ceiling(&self) -> Result<Self, CommonError> {
        self.native()
            .checked_ceiling()
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn round(
        &self,
        decimal_places: i32,
        rounding_mode: RoundingMode,
    ) -> Result<Self, CommonError> {
        self.native()
            .checked_round(decimal_places, rounding_mode.into())
            .ok_or(CommonError::DecimalError)
            .map(Self::from_native)
    }

    pub fn powi(&self, exp: i64) -> Result<Self, CommonError> {
        self.native()
            .checked_powi(exp)
            .map(Self::from_native)
            .ok_or(CommonError::DecimalError)
    }

    pub fn sqrt(&self) -> Option<Self> {
        self.native().checked_sqrt().map(Self::from_native)
    }

    pub fn cbrt(&self) -> Result<Self, CommonError> {
        self.native()
            .checked_cbrt()
            .map(Self::from_native)
            .ok_or(CommonError::DecimalError)
    }

    pub fn nth_root(&self, n: u32) -> Option<Self> {
        self.native().checked_nth_root(n).map(Self::from_native)
    }

    pub fn equal(&self, other: Self) -> bool {
        self.native().eq(&other.native())
    }

    pub fn not_equal(&self, other: Self) -> bool {
        self.native().ne(&other.native())
    }

    pub fn greater_than(&self, other: Self) -> bool {
        self.native().gt(&other.native())
    }

    pub fn greater_than_or_equal(&self, other: Self) -> bool {
        self.native().ge(&other.native())
    }

    pub fn less_than(&self, other: Self) -> bool {
        self.native().lt(&other.native())
    }

    pub fn less_than_or_equal(&self, other: Self) -> bool {
        self.native().le(&other.native())
    }

    pub fn mantissa(&self) -> String {
        self.native().0.to_string()
    }
}

#[derive(Clone, Debug, uniffi::Enum)]
pub enum RoundingMode {
    ToPositiveInfinity,
    ToNegativeInfinity,
    ToZero,
    AwayFromZero,
    ToNearestMidpointTowardZero,
    ToNearestMidpointAwayFromZero,
    ToNearestMidpointToEven,
}

impl From<RoundingMode> for radix_engine_common::math::RoundingMode {
    fn from(value: RoundingMode) -> Self {
        match value {
            RoundingMode::ToPositiveInfinity => {
                radix_engine_common::math::RoundingMode::ToPositiveInfinity
            }
            RoundingMode::ToNegativeInfinity => {
                radix_engine_common::math::RoundingMode::ToNegativeInfinity
            }
            RoundingMode::ToZero => radix_engine_common::math::RoundingMode::ToZero,
            RoundingMode::AwayFromZero => radix_engine_common::math::RoundingMode::AwayFromZero,
            RoundingMode::ToNearestMidpointTowardZero => {
                radix_engine_common::math::RoundingMode::ToNearestMidpointTowardZero
            }
            RoundingMode::ToNearestMidpointAwayFromZero => {
                radix_engine_common::math::RoundingMode::ToNearestMidpointAwayFromZero
            }
            RoundingMode::ToNearestMidpointToEven => {
                radix_engine_common::math::RoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Decimal;

    #[test]
    fn add() {
        let a =
            Decimal::try_from_str("3138550867693340381917894711603833208051.177722232017256447")
                .unwrap();
        let b =
            Decimal::try_from_str("3036550867693340381917894711603833208050.177722232017256447")
                .unwrap();
        let c = Decimal::try_from_str("102000000000000000000000000000000000001");
        assert_eq!(a.sub(b), c);
    }
}
