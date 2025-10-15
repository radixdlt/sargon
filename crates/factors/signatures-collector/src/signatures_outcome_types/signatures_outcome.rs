use crate::prelude::*;

/// The outcome of a SignaturesCollector, containing a collection for transactions
/// which would be successful if submitted to the network (from a signatures point of view)
/// and a collection of transactions which would fail if submitted to the network,
/// since not enough signatures have been gathered. And a collection of factor sources
/// which were skipped.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SignaturesOutcome<ID: SignableID> {
    /// A potentially empty collection of transactions which would be
    /// successful if submitted to the network (from a signatures point of view).
    ///
    /// Potentially empty
    successful_transactions: MaybeSignedTransactions<ID>,

    /// A collection of transactions which would fail if submitted to the network,
    /// since not enough signatures have been gathered.
    ///
    /// Potentially empty
    failed_transactions: MaybeSignedTransactions<ID>,

    /// List of all neglected factor sources, either explicitly skipped by user or
    /// implicitly neglected due to failure.
    neglected_factor_sources: IndexSet<NeglectedFactor>,
}

impl<ID: SignableID> SignaturesOutcome<ID> {
    /// # Panics
    /// Panics if the `successful_transactions` or `failed_transactions` shared
    /// either any transaction intent hash, or any signature.
    pub(crate) fn new(
        successful_transactions: MaybeSignedTransactions<ID>,
        failed_transactions: MaybeSignedTransactions<ID>,
        neglected_factor_sources: impl IntoIterator<Item = NeglectedFactor>,
    ) -> Self {
        let neglected_factor_sources = neglected_factor_sources
            .into_iter()
            .collect::<IndexSet<_>>();

        let successful_hashes = successful_transactions
            .transactions
            .keys()
            .cloned()
            .collect::<IndexSet<_>>();

        let failure_hashes = failed_transactions
            .transactions
            .keys()
            .cloned()
            .collect::<IndexSet<_>>();

        let valid = successful_hashes
            .intersection(&failure_hashes)
            .collect_vec()
            .is_empty();

        assert!(
            valid,
            "Discrepancy, found intent hash in both successful and failed transactions, this is a programmer error."
        );

        assert!(failed_transactions.is_empty() || !neglected_factor_sources.is_empty(), "Discrepancy, found failed transactions but no neglected factor sources, this is a programmer error.");

        Self {
            successful_transactions,
            failed_transactions,
            neglected_factor_sources,
        }
    }

    pub fn successful(&self) -> bool {
        self.failed_transactions.is_empty()
    }

    pub fn success_or(&self, or: Self) -> Self {
        if self.successful() {
            self.clone()
        } else {
            or
        }
    }

    pub fn signatures_of_successful_transactions(
        &self,
    ) -> IndexSet<HDSignature<ID>> {
        self.successful_transactions.all_signatures()
    }

    pub fn successful_transactions(&self) -> Vec<SignedTransaction<ID>> {
        self.successful_transactions.clone().transactions()
    }

    pub fn failed_transactions(&self) -> Vec<SignedTransaction<ID>> {
        self.failed_transactions.clone().transactions()
    }

    pub fn neglected_factor_sources(&self) -> IndexSet<NeglectedFactor> {
        self.neglected_factor_sources.clone()
    }

    #[allow(unused)]
    fn ids_of_neglected_factor_sources_filter(
        &self,
        filter: fn(&NeglectedFactor) -> bool,
    ) -> IndexSet<FactorSourceIDFromHash> {
        self.neglected_factor_sources()
            .into_iter()
            .filter(filter)
            .map(|n| n.factor_source_id())
            .collect()
    }

    #[allow(unused)]
    pub fn ids_of_neglected_factor_sources(
        &self,
    ) -> IndexSet<FactorSourceIDFromHash> {
        self.ids_of_neglected_factor_sources_filter(|_| true)
    }

    #[allow(unused)]
    pub fn ids_of_neglected_factor_sources_skipped_by_user(
        &self,
    ) -> IndexSet<FactorSourceIDFromHash> {
        self.ids_of_neglected_factor_sources_filter(|nf| {
            nf.reason == NeglectFactorReason::UserExplicitlySkipped
        })
    }

    #[allow(unused)]
    pub fn ids_of_neglected_factor_sources_failed(
        &self,
    ) -> IndexSet<FactorSourceIDFromHash> {
        self.ids_of_neglected_factor_sources_filter(|nf| {
            nf.reason == NeglectFactorReason::Failure
        })
    }

    #[allow(unused)]
    pub fn ids_of_neglected_factor_sources_irrelevant(
        &self,
    ) -> IndexSet<FactorSourceIDFromHash> {
        self.ids_of_neglected_factor_sources_filter(|nf| {
            nf.reason == NeglectFactorReason::Irrelevant
        })
    }

    #[allow(unused)]
    pub(crate) fn signatures_of_failed_transactions(
        &self,
    ) -> IndexSet<HDSignature<ID>> {
        self.failed_transactions.all_signatures()
    }

    #[allow(unused)]
    /// All signatures from both successful transactions and failed transactions.
    pub fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.signatures_of_successful_transactions()
            .union(&self.signatures_of_failed_transactions())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignaturesOutcome<TransactionIntentHash>;

    #[test]
    #[should_panic(
        expected = "Discrepancy, found intent hash in both successful and failed transactions, this is a programmer error."
    )]
    fn new_panics_if_intent_hash_is_in_both_failed_and_success_collection() {
        SUT::new(
            MaybeSignedTransactions::sample(),
            MaybeSignedTransactions::sample(),
            [],
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found failed transactions but no neglected factor sources, this is a programmer error."
    )]
    fn new_panics_if_failed_tx_is_not_empty_but_neglected_is() {
        SUT::new(
            MaybeSignedTransactions::empty(),
            MaybeSignedTransactions::sample(),
            [],
        );
    }
}
