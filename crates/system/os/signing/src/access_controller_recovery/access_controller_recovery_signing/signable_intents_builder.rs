use crate::prelude::*;

pub struct AccessControllerRecoveryIntentsBuilder {
    base_intent: TransactionIntent,
    // The lock fee data for the above intent
    lock_fee_data: LockFeeData,
    securified_entity: AnySecurifiedEntity,
    proposed_security_structure: SecurityStructureOfFactorInstances,
    ac_state_details: AccessControllerStateDetails,
}

impl AccessControllerRecoveryIntentsBuilder {
    pub fn new(
        base_intent: TransactionIntent,
        lock_fee_data: LockFeeData,
        securified_entity: AnySecurifiedEntity,
        proposed_security_structure: SecurityStructureOfFactorInstances,
        ac_state_details: AccessControllerStateDetails,
    ) -> Self {
        Self {
            base_intent,
            lock_fee_data,
            securified_entity,
            proposed_security_structure,
            ac_state_details,
        }
    }
}

impl AccessControllerRecoveryIntentsBuilder {
    pub fn build(&self) -> Result<AccessControllerRecoveryIntents> {
        let with_recovery_confirmation =
            self.signable_for_role_combination(
                RolesExercisableInTransactionManifestCombination::
                    InitiateWithRecoveryCompleteWithConfirmation,
            )?;
        let with_recovery_primary = self.signable_for_role_combination(
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithPrimary,
        )?;
        let with_recovery_delayed = self.signable_for_role_combination(
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryDelayedCompletion,
        )?;
        let with_primary_confirmation = self.signable_for_role_combination(
            RolesExercisableInTransactionManifestCombination::
                InitiateWithPrimaryCompleteWithConfirmation,
        )?;

        Ok(AccessControllerRecoveryIntents::new(
            with_recovery_confirmation,
            with_recovery_primary,
            with_recovery_delayed,
            with_primary_confirmation,
        ))
    }

    fn signable_for_role_combination(
        &self,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SignableWithEntities<TransactionIntent>> {
        let mut manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                self.securified_entity.clone(),
                self.proposed_security_structure.clone(),
                role_combination,
            );

        manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &self.ac_state_details,
            role_combination,
        );
        manifest = manifest.apply_lock_fee_instruction(
            self.securified_entity.address(),
            &self.lock_fee_data,
            &self.ac_state_details,
            role_combination,
        );

        let intent = TransactionIntent::new(
            self.base_intent.header,
            manifest,
            self.base_intent.message.clone(),
        )?;

        Ok(SignableWithEntities::with(
            intent,
            vec![self.securified_entity.entity.clone()],
        ))
    }
}

pub struct AccessControllerRecoveryIntents {
    pub initiate_with_recovery_complete_with_confirmation:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_recovery_complete_with_primary:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_recovery_delayed_completion:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_primary_complete_with_confirmation:
        SignableWithEntities<TransactionIntent>,
}

impl AccessControllerRecoveryIntents {
    pub fn new(
        initiate_with_recovery_complete_with_confirmation: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_recovery_complete_with_primary: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_recovery_delayed_completion: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_primary_complete_with_confirmation: SignableWithEntities<
            TransactionIntent,
        >,
    ) -> Self {
        Self {
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
        }
    }

    pub fn all_signables(
        &self,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        IdentifiedVecOf::from(vec![
            self.initiate_with_recovery_complete_with_confirmation
                .clone(),
            self.initiate_with_recovery_complete_with_primary.clone(),
            self.initiate_with_recovery_delayed_completion.clone(),
            self.initiate_with_primary_complete_with_confirmation
                .clone(),
        ])
    }

    pub fn signable_for_hash(
        &self,
        id: &TransactionIntentHash,
    ) -> Option<SignableWithEntities<TransactionIntent>> {
        [
            &self.initiate_with_recovery_complete_with_confirmation,
            &self.initiate_with_recovery_complete_with_primary,
            &self.initiate_with_recovery_delayed_completion,
            &self.initiate_with_primary_complete_with_confirmation,
        ]
        .into_iter()
        .find(|signable| signable.id == *id)
        .map(|signable| signable.clone())
    }

    pub fn role_combination_used_for_transaction(
        &self,
        id: &TransactionIntentHash,
    ) -> RolesExercisableInTransactionManifestCombination {
        if self.initiate_with_recovery_complete_with_confirmation.id == *id {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation
        } else if self.initiate_with_recovery_complete_with_primary.id == *id {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary
        } else if self.initiate_with_recovery_delayed_completion.id == *id {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion
        } else {
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation
        }
    }
}
