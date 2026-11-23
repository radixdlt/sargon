use crate::prelude::*;

pub struct StopTimedRecoverySignaturesCollectorFactory {
    pub(crate) intents: AccessControllerStopTimedRecoveryIntents,
    collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
    securified_entity: AnySecurifiedEntity,
    /// The account that pays the fee, unless the access controller does.
    fee_payer_account: Option<Account>,
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn new(
        base_intent: TransactionIntent,
        securified_entity: AnySecurifiedEntity,
        fee_payer: AccessControllerTransactionFeePayer,
        fee_payer_account: Option<Account>,
        ac_state_details: AccessControllerStateDetails,
        collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder>,
    ) -> Result<Self> {
        let intents = AccessControllerStopTimedRecoveryIntentsBuilder::new(
            base_intent,
            fee_payer,
            securified_entity.clone(),
            ac_state_details,
        )
        .build()?;

        Ok(Self {
            intents,
            collector_builder,
            securified_entity,
            fee_payer_account,
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
        intent_hash: &TransactionIntentHash,
    ) -> Result<Option<Box<dyn TransactionIntentSignaturesCollector>>> {
        // The access controller pays: no account needs to sign for it.
        let Some(fee_payer_account) = &self.fee_payer_account else {
            return Ok(None);
        };
        let fee_payer_is_securified_account =
            fee_payer_account.address().scrypto()
                == self.securified_entity.address().scrypto();
        let used_role_combination =
            self.intents.role_kind_for_intent_hash(intent_hash);
        let intent = self
            .intent_for_hash(intent_hash)
            .expect("Intent should exist");

        if self.securified_entity.entity.is_account_entity()
            && fee_payer_is_securified_account
            && used_role_combination == RoleKind::Primary
        {
            // Already signed with Primary, no need for a new signature
            return Ok(None);
        };

        let fee_payer_signable =
            SignableWithEntities::with(intent, vec![fee_payer_account.clone()]);

        Ok(Some(self.collector_builder.build(
            IdentifiedVecOf::from(vec![fee_payer_signable]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Primary,
            },
        )))
    }
}

impl StopTimedRecoverySignaturesCollectorFactory {
    pub fn stop_and_cancel_with_recovery_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IdentifiedVecOf::from(vec![self
                .intents
                .stop_with_recovery
                .clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Recovery,
            },
        )
    }

    pub fn stop_with_primary_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IdentifiedVecOf::from(vec![self.intents.stop_with_primary.clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Primary,
            },
        )
    }

    pub fn stop_with_confirmation_role_collector(
        &self,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        self.collector_builder.build(
            IdentifiedVecOf::from(vec![self
                .intents
                .stop_with_confirmation
                .clone()]),
            SigningPurpose::SignTX {
                role_kind: RoleKind::Confirmation,
            },
        )
    }
}
