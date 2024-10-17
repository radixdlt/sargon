use crate::prelude::*;

/// Represents unspecified resources, which can be either none present or
/// may be present with a list of change sources.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum UnspecifiedResources {
    NonePresent,
    MayBePresent { change_sources: Vec<ChangeSource> },
}

impl UnspecifiedResources {
    pub fn none_present() -> Self {
        Self::NonePresent
    }

    pub fn may_be_present(
        change_sources: impl IntoIterator<Item = ChangeSource>,
    ) -> Self {
        Self::MayBePresent {
            change_sources: change_sources.into_iter().collect(),
        }
    }
}

impl From<ScryptoUnspecifiedResources> for UnspecifiedResources {
    fn from(value: ScryptoUnspecifiedResources) -> Self {
        match value {
            ScryptoUnspecifiedResources::NonePresent => Self::NonePresent,
            ScryptoUnspecifiedResources::MayBePresent(sources) => {
                Self::MayBePresent {
                    change_sources: sources
                        .into_iter()
                        .map(ChangeSource::from)
                        .collect(),
                }
            }
        }
    }
}

impl HasSampleValues for UnspecifiedResources {
    fn sample() -> Self {
        Self::NonePresent
    }

    fn sample_other() -> Self {
        Self::may_be_present(vec![ChangeSource::sample()])
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
