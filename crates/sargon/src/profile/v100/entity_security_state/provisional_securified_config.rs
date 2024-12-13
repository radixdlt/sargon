use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProvisionalSecurifiedTransactionQueued {
    pub factor_instances: SecurityStructureOfFactorInstances,
    pub txid: TransactionIntentHash,
}

impl ProvisionalSecurifiedTransactionQueued {
    pub fn new(
        factor_instances: SecurityStructureOfFactorInstances,
        txid: TransactionIntentHash,
    ) -> Self {
        Self {
            factor_instances,
            txid,
        }
    }
}
impl HasSampleValues for ProvisionalSecurifiedTransactionQueued {
    fn sample() -> Self {
        Self::new(
            SecurityStructureOfFactorInstances::sample(),
            TransactionIntentHash::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            SecurityStructureOfFactorInstances::sample_other(),
            TransactionIntentHash::sample_other(),
        )
    }
}
#[cfg(test)]
mod provisional_securified_transaction_queued_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProvisionalSecurifiedTransactionQueued;

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

    /// User has made queued a transaction to apply a `SecurityStructureOfFactorInstances`
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
