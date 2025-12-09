use crate::prelude::*;

pub struct SignaturesCollectorFactory {
    finish_early_strategy: SigningFinishEarlyStrategy,
    profile: Profile,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    recovery_intents: AccessControllerRecoveryIntents,
    securified_entity: AnySecurifiedEntity,
    proposed_security_structure: SecurityStructureOfFactorInstances,
    lock_fee_data: LockFeeData,
    ac_state_details: AccessControllerStateDetails,
}

impl SignaturesCollectorFactory {
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

        let proposed_security_structure = securified_entity
            .securified_entity_control
            .provisional_securified_config
            .clone()
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?
            .get_security_structure_of_factor_instances();

        let recovery_intents = AccessControllerRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data.clone(),
            securified_entity.clone(),
            proposed_security_structure.clone(),
            ac_state_details.clone(),
        )
        .build()?;

        Ok(Self {
            finish_early_strategy: SigningFinishEarlyStrategy::default(),
            profile,
            interactor,
            recovery_intents,
            securified_entity,
            proposed_security_structure,
            lock_fee_data,
            ac_state_details,
        })
    }

    pub fn intent_for_hash(
        &self,
        hash: &TransactionIntentHash,
    ) -> Option<TransactionIntent> {
        self.recovery_intents
            .all_signables()
            .into_iter()
            .find(|signable| signable.id == *hash)
            .map(|signable| signable.signable.clone())
    }

    pub fn intent_hash_of_timed_recovery(&self) -> TransactionIntentHash {
        self.recovery_intents
            .initiate_with_recovery_delayed_completion
            .id()
    }

    /// Returns true if there's an existing recovery attempt that must be cancelled.
    /// When this is true, the Primary-initiated flow cannot succeed and should be skipped,
    /// because only the Recovery role can cancel an existing recovery proposal.
    pub fn has_existing_recovery_attempt(&self) -> bool {
        self.ac_state_details
            .state
            .recovery_role_recovery_attempt
            .is_some()
    }
}

impl SignaturesCollectorFactory {
    pub fn initiate_recovery_with_recovery_sign_with_recovery_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Recovery,
        )
    }

    pub fn initiate_recovery_with_recovery_sign_with_confirmation_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Confirmation,
        )
    }

    pub fn initiate_recovery_with_recovery_sign_with_primary_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Primary,
        )
    }

    pub fn initiate_recovery_with_primary_sign_with_primary_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Primary,
            RoleKind::Primary,
        )
    }

    pub fn initiate_recovery_with_primary_sign_with_confirmation_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Primary,
            RoleKind::Confirmation,
        )
    }

    pub fn signature_collector_for_post_processing_signatures(
        &self,
        intent_hash: &TransactionIntentHash,
    ) -> Result<Option<SignaturesCollector<TransactionIntent>>> {
        let fee_payer_is_securified_account =
            self.lock_fee_data.fee_payer_address.scrypto()
                == self.securified_entity.address().scrypto();
        let used_role_combination = self
            .recovery_intents
            .role_combination_used_for_transaction(intent_hash);
        let intent = self
            .recovery_intents
            .signable_for_hash(intent_hash)
            .expect("Programmer error: Signable should exist")
            .signable;
        let authentication_role_is_updated = self
            .securified_entity
            .current_authentication_signing_factor_instance()
            != self
                .proposed_security_structure
                .authentication_signing_factor_instance;

        let mut signable_entities: Vec<Account> = Vec::<_>::new();

        // First handle the the case when the fee payer is another account
        if !fee_payer_is_securified_account {
            // Fee payer is some other account;
            let fee_payer_account = self.profile.account_by_address(
                self.lock_fee_data.fee_payer_address.clone(),
            )?;
            signable_entities.push(fee_payer_account);
        }

        // Second, handle the scenarios when it is needed to sign with the new Primary Role
        if self.securified_entity.entity.is_account_entity()
            && used_role_combination.can_quick_confirm()
            && !used_role_combination.can_exercise_primary_role()
        {
            // The recovery was quick confirmed, but the current Primary role was not used, so try to use the new Primary role if needed.
            if fee_payer_is_securified_account || authentication_role_is_updated
            {
                // Need to create a copy of the securified entity and commit the provisional security structure,
                // so the SignaturesCollector can use it when signing with Primary role.
                let mut securified_entity = self.securified_entity.clone();
                securified_entity.commit_provisional()?;

                signable_entities.push(
                    securified_entity
                    .entity
                    .as_account_entity()
                    .expect("Safey to unwrap, since we do check earlier if the entity is account")
                    .clone()
                );
            }
        }

        if signable_entities.is_empty() {
            Ok(None)
        } else {
            Ok(Some(SignaturesCollector::with(
                self.finish_early_strategy.clone(),
                IndexSet::from_iter(self.profile.factor_sources.iter()),
                IdentifiedVecOf::from(vec![SignableWithEntities::with(
                    intent,
                    signable_entities,
                )]),
                self.interactor.clone(),
                SigningPurpose::SignTX {
                    role_kind: RoleKind::Primary,
                },
            )))
        }
    }

    fn signature_collector_for_recovery_signing(
        &self,
        recovery_proposer_kind: RoleKind,
        signing_kind: RoleKind,
    ) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            self.finish_early_strategy.clone(),
            IndexSet::from_iter(self.profile.factor_sources.iter()),
            self.transaction_intents_for_recovery_signing(
                recovery_proposer_kind,
                signing_kind,
            ),
            self.interactor.clone(),
            SigningPurpose::SignTX {
                role_kind: signing_kind,
            },
        )
    }
}

