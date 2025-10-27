use crate::prelude::*;

pub struct StopTimedRecoverySignaturesCollectorFactory {
    finish_early_strategy: SigningFinishEarlyStrategy,
    profile: Profile,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    intents: AccessControllerStopTimedRecoveryIntents,
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn new(
        base_intent: TransactionIntent,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        profile: Profile,
        lock_fee_data: LockFeeData,
        ac_state_details: AccessControllerStateDetails,
    ) -> Result<Self> {
        let entity = profile
            .entity_by_access_controller_address(ac_state_details.address)?;
        let securified_entity = AnySecurifiedEntity::try_from(entity)?;

        let intents = AccessControllerStopTimedRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data.clone(),
            securified_entity.clone(),
        )
        .build()?;

        Ok(Self {
            finish_early_strategy: SigningFinishEarlyStrategy::default(),
            profile,
            interactor,
            intents,
        })
    }

    pub fn intent_for_hash(
        &self,
        hash: &TransactionIntentHash,
    ) -> Option<TransactionIntent> {
        self.intents
            .all_signables()
            .into_iter()
            .find(|signable| signable.id == *hash)
            .map(|signable| signable.signable.clone())
    }

    pub fn signature_collector_for_post_processing_signatures(
        &self,
        _intent_hash: &TransactionIntentHash,
    ) -> Result<Option<SignaturesCollector<TransactionIntent>>> {
        Ok(None)
    }
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn stop_and_cancel_with_recovery_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            self.finish_early_strategy.clone(),
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop_and_cancel.clone()]),
            self.interactor.clone(),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Recovery,
            },
        )
    }

    pub fn stop_with_primary_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            self.finish_early_strategy.clone(),
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop.clone()]),
            self.interactor.clone(),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Primary,
            },
        )
    }

    pub fn stop_with_confirmation_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            self.finish_early_strategy.clone(),
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop.clone()]),
            self.interactor.clone(),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Confirmation,
            },
        )
    }
}
