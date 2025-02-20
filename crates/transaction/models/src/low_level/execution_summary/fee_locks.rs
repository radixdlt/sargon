use crate::prelude::*;

/// Information on how much fees were contingent and how much were not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeeLocks {
    pub lock: Decimal192,
    pub contingent_lock: Decimal192,
}

impl FeeLocks {
    pub fn new(
        lock: impl Into<Decimal192>,
        contingent_lock: impl Into<Decimal192>,
    ) -> Self {
        Self {
            lock: lock.into(),
            contingent_lock: contingent_lock.into(),
        }
    }
}

impl Default for FeeLocks {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl From<ScryptoFeeLocks> for FeeLocks {
    fn from(value: ScryptoFeeLocks) -> Self {
        Self::new(value.lock, value.contingent_lock)
    }
}

impl HasSampleValues for FeeLocks {
    fn sample() -> Self {
        Self::new(25, 0)
    }

    fn sample_other() -> Self {
        Self::new(5, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FeeLocks;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_ret() {
        assert_eq!(
            SUT::from(ScryptoFeeLocks {
                lock: 5.into(),
                contingent_lock: 2.into(),
            }),
            SUT::sample_other()
        );
    }
}
