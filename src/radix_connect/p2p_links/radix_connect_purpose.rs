use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
pub enum RadixConnectPurpose {
    #[serde(rename = "general")]
    General,
}

impl HasSampleValues for RadixConnectPurpose {
    fn sample() -> Self {
        Self::General
    }

    fn sample_other() -> Self {
        Self::General
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectPurpose;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::General, SUT::General].into_iter()).len(),
            1
        );
    }
}
