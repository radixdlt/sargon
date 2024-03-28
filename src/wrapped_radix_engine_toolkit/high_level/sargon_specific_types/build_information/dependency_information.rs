use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Enum,
)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

impl DependencyInformation {
    pub(crate) fn with_value(version: &str) -> Self {
        let mut split = version.split('=');
        let identifier = split.next().expect("Should never fail").trim();
        let value = split.next().expect("Should never fail").trim();

        match identifier {
            "version" => Self::Version(value.into()),
            "tag" => Self::Tag(value.into()),
            "branch" => Self::Branch(value.into()),
            "rev" => Self::Rev(value.into()),
            _ => {
                unreachable!("Unknown identifier encountered: '{}'", identifier)
            }
        }
    }
}

impl HasSampleValues for DependencyInformation {
    fn sample() -> Self {
        Self::Branch("develop".to_owned())
    }

    fn sample_other() -> Self {
        Self::Tag("2.3.7".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DependencyInformation;

    #[test]
    #[ignore]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    #[ignore]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "develop");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample_other()), "Tag(\"2.3.7\")");
    }
}
