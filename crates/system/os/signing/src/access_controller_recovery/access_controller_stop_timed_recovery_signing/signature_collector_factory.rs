use crate::prelude::*;

pub struct StopTimedRecoverySignaturesCollectorFactory {
    profile: Profile,
    pub(crate) intents: AccessControllerStopTimedRecoveryIntents,
    collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn new(
        base_intent: TransactionIntent,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        profile: Profile,
        lock_fee_data: LockFeeData,
        ac_state_details: AccessControllerStateDetails,
    ) -> Result<Self> {
        Self::with_collector_builder(
            base_intent,
            profile,
            lock_fee_data,
            ac_state_details,
            Arc::new(DefaultTransactionIntentSignaturesCollectorBuilder::new(interactor)),
        )
    }

    pub fn with_collector_builder(
        base_intent: TransactionIntent,
        profile: Profile,
        lock_fee_data: LockFeeData,
        ac_state_details: AccessControllerStateDetails,
        collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
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
            profile,
            intents,
            collector_builder,
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
    ) -> Result<Option<Box<dyn TransactionIntentSignaturesCollector>>> {
        Ok(None)
    }
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn stop_and_cancel_with_recovery_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop_and_cancel.clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Recovery,
            },
        )
    }

    pub fn stop_with_primary_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop.clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Primary,
            },
        )
    }

    pub fn stop_with_confirmation_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            IdentifiedVecOf::from(vec![self.intents.stop.clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Confirmation,
            },
        )
    }
}
