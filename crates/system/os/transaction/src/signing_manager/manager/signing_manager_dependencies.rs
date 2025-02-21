use crate::prelude::*;

pub(crate) struct SigningManagerDependencies {
    pub(super) proto_profile: Arc<dyn IsProtoProfile>,
    pub(super) interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    pub(super) saver_of_intents_to_confirm_after_delay:
        SaveIntentsToConfirmAfterDelayClient,
}

impl SigningManagerDependencies {
    pub(crate) fn new(
        proto_profile: Arc<dyn IsProtoProfile>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        saver_of_intents_to_confirm_after_delay: SaveIntentsToConfirmAfterDelayClient,
    ) -> Self {
        Self {
            proto_profile,
            interactor,
            saver_of_intents_to_confirm_after_delay,
        }
    }
}
