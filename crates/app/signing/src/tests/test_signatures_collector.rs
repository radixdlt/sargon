#![allow(unused)]

use crate::prelude::*;

impl<S: Signable + 'static> SignaturesCollector<S> {
    pub(crate) fn new_test_with(
        finish_early_strategy: SigningFinishEarlyStrategy,
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IdentifiedVecOf<SignableWithEntities<S>>,
        interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Self {
        Self::with(
            finish_early_strategy,
            all_factor_sources_in_profile,
            transactions,
            interactor,
            purpose,
        )
    }

    pub(crate) fn new_test(
        finish_early_strategy: SigningFinishEarlyStrategy,
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
        simulated_user: SimulatedUser<S>,
        purpose: SigningPurpose,
    ) -> Self {
        Self::new_test_with(
            finish_early_strategy,
            all_factor_sources_in_profile.into_iter().collect(),
            IdentifiedVecOf::from_iter(transactions),
            Arc::new(TestSignInteractor::new(simulated_user)),
            purpose,
        )
    }

    pub(crate) fn test_prudent_with_factors_and_finish_early(
        finish_early_strategy: SigningFinishEarlyStrategy,
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::new_test(
            finish_early_strategy,
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::prudent_no_fail(),
            SigningPurpose::sign_transaction_primary(),
        )
    }

    pub(crate) fn test_prudent_with_finish_early(
        finish_early_strategy: SigningFinishEarlyStrategy,
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::test_prudent_with_factors_and_finish_early(
            finish_early_strategy,
            FactorSource::sample_all(),
            transactions,
        )
    }

    pub(crate) fn test_prudent(
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::test_prudent_with_finish_early(
            SigningFinishEarlyStrategy::default(),
            transactions,
        )
    }

    pub(crate) fn test_prudent_with_failures(
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
        simulated_failures: SimulatedFailures,
    ) -> Self {
        Self::new_test(
            SigningFinishEarlyStrategy::default(),
            FactorSource::sample_all(),
            transactions,
            SimulatedUser::prudent_with_failures(simulated_failures),
            SigningPurpose::sign_transaction_primary(),
        )
    }

    pub(crate) fn test_lazy_sign_minimum_no_failures_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::new_test(
            SigningFinishEarlyStrategy::default(),
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::lazy_sign_minimum([]),
            SigningPurpose::sign_transaction_primary(),
        )
    }

    pub(crate) fn test_lazy_sign_minimum_no_failures(
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::test_lazy_sign_minimum_no_failures_with_factors(
            FactorSource::sample_all(),
            transactions,
        )
    }

    pub(crate) fn test_lazy_always_skip_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::new_test(
            SigningFinishEarlyStrategy::default(),
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::lazy_always_skip_no_fail(),
            SigningPurpose::sign_transaction_primary(),
        )
    }

    pub(crate) fn test_lazy_always_skip(
        transactions: impl IntoIterator<Item = SignableWithEntities<S>>,
    ) -> Self {
        Self::test_lazy_always_skip_with_factors(
            FactorSource::sample_all(),
            transactions,
        )
    }
}
