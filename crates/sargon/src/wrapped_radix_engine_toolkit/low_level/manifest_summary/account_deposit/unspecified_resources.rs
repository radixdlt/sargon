use crate::prelude::*;

/// Represents unspecified resources, which can be either none present or
/// may be present with a list of change sources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnspecifiedResources {
    /// There are no unspecified resources present
    NonePresent,

    /// There might be non-zero balances of unspecified resources present
    MayBePresent,
}

impl UnspecifiedResources {
    pub fn none_present() -> Self {
        Self::NonePresent
    }

    pub fn may_be_present() -> Self {
        Self::MayBePresent
    }
}

impl From<ScryptoUnspecifiedResources> for UnspecifiedResources {
    fn from(value: ScryptoUnspecifiedResources) -> Self {
        match value {
            ScryptoUnspecifiedResources::NonePresent => Self::NonePresent,
            ScryptoUnspecifiedResources::MayBePresent(_) => Self::MayBePresent,
        }
    }
}

impl HasSampleValues for UnspecifiedResources {
    fn sample() -> Self {
        Self::none_present()
    }

    fn sample_other() -> Self {
        Self::may_be_present()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnspecifiedResources;

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
    fn from_scrypto_none_present() {
        let scrypto = ScryptoUnspecifiedResources::NonePresent;
        assert_eq!(SUT::from(scrypto), SUT::none_present());
    }

    #[test]
    fn from_scrypto_may_be_present() {
        let scrypto =
            ScryptoUnspecifiedResources::MayBePresent(IndexSet::from_iter(
                vec![ScryptoChangeSource::InitialYieldFromParent],
            ));
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }
}
