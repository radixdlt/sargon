use crate::prelude::*;

/// The different intermediary states of changing the security structure of an entity.
/// This type is put in an `Option` on either `UnsecuredEntityControl` or `SecurifiedEntityControl`,
/// and if `None` it means user has no provisionally changed security structure. If set, it contains
/// these different variants:
/// * `ShieldSelected` - User has selected which security shield to use for some entity,
/// * `FactorInstancesDerived` - Sargon has provided a `SecurityStructureOfFactorInstances` but
///     user has not made a transaction to apply it to the entity yet.
/// * `TransactionQueued` - User has signed and queued a transaction changing to `SecurityStructureOfFactorInstances`
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, EnumAsInner,
)]
#[serde(tag = "discriminator")]
pub enum ProvisionalSecurifiedConfig {
    /// User has selected which security shield to use for some entity,
    /// but no FactorInstances has been provided yet.
    #[serde(rename = "shieldSelected")]
    ShieldSelected { value: SecurityStructureID },

    /// User has fully prepared a `SecurityStructureOfFactorInstances` but
    /// not made a transaction to apply it to the entity yet.
    #[serde(rename = "factorInstancesDerived")]
    FactorInstancesDerived {
        value: SecurityStructureOfFactorInstances,
    },

    /// User has signed and queued a transaction to apply a `SecurityStructureOfFactorInstances`
    /// but it has not been submitted (confirmed) yet.
    #[serde(rename = "transactionQueued")]
    TransactionQueued {
        value: ProvisionalSecurifiedTransactionQueued,
    },
}

impl HasSampleValues for ProvisionalSecurifiedConfig {
    fn sample() -> Self {
        Self::ShieldSelected {
            value: SecurityStructureID::sample(),
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
