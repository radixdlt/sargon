use crate::prelude::*;

/// Defines the rounding strategy used when you round e.g. `Decimal192`.
///
/// Following the same naming convention as https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, enum_iterator::Sequence, strum::Display,
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

impl From<RoundingMode> for ScryptoRoundingMode {
    fn from(value: RoundingMode) -> Self {
        match value {
            RoundingMode::ToPositiveInfinity => {
                ScryptoRoundingMode::ToPositiveInfinity
            }
            RoundingMode::ToNegativeInfinity => {
                ScryptoRoundingMode::ToNegativeInfinity
            }
            RoundingMode::ToZero => ScryptoRoundingMode::ToZero,
            RoundingMode::AwayFromZero => ScryptoRoundingMode::AwayFromZero,
            RoundingMode::ToNearestMidpointTowardZero => {
                ScryptoRoundingMode::ToNearestMidpointTowardZero
            }
            RoundingMode::ToNearestMidpointAwayFromZero => {
                ScryptoRoundingMode::ToNearestMidpointAwayFromZero
            }
            RoundingMode::ToNearestMidpointToEven => {
                ScryptoRoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

impl From<ScryptoRoundingMode> for RoundingMode {
    fn from(value: ScryptoRoundingMode) -> Self {
        match value {
            ScryptoRoundingMode::ToPositiveInfinity => {
                RoundingMode::ToPositiveInfinity
            }
            ScryptoRoundingMode::ToNegativeInfinity => {
                RoundingMode::ToNegativeInfinity
            }
            ScryptoRoundingMode::ToZero => RoundingMode::ToZero,
            ScryptoRoundingMode::AwayFromZero => RoundingMode::AwayFromZero,
            ScryptoRoundingMode::ToNearestMidpointTowardZero => {
                RoundingMode::ToNearestMidpointTowardZero
            }
            ScryptoRoundingMode::ToNearestMidpointAwayFromZero => {
                RoundingMode::ToNearestMidpointAwayFromZero
            }
            ScryptoRoundingMode::ToNearestMidpointToEven => {
                RoundingMode::ToNearestMidpointToEven
            }
        }
    }
}

impl HasSampleValues for RoundingMode {
    fn sample() -> Self {
        RoundingMode::ToPositiveInfinity
    }

    fn sample_other() -> Self {
        RoundingMode::ToNegativeInfinity
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use super::*;

    #[test]
    fn rounding_mode_conversion() {
        let test = |m: RoundingMode| {
            assert_eq!(RoundingMode::from(ScryptoRoundingMode::from(m)), m)
        };
        all::<RoundingMode>().for_each(test);
    }
}
