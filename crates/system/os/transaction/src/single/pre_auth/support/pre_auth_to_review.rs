use crate::prelude::*;

/// This is the result of the Pre-Auth preview analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
pub enum PreAuthToReview {
    /// Pre-Auth analysis open manifest, which contains multiple interactions with the parent manifest,
    /// thus its preview can be computed only based on the static analysis manifest summary
    Open(PreAuthOpenManifest),

    /// Pre-Auth analysis enclosed manifest, which does not contain any interactions with the parent manifest,
    /// thus its preview can be computed as if it would have been a standalone transaction.
    Enclosed(PreAuthEnclosedManifest),
}

/// Pre-Auth analysis open manifest, which contains multiple interactions with the parent manifest,
/// thus its preview can be computed only based on the static analysis manifest summary
#[derive(Clone, Debug, PartialEq)]
pub struct PreAuthOpenManifest {
    pub manifest: SubintentManifest,
    pub summary: ManifestSummary,
}

/// Pre-Auth analysis enclosed manifest, which does not contain any interactions with the parent manifest,
/// thus its preview can be computed as if it would have been a standalone transaction.
#[derive(Clone, Debug, PartialEq)]
pub struct PreAuthEnclosedManifest {
    pub manifest: SubintentManifest,
    pub summary: ExecutionSummary,
}

impl HasSampleValues for PreAuthToReview {
    fn sample() -> Self {
        Self::Open(PreAuthOpenManifest::sample())
    }

    fn sample_other() -> Self {
        Self::Enclosed(PreAuthEnclosedManifest::sample())
    }
}

impl HasSampleValues for PreAuthOpenManifest {
    fn sample() -> Self {
        Self {
            manifest: SubintentManifest::sample(),
            summary: ManifestSummary::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            manifest: SubintentManifest::sample_other(),
            summary: ManifestSummary::sample_other(),
        }
    }
}

impl HasSampleValues for PreAuthEnclosedManifest {
    fn sample() -> Self {
        Self {
            manifest: SubintentManifest::sample(),
            summary: ExecutionSummary::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            manifest: SubintentManifest::sample_other(),
            summary: ExecutionSummary::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PreAuthToReview;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
