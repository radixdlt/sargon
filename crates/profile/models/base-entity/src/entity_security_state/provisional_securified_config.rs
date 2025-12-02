use crate::prelude::*;

/// Intermediary state of changing the security structure of an entity.
/// Only a single variant for now but we might update it later. E.g.
/// we could have one state for when user has selected a shield but not
/// derived the factor instances yet.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumAsInner,
)]
#[serde(tag = "discriminator")]
pub enum ProvisionalSecurifiedConfig {
    /// User has fully prepared a `SecurityStructureOfFactorInstances` but
    /// not made a transaction to apply it to the entity yet.
    #[serde(rename = "factorInstancesDerived")]
    FactorInstancesDerived {
        value: SecurityStructureOfFactorInstances,
    },
}

impl ProvisionalSecurifiedConfig {
    pub fn get_security_structure_of_factor_instances(
        &self,
    ) -> SecurityStructureOfFactorInstances {
        match self {
            Self::FactorInstancesDerived { value } => value.clone(),
        }
    }
}

impl HasSampleValues for ProvisionalSecurifiedConfig {
    fn sample() -> Self {
        Self::FactorInstancesDerived {
            value: SecurityStructureOfFactorInstances::sample(),
        }
    }
    fn sample_other() -> Self {
        Self::FactorInstancesDerived {
            value: SecurityStructureOfFactorInstances::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProvisionalSecurifiedConfig;

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
