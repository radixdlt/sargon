use std::{ops::Deref, sync::Arc};

use crate::CommonError;
use radix_engine_common::math::Decimal as NativeDecimal;
use radix_engine_toolkit_json::models::common::SerializableDecimal;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

// FIXME: Use RET's type!
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Object, Default)]
pub struct Decimal(pub(crate) NativeDecimal);

impl Serialize for Decimal {
    /// Serializes this `HDPath` into its bech32 address string as JSON.
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let dec: SerializableDecimal = self.0.into();
        SerializableDecimal::serialize(&dec, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Decimal {
    /// Tries to deserializes a JSON string as a bech32 address into an `HDPath`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
        let s = SerializableDecimal::deserialize(d)?;
        let native: NativeDecimal = *s.deref();
        Ok(Self(native))
    }
}

impl Decimal {
    pub fn try_from_str(s: &str) -> Result<Self, CommonError> {
        Self::new(s.to_string()).map(|a| a.deref().clone())
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[uniffi::export]
impl Decimal {
    #[uniffi::constructor]
    pub fn new(value: String) -> Result<Arc<Self>, CommonError> {
        value
            .parse()
            .map(|value| Arc::new(Self(value)))
            .map_err(|e| CommonError::DecimalError)
    }

    #[uniffi::constructor]
    pub fn max() -> Arc<Self> {
        Arc::new(Self(NativeDecimal::MAX))
    }

    #[uniffi::constructor]
    pub fn min() -> Arc<Self> {
        Arc::new(Self(NativeDecimal::MIN))
    }

    #[uniffi::constructor]
    pub fn zero() -> Arc<Self> {
        Arc::new(Self(NativeDecimal::zero()))
    }

    #[uniffi::constructor]
    pub fn one() -> Arc<Self> {
        Arc::new(Self(NativeDecimal::one()))
    }

    pub fn add(&self, other: Arc<Self>) -> Result<Arc<Self>, CommonError> {
        use radix_engine_common::math::CheckedAdd;
        self.0
            .checked_add(other.0)
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn sub(&self, other: Arc<Self>) -> Result<Arc<Self>, CommonError> {
        use radix_engine_common::math::CheckedSub;
        self.0
            .checked_sub(other.0)
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn mul(&self, other: Arc<Self>) -> Result<Arc<Self>, CommonError> {
        use radix_engine_common::math::CheckedMul;
        self.0
            .checked_mul(other.0)
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn div(&self, other: Arc<Self>) -> Result<Arc<Self>, CommonError> {
        use radix_engine_common::math::CheckedDiv;
        self.0
            .checked_div(other.0)
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn abs(&self) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_abs()
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn floor(&self) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_floor()
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn ceiling(&self) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_ceiling()
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn round(
        &self,
        decimal_places: i32,
        rounding_mode: RoundingMode,
    ) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_round(decimal_places, rounding_mode.into())
            .ok_or(CommonError::DecimalError)
            .map(Self)
            .map(Arc::new)
    }

    pub fn powi(&self, exp: i64) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_powi(exp)
            .map(Self)
            .map(Arc::new)
            .ok_or(CommonError::DecimalError)
    }

    pub fn sqrt(&self) -> Option<Arc<Self>> {
        self.0.checked_sqrt().map(|value| Arc::new(Self(value)))
    }

    pub fn cbrt(&self) -> Result<Arc<Self>, CommonError> {
        self.0
            .checked_cbrt()
            .map(Self)
            .map(Arc::new)
            .ok_or(CommonError::DecimalError)
    }

    pub fn nth_root(&self, n: u32) -> Option<Arc<Self>> {
        self.0
            .checked_nth_root(n)
            .map(|value| Arc::new(Self(value)))
    }

    pub fn equal(&self, other: Arc<Self>) -> bool {
        self.0.eq(&other.0)
    }

    pub fn not_equal(&self, other: Arc<Self>) -> bool {
        self.0.ne(&other.0)
    }

    pub fn greater_than(&self, other: Arc<Self>) -> bool {
        self.0.gt(&other.0)
    }

    pub fn greater_than_or_equal(&self, other: Arc<Self>) -> bool {
        self.0.ge(&other.0)
    }

    pub fn less_than(&self, other: Arc<Self>) -> bool {
        self.0.lt(&other.0)
    }

    pub fn less_than_or_equal(&self, other: Arc<Self>) -> bool {
        self.0.le(&other.0)
    }

    pub fn mantissa(&self) -> String {
        self.0 .0.to_string()
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
