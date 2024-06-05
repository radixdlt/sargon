use crate::prelude::*;

/// A stable and globally unique identifier of a `SecurityStructureOfFactorSources` the
/// user has created. Also used in `SecurityStructureOfFactorSourceIDs` and in
/// `SecurityStructureOfFactorInstances`.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
#[serde(transparent)]
pub struct SecurityStructureID(pub(crate) Uuid);
uniffi::custom_newtype!(SecurityStructureID, Uuid);

impl From<Uuid> for SecurityStructureID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for SecurityStructureID {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(SecurityStructureID).map_err(|_| {
            CommonError::InvalidSecurityStructureID {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl HasSampleValues for SecurityStructureID {
    fn sample() -> Self {
        SecurityStructureID(Uuid::sample())
    }

    fn sample_other() -> Self {
        SecurityStructureID(Uuid::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureID;

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
    fn from_str_invalid() {
        assert_eq!(
            "bad".parse::<SUT>(),
            Err(CommonError::InvalidSecurityStructureID {
                bad_value: "bad".to_owned()
            })
        );
    }
}
