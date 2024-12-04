use crate::prelude::*;

/// The response of a batch signing request, either a PolyFactor or MonoFactor signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("SignResponse {{ signatures: {:#?} }}", signatures.values().map(|f| format!("{:#?}", f)).join(", "))]
pub struct SignResponse<ID: SignableID> {
    pub signatures: IndexMap<FactorSourceIDFromHash, IndexSet<HDSignature<ID>>>,
}

impl<ID: SignableID> SignResponse<ID> {
    pub fn new(
        signatures: IndexMap<FactorSourceIDFromHash, IndexSet<HDSignature<ID>>>,
    ) -> Self {
        Self { signatures }
    }

    pub fn with_signatures(signatures: IndexSet<HDSignature<ID>>) -> Self {
        let signatures = signatures
            .into_iter()
            .into_group_map_by(|x| x.factor_source_id());
        Self::new(
            signatures
                .into_iter()
                .map(|(k, v)| (k, IndexSet::from_iter(v)))
                .collect(),
        )
    }
}

impl<ID: SignableID + HasSampleValues> HasSampleValues for SignResponse<ID> {
    fn sample() -> Self {
        let hd_signature = HDSignature::sample();
        let factor_source_id = hd_signature
            .input
            .owned_factor_instance
            .value
            .factor_source_id;
        Self::new(IndexMap::just((
            factor_source_id,
            IndexSet::just(hd_signature),
        )))
    }

    fn sample_other() -> Self {
        let hd_signature = HDSignature::sample_other();
        let factor_source_id = hd_signature
            .input
            .owned_factor_instance
            .value
            .factor_source_id;
        Self::new(IndexMap::just((
            factor_source_id,
            IndexSet::just(hd_signature),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignResponse<TransactionIntentHash>;

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
