pub use crate::prelude::*;

/// A type-safe consensus epoch number.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct Epoch(pub EpochInner);

impl Epoch {
    /// Circa 1 hour, since one epoch is circa 6 minutes.
    pub const NUMBER_OF_EPOCHS_PER_HOUR: EpochInner = 10;
    pub const NUMBER_OF_EPOCHS_PER_DAY: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_HOUR * 24;
    pub const NUMBER_OF_EPOCHS_PER_WEEK: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_DAY * 7;

    pub const DEFAULT_EPOCH_WINDOW_SIZE: EpochInner =
        Self::NUMBER_OF_EPOCHS_PER_HOUR;

    /// Arbitrarily set, you have a life.
    const TX_PER_HOUR_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX: EpochInner =
        4;

    /// Arbitrarily set. You sleep and work too.
    const HOURS_PER_DAY_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX: EpochInner =
        8;

    /// Arbitrarily set, you can't spend all day on this.
    const TX_PER_DAY_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX: EpochInner =
        Self::TX_PER_HOUR_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX
            * Self::HOURS_PER_DAY_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX;
}

pub type EpochInner = u64;
impl Epoch {
    pub fn new(value: impl Into<EpochInner>) -> Self {
        Self(value.into())
    }

    pub fn window_end_from_start(start: Self) -> Self {
        Self::new(start.0 + Self::DEFAULT_EPOCH_WINDOW_SIZE)
    }

    pub fn adding(&self, amount: EpochInner) -> Self {
        Self::new(self.0 + amount)
    }

    pub fn one_week_or_more_if_many_manifests_starting_from(
        start: Self,
        number_of_manifests: usize,
    ) -> Self {
        let end = Self::duration_one_week_or_more_if_many_manifests(
            number_of_manifests,
        );
        Self::new(start.0 + end)
    }

    fn duration_one_week_or_more_if_many_manifests(
        number_of_manifests: usize,
    ) -> EpochInner {
        std::cmp::max(
            Self::NUMBER_OF_EPOCHS_PER_WEEK,
            Self::duration_enough_for_broadcasting_manifests(
                number_of_manifests,
            ),
        )
    }

    fn duration_enough_for_broadcasting_manifests(
        number_of_manifests: usize,
    ) -> EpochInner {
        let tx_per_day =
            Self::TX_PER_DAY_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX;
        let days_needed = (number_of_manifests as f32) / (tx_per_day as f32);
        let days_needed = days_needed.ceil() as EpochInner;
        days_needed * Self::NUMBER_OF_EPOCHS_PER_DAY // epochs needed
    }
}

impl From<EpochInner> for Epoch {
    fn from(value: EpochInner) -> Self {
        Self::new(value)
    }
}

impl From<Epoch> for EpochInner {
    fn from(value: Epoch) -> Self {
        value.0
    }
}

impl From<Epoch> for ScryptoEpoch {
    fn from(value: Epoch) -> Self {
        Self::of(value.0)
    }
}

impl From<ScryptoEpoch> for Epoch {
    fn from(value: ScryptoEpoch) -> Self {
        Self::new(value.number())
    }
}

impl HasSampleValues for Epoch {
    fn sample() -> Self {
        Self(0)
    }

    fn sample_other() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Epoch;

    #[test]
    fn into_from_scrypto() {
        let test = |u: u64| assert_eq!(SUT::from(ScryptoEpoch::of(u)).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn from_u64() {
        let test = |u: u64| assert_eq!(u64::from(SUT::from(u)), u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn to_u64() {
        let test = |u: u64| assert_eq!(SUT::from(u64::from(SUT::from(u))).0, u);
        test(0);
        test(1);
        test(2);
        test(1337);
    }

    #[test]
    fn adding() {
        let sut = SUT::new(10u64);
        assert_eq!(sut.adding(5), SUT::new(15u64));
    }

    #[test]
    fn epoch_window_is_always_at_least_one_week_even_if_one_manifest() {
        let duration = SUT::duration_one_week_or_more_if_many_manifests(1);
        assert_eq!(duration, SUT::NUMBER_OF_EPOCHS_PER_WEEK);
    }

    #[test]
    fn epoch_window_more_than_a_week() {
        let duration = SUT::duration_one_week_or_more_if_many_manifests(
            225, // 7 (days per week) * Self::TX_PER_DAY_USER_CAN_SPEND_ON_ADMIN_BROADCASTING_OF_TX => 224
        );
        assert!(duration > SUT::NUMBER_OF_EPOCHS_PER_WEEK);
    }

    #[test]
    fn epoch_window_is_two_weeks_for_many() {
        let duration =
            SUT::duration_one_week_or_more_if_many_manifests(224 * 2);
        assert_eq!(duration, SUT::NUMBER_OF_EPOCHS_PER_WEEK * 2);
    }

    #[test]
    fn epoch_window_is_one_year_for_ridicilus_amount_of_tx() {
        let duration = SUT::duration_one_week_or_more_if_many_manifests(13_000);
        assert!(duration > (SUT::NUMBER_OF_EPOCHS_PER_WEEK * 52));
    }
}