impl SignaturesCollectorFactory {
    fn transaction_intents_for_recovery_signing(
        &self,
        recovery_proposer_kind: RoleKind,
        signing_kind: RoleKind,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        match recovery_proposer_kind {
            RoleKind::Recovery => match signing_kind {
                RoleKind::Recovery => IdentifiedVecOf::from(vec![
                    self.recovery_intents
                        .initiate_with_recovery_complete_with_confirmation
                        .clone(),
                    self.recovery_intents
                        .initiate_with_recovery_complete_with_primary
                        .clone(),
                    self.recovery_intents
                        .initiate_with_recovery_delayed_completion
                        .clone(),
                ]),
                RoleKind::Primary => IdentifiedVecOf::from(vec![self
                    .recovery_intents
                    .initiate_with_recovery_complete_with_primary
                    .clone()]),
                RoleKind::Confirmation => IdentifiedVecOf::from(vec![self
                    .recovery_intents
                    .initiate_with_recovery_complete_with_confirmation
                    .clone()]),
            },
            RoleKind::Primary => IdentifiedVecOf::from(vec![self
                .recovery_intents
                .initiate_with_primary_complete_with_confirmation
                .clone()]),
            RoleKind::Confirmation => {
                panic!("Confirmation role cannot iniate recovery")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_existing_recovery_attempt_returns_true_when_attempt_exists() {
        let ac_state_details = AccessControllerStateDetails::new(
            AccessControllerAddress::sample_mainnet(),
            AccessControllerFieldStateValue {
                controlled_vault: EntityReference {
                    entity_type: CoreApiEntityType::InternalFungibleVault,
                    is_global: false,
                    entity_address: "internal_vault".to_owned(),
                },
                xrd_fee_vault: None,
                timed_recovery_delay_minutes: None,
                recovery_badge_resource_address:
                    ResourceAddress::sample_mainnet(),
                is_primary_role_locked: false,
                primary_role_recovery_attempt: None,
                has_primary_role_badge_withdraw_attempt: false,
                recovery_role_recovery_attempt: Some(
                    RecoveryRoleRecoveryAttempt {
                        recovery_proposal: RecoveryProposal {
                            primary_role: AccessRule::AllowAll,
                            recovery_role: AccessRule::AllowAll,
                            confirmation_role: AccessRule::AllowAll,
                            timed_recovery_delay_minutes: None,
                        },
                        allow_timed_recovery_after: None,
                    },
                ),
                has_recovery_role_badge_withdraw_attempt: false,
            },
            Decimal192::ten(),
        );

        assert!(ac_state_details
            .state
            .recovery_role_recovery_attempt
            .is_some());
    }

    #[test]
    fn has_existing_recovery_attempt_returns_false_when_no_attempt() {
        let ac_state_details = AccessControllerStateDetails::new(
            AccessControllerAddress::sample_mainnet(),
            AccessControllerFieldStateValue {
                controlled_vault: EntityReference {
                    entity_type: CoreApiEntityType::InternalFungibleVault,
                    is_global: false,
                    entity_address: "internal_vault".to_owned(),
                },
                xrd_fee_vault: None,
                timed_recovery_delay_minutes: None,
                recovery_badge_resource_address:
                    ResourceAddress::sample_mainnet(),
                is_primary_role_locked: false,
                primary_role_recovery_attempt: None,
                has_primary_role_badge_withdraw_attempt: false,
                recovery_role_recovery_attempt: None,
                has_recovery_role_badge_withdraw_attempt: false,
            },
            Decimal192::ten(),
        );

        assert!(ac_state_details
            .state
            .recovery_role_recovery_attempt
            .is_none());
    }
}
