use crate::prelude::*;

pub struct StopTimedRecoverySignaturesCollectorFactory {
    pub(crate) intents: AccessControllerStopTimedRecoveryIntents,
    collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
    lock_fee_data: LockFeeData,
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn new(
        base_intent: TransactionIntent,
        securified_entity: AnySecurifiedEntity,
        lock_fee_data: LockFeeData,
        ac_state_details: AccessControllerStateDetails,
        collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
    ) -> Result<Self> {
        let intents = AccessControllerStopTimedRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data.clone(),
            securified_entity.clone(),
        )
        .build()?;

        Ok(Self {
            intents,
            collector_builder,
            lock_fee_data,
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
        let fee_payer_account = self.profile.account_by_address(
            self.lock_fee_data.fee_payer_address.clone(),
        )?;
    }
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn stop_and_cancel_with_recovery_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
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
            IdentifiedVecOf::from(vec![self.intents.stop.clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Confirmation,
            },
        )
    }
}
