use crate::prelude::*;

impl SargonOS {
    pub async fn make_transaction_manifest_for_securified_entity(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        entity: AccountOrPersona,
        entity_sec_structure: SecurityStructureOfFactorInstances,
    ) -> Result<TransactionManifest> {
        let ac_address = entity
            .security_state()
            .as_securified()
            .ok_or(CommonError::Unknown {
                error_message: "The entity must be securified".to_owned(),
            })?
            .access_controller_address;
        // Read from the ledger, not the cache: the user may just have
        // cancelled the pending recovery.
        ensure_no_recovery_is_pending(
            &self
                .gateway_client()
                .fetch_access_controller_details(ac_address)
                .await?,
        )?;

        let profile_snapshot = self.profile()?;

        // Read the current configuration
        let matrix_of_factor_source_ids =
            SecurityStructureOfFactorSourceIds::from(
                entity_sec_structure.clone(),
            );

        let current_factor_sources: SecurityStructureOfFactorSources = (
            &matrix_of_factor_source_ids,
            &profile_snapshot.factor_sources,
        )
            .try_into()?;

        // Derive new instance for rola only if it changed
        let new_rola_factor = if security_structure
            .authentication_signing_factor
            != current_factor_sources.authentication_signing_factor
        {
            Some(security_structure.authentication_signing_factor.clone())
        } else {
            None
        };

        // Get the newly added factors
        let new_factors: HashSet<&FactorSource> = security_structure
            .matrix_of_factors
            .all_factors()
            .difference(&current_factor_sources.matrix_of_factors.all_factors())
            .copied()
            .collect();

        // Create instances for new factors
        let mut factor_instances = self
            .derive_factor_instances(
                entity.clone(),
                new_factors,
                new_rola_factor,
            )
            .await?;

        // Merge kept factor instances with new factor instances.

        let new_factor_source_ids = SecurityStructureOfFactorSourceIds::from(
            security_structure.clone(),
        );
        let kept_instances = entity_sec_structure
            .unique_all_factor_instances()
            .into_iter()
            .filter(|instance| {
                new_factor_source_ids
                    .all_factors()
                    .contains(&instance.factor_source_id)
            })
            .collect::<IndexSet<FactorInstance>>();

        for instance in kept_instances {
            factor_instances.append_or_insert_element_to(
                // For now we do work only with factor source which are derived from factor as a hash
                instance.factor_source_id.into_hash().unwrap(),
                // For now we do work only with hd factor instances
                instance.try_as_hd_factor_instances().unwrap(),
            );
        }

        // Create the updated security structure of instances
        let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
            &mut factor_instances,
            None,
            &security_structure
        )?;

        // 4. Set the security structure as provisional, this will be extracted on transaction analysis
        let mut entity = entity;
        entity.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: security_structure_of_factor_instances.clone(),
            },
        );

        self.update_entities_erased(vec![entity.clone()].into())
            .await?;
        for factor_source_id in factor_instances.keys() {
            self.update_last_used_of_factor_source(*factor_source_id)
                .await?
        }

        // Create the transaction manifest next for R+T.
        // Hardcoded for now to unblock the work for implementing Timed Recovery flows.
        let role_combination = RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion;

        let securified_entity_control =
            entity.security_state().into_securified().unwrap();

        Ok(
            TransactionManifest::apply_security_shield_for_securified_entity(
                AnySecurifiedEntity::with_securified_entity_control(
                    entity.clone(),
                    securified_entity_control,
                ),
                security_structure_of_factor_instances,
                role_combination,
            ),
        )
    }
}

/// New rules can't be proposed while a recovery is pending: a role can't
/// propose while its own proposal is pending, and the wallet signs whichever
/// role combination the user's factors can complete. The user reviews and
/// cancels the pending recovery first, in a transaction of its own, rather
/// than it being cancelled unseen in the one proposing new rules.
fn ensure_no_recovery_is_pending(
    access_controller: &AccessControllerStateDetails,
) -> Result<()> {
    let state = &access_controller.state;
    if state.recovery_role_recovery_attempt.is_some()
        || state.primary_role_recovery_attempt.is_some()
    {
        Err(CommonError::AccessControllerRecoveryProposalPending)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn access_controller(
        primary_role_proposal: bool,
        recovery_role_proposal: bool,
    ) -> AccessControllerStateDetails {
        let proposal = || RecoveryProposal {
            primary_role: AccessRule::AllowAll,
            recovery_role: AccessRule::AllowAll,
            confirmation_role: AccessRule::AllowAll,
            timed_recovery_delay_minutes: None,
        };
        AccessControllerStateDetails::new(
            AccessControllerAddress::sample(),
            AccessControllerFieldStateValue {
                primary_role_recovery_attempt: primary_role_proposal.then(
                    || PrimaryRoleRecoveryAttempt {
                        recovery_proposal: proposal(),
                    },
                ),
                recovery_role_recovery_attempt: recovery_role_proposal.then(
                    || RecoveryRoleRecoveryAttempt {
                        recovery_proposal: proposal(),
                        allow_timed_recovery_after: None,
                    },
                ),
                ..AccessControllerFieldStateValue::sample()
            },
            Decimal192::zero(),
        )
    }

    #[test]
    fn new_rules_can_be_proposed_without_a_pending_recovery() {
        assert_eq!(
            ensure_no_recovery_is_pending(&access_controller(false, false)),
            Ok(())
        );
    }

    #[test]
    fn a_pending_recovery_of_either_role_must_be_cancelled_first() {
        for (primary, recovery) in [(true, false), (false, true), (true, true)]
        {
            assert_eq!(
                ensure_no_recovery_is_pending(&access_controller(
                    primary, recovery
                )),
                Err(CommonError::AccessControllerRecoveryProposalPending)
            );
        }
    }
}
