use crate::prelude::*;

pub(crate) struct SigningManagerDependencies {
    /// FactorSources in Profile
    pub(super) factor_sources_in_profile: IndexSet<FactorSource>,
    pub(super) interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    pub(super) saver_of_intents_to_confirm_after_delay:
        Arc<dyn SaverOfIntentsToConfirmAfterDelay>,
}

impl SigningManagerDependencies {
    pub(crate) fn new(
        factor_sources_in_profile: IndexSet<FactorSource>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        saver_of_intents_to_confirm_after_delay: Arc<
            dyn SaverOfIntentsToConfirmAfterDelay,
        >,
    ) -> Self {
        Self {
            factor_sources_in_profile,
            interactor,
            saver_of_intents_to_confirm_after_delay,
        }
    }
}
