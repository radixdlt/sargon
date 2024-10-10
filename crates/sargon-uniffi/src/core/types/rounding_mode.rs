use crate::prelude::*;
use sargon::RoundingMode as InternalRoundingMode;

/// Defines the rounding strategy used when you round e.g. `Decimal192`.
///
/// Following the same naming convention as https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html.
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    strum::Display,
    uniffi::Enum,
)]
pub enum RoundingMode {
    /// The number is always rounded toward positive infinity, e.g. `3.1 -> 4`, `-3.1 -> -3`.
    ToPositiveInfinity,

    /// The number is always rounded toward negative infinity, e.g. `3.1 -> 3`, `-3.1 -> -4`.
    ToNegativeInfinity,

    /// The number is always rounded toward zero, e.g. `3.1 -> 3`, `-3.1 -> -3`.
    ToZero,

    /// The number is always rounded away from zero, e.g. `3.1 -> 4`, `-3.1 -> -4`.
    AwayFromZero,

    /// The number is rounded to the nearest, and when it is halfway between two others, it's rounded toward zero, e.g. `3.5 -> 3`, `-3.5 -> -3`.
    ToNearestMidpointTowardZero,

    /// The number is rounded to the nearest, and when it is halfway between two others, it's rounded away from zero, e.g. `3.5 -> 4`, `-3.5 -> -4`.
    ToNearestMidpointAwayFromZero,

    /// The number is rounded to the nearest, and when it is halfway between two others, it's rounded toward the nearest even number. Also known as "Bankers Rounding".
    ToNearestMidpointToEven,
}

impl From<InternalRoundingMode> for RoundingMode {
    fn from(value: InternalRoundingMode) -> Self {
        match value {
            InternalRoundingMode::ToPositiveInfinity => Self::ToPositiveInfinity,
            InternalRoundingMode::ToNegativeInfinity => Self::ToNegativeInfinity,
            InternalRoundingMode::ToZero => Self::ToZero,
            InternalRoundingMode::AwayFromZero => Self::AwayFromZero,
            InternalRoundingMode::ToNearestMidpointTowardZero => Self::ToNearestMidpointTowardZero,
            InternalRoundingMode::ToNearestMidpointAwayFromZero => Self::ToNearestMidpointAwayFromZero,
            InternalRoundingMode::ToNearestMidpointToEven => Self::ToNearestMidpointToEven,
        }
    }
}

impl Into<InternalRoundingMode> for RoundingMode {
    fn into(self) -> InternalRoundingMode {
        match self {
            RoundingMode::ToPositiveInfinity => InternalRoundingMode::ToPositiveInfinity,
            RoundingMode::ToNegativeInfinity => InternalRoundingMode::ToNegativeInfinity,
            RoundingMode::ToZero => InternalRoundingMode::ToZero,
            RoundingMode::AwayFromZero => InternalRoundingMode::AwayFromZero,
            RoundingMode::ToNearestMidpointTowardZero => InternalRoundingMode::ToNearestMidpointTowardZero,
            RoundingMode::ToNearestMidpointAwayFromZero => InternalRoundingMode::ToNearestMidpointAwayFromZero,
            RoundingMode::ToNearestMidpointToEven => InternalRoundingMode::ToNearestMidpointToEven,
        }
    }
}