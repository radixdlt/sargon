use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct WalletInteractionVersion(pub u64);

impl From<u64> for WalletInteractionVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl WalletInteractionVersion {
    pub fn current() -> Self {
        Self(2)
    }
}

impl HasSampleValues for WalletInteractionVersion {
    fn sample() -> Self {
        Self(1)
    }

    fn sample_other() -> Self {
        Self(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletInteractionVersion;

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
    fn current() {
        assert_eq!(SUT::current(), 2.into());
    }
}
