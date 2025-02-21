use crate::prelude::*;

pub(crate) struct SigningManagerDependencies {
    /// FactorSources in Profile
    pub(super) factor_sources_in_profile: IndexSet<FactorSource>,
    pub(super) get_entities_by_address: Arc<dyn GetEntityByAddress>,
    pub(super) interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    pub(super) saver_of_intents_to_confirm_after_delay:
        SaveIntentsToConfirmAfterDelayClient,
}

impl SigningManagerDependencies {
    pub(crate) fn new(
        factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        get_entities_by_address: Arc<dyn GetEntityByAddress>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        saver_of_intents_to_confirm_after_delay: SaveIntentsToConfirmAfterDelayClient,
    ) -> Self {
        Self {
            factor_sources_in_profile: IndexSet::from_iter(
                factor_sources_in_profile,
            ),
            get_entities_by_address,
            interactor,
            saver_of_intents_to_confirm_after_delay,
        }
    }
}
