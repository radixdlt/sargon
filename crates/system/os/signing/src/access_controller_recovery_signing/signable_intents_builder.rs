use crate::prelude::*;

pub struct AccessControllerRecoveryIntentsBuilder {
    base_intent: TransactionIntent,
    // The lock fee data for the above intent
    lock_fee_data: LockFeeData,
    securified_entity: AnySecurifiedEntity,
    proposed_security_structure: SecurityStructureOfFactorInstances,
    fee_payer_account: Option<Account>,
}

impl AccessControllerRecoveryIntentsBuilder {
    pub fn new(
        base_intent: TransactionIntent,
        lock_fee_data: LockFeeData,
        securified_entity: AnySecurifiedEntity,
        proposed_security_structure: SecurityStructureOfFactorInstances,
        fee_payer_account: Option<Account>,
    ) -> Self {
        Self {
            base_intent,
            lock_fee_data,
            securified_entity,
            proposed_security_structure,
            fee_payer_account,
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
            self.fee_payer_account.clone(),
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

        manifest = manifest.modify_add_lock_fee(self.lock_fee_data.clone())?;

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
    fee_payer_account: Option<Account>,
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
        fee_payer_account: Option<Account>,
    ) -> Self {
        Self {
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            fee_payer_account,
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

    pub fn fee_payer_account(&self) -> Option<&Account> {
        self.fee_payer_account.as_ref()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_builder_inputs() -> (
        TransactionIntent,
        LockFeeData,
        AnySecurifiedEntity,
        SecurityStructureOfFactorInstances,
    ) {
        let base_intent = TransactionIntent::sample();
        let securified_entity = AnySecurifiedEntity::sample_account();
        let security_structure = securified_entity
            .securified_entity_control()
            .security_structure
            .clone();
        let lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            AccountAddress::sample_mainnet_other(),
            Decimal192::one(),
        );

        (
            base_intent,
            lock_fee_data,
            securified_entity,
            security_structure,
        )
    }

    #[test]
    fn no_fee_payer_account_produces_none() {
        let (base_intent, lock_fee_data, securified_entity, security_structure) =
            sample_builder_inputs();

        let intents = AccessControllerRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data,
            securified_entity,
            security_structure,
            None,
        )
        .build()
        .expect("builder should succeed");

        assert!(intents.fee_payer_account().is_none());
        let hash = intents
            .initiate_with_recovery_complete_with_confirmation
            .id
            .clone();
        assert!(intents.signable_for_hash(&hash).is_some());
    }

    #[test]
    fn fee_payer_account_is_preserved_and_signables_lookup() {
        let (base_intent, lock_fee_data, securified_entity, security_structure) =
            sample_builder_inputs();
        let fee_payer_account = Account::sample_mainnet_third();

        let intents = AccessControllerRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data,
            securified_entity,
            security_structure,
            Some(fee_payer_account.clone()),
        )
        .build()
        .expect("builder should succeed");

        let stored_account = intents.fee_payer_account().unwrap();
        assert_eq!(stored_account.address, fee_payer_account.address);

        let hash = intents
            .initiate_with_primary_complete_with_confirmation
            .id
            .clone();
        let signable = intents
            .signable_for_hash(&hash)
            .expect("signable for hash should exist");
        assert_eq!(signable.id, hash);
    }
}
