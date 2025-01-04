use crate::prelude::*;

/// A tuple of a `SecurityStructureOfFactorInstances` and a `TransactionIntentHash`
/// which represents a queued transaction to which is changing the security structure
/// if some entity. Since this provisional state is set on the entity itself, no
/// need to store the entity address here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProvisionalSecurifiedTransactionQueued {
    /// The FactorInstances we are changing to.
    pub factor_instances: SecurityStructureOfFactorInstances,

    /// The ID of the queued transaction which is changing the security structure
    /// to `factor_instances`.
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
