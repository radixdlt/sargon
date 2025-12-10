use crate::prelude::*;

#[extend::ext]
pub impl TransactionManifest {
    /// Mutates the manifest so that the transaction fee is covered according to the
    /// current Access Controller state and the roles that can be exercised for the
    /// recovery flow. The logic is intentionally conservative:
    ///
    /// * When the fee payer is an external account (or the primary role is available)
    ///   we prefer to withdraw from that account and optionally backfill the Access
    ///   Controller vault so it is ready for future fees.
    /// * When the securified entity is also the fee payer we avoid withdrawing from
    ///   the account if doing so requires the (possibly unavailable) primary role and
    ///   instead lock the fee directly against the Access Controller XRD vault.
    /// * If the generator detects that the Access Controller vault cannot cover the
    ///   fee we leave the manifest untouched – the calling flow should prevent this
    ///   situation from being offered to the user in the first place.
    fn apply_lock_fee_instruction(
        &self,
        securified_entity_address: AddressOfAccountOrPersona,
        lock_fee_data: &LockFeeData,
        ac_state_details: &AccessControllerStateDetails,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Self {
        let fee_payer_is_securified_entity =
            securified_entity_address == lock_fee_data.fee_payer_address.into();
        let ac_vault_xrd_balance = ac_state_details.xrd_balance;
        let required_lock_fee = lock_fee_data.fee();
        let fee_payer_xrd_balance = lock_fee_data
            .fee_payer_xrd_balance
            .unwrap_or(Decimal192::zero());
        let manifest_ac_address = ScryptoManifestGlobalAddress::Static(
            ac_state_details.address.scrypto(),
        );
        let primary_role_locked = ac_state_details.state.is_primary_role_locked;

        let mut manifest = self.clone();
        let can_use_primary_role = role_combination.can_exercise_primary_role()
            && !primary_role_locked;
        let should_top_up_from_fee_payer = ac_vault_xrd_balance
            < required_lock_fee
            && fee_payer_xrd_balance > required_lock_fee.mul(Decimal192::two());

        let add_top_up_instructions =
            |manifest: TransactionManifest| -> TransactionManifest {
                TransactionManifest::modify_manifest_top_up_ac_from_securified_account(
                manifest,
                manifest_ac_address,
                lock_fee_data.fee_payer_address,
                lock_fee_data.access_controller_address,
                required_lock_fee,
                can_use_primary_role,
            ).unwrap()
            };

        if !fee_payer_is_securified_entity || can_use_primary_role {
            if should_top_up_from_fee_payer {
                manifest = add_top_up_instructions(manifest);
            }

            // The fee payer is not the securified entity, we will pay the fee from that account,
            // and no need to lock against the access controller vault, nor to top up the AC vault since XRDs from AC were not consumed.
            manifest =
                manifest.modify_add_lock_fee(lock_fee_data.clone()).unwrap();
        } else if ac_vault_xrd_balance >= required_lock_fee {
            // The fee payer has enough XRD in AC vault to cover the lock fee and it is possible to quick confirm, so users can sign with the new Primary role the withdrawal of XRDs for the lock fee.
            if !primary_role_locked
                && fee_payer_xrd_balance >= required_lock_fee
                && role_combination.can_quick_confirm()
            {
                manifest = add_top_up_instructions(manifest);
            }

            manifest = TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                manifest,
                required_lock_fee,
                ac_state_details.address,
            );
        } else {
            // This should not happen: the host should have validated that the AC vault can cover the fee.
        };

        manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn empty_manifest() -> TransactionManifest {
        TransactionManifest::empty(NetworkID::Mainnet)
    }

    fn sample_ac_state_details(
        address: AccessControllerAddress,
        balance: Decimal192,
        is_primary_role_locked: bool,
        include_recovery_attempt: bool,
    ) -> AccessControllerStateDetails {
        AccessControllerStateDetails::new(
            address,
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
                is_primary_role_locked,
                primary_role_recovery_attempt: None,
                has_primary_role_badge_withdraw_attempt: false,
                recovery_role_recovery_attempt: include_recovery_attempt
                    .then_some(sample_recovery_attempt()),
                has_recovery_role_badge_withdraw_attempt: false,
            },
            balance,
        )
    }

    fn sample_recovery_attempt() -> RecoveryRoleRecoveryAttempt {
        RecoveryRoleRecoveryAttempt {
            recovery_proposal: RecoveryProposal {
                primary_role: AccessRule::AllowAll,
                recovery_role: AccessRule::AllowAll,
                confirmation_role: AccessRule::AllowAll,
                timed_recovery_delay_minutes: None,
            },
            allow_timed_recovery_after: None,
        }
    }

    fn decimal(value: &str) -> Decimal192 {
        Decimal192::from_str(value).expect("valid decimal")
    }

    fn xrd_address_str() -> String {
        ResourceAddress::xrd_on_network(NetworkID::Mainnet).to_string()
    }

    #[test]
    fn pays_lock_fee_from_external_fee_payer_without_top_up() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet_other();
        let mut lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            fee_payer.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(Decimal192::ten());

        let securified_entity_address =
            AddressOfAccountOrPersona::from(AccountAddress::sample_mainnet());
        let ac_state_details = sample_ac_state_details(
            AccessControllerAddress::sample_mainnet_other(),
            Decimal192::ten(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{fee_payer}")
                    "lock_fee"
                    Decimal("{fee}")
                ;
                "#,
                fee_payer = lock_fee_data.fee_payer_address,
                fee = fee.to_string(),
            ),
        );
    }

    #[test]
    fn pays_lock_fee_from_external_fee_payer_with_top_up() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet_other();
        let mut lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            fee_payer.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("15"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(AccountAddress::sample_mainnet());
        let ac_address = AccessControllerAddress::sample_mainnet_other();
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::one(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{fee_payer}")
                    "lock_fee"
                    Decimal("{fee}")
                ;
                CALL_METHOD
                    Address("{fee_payer}")
                    "withdraw"
                    Address("{xrd}")
                    Decimal("{fee}")
                ;
                TAKE_FROM_WORKTOP
                    Address("{xrd}")
                    Decimal("{fee}")
                    Bucket("bucket1")
                ;
                CALL_METHOD
                    Address("{ac}")
                    "contribute_recovery_fee"
                    Bucket("bucket1")
                ;
                "#,
                fee_payer = lock_fee_data.fee_payer_address,
                fee = fee.to_string(),
                xrd = xrd_address_str(),
                ac = ac_address,
            ),
        );
    }

    #[test]
    fn securified_fee_payer_with_primary_role_pays_and_tops_up() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer.clone(),
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("15"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::one(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithPrimary,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "create_proof"
                ;
                CALL_METHOD
                    Address("{fee_payer}")
                    "lock_fee"
                    Decimal("{fee}")
                ;
                CALL_METHOD
                    Address("{ac}")
                    "create_proof"
                ;
                CALL_METHOD
                    Address("{fee_payer}")
                    "withdraw"
                    Address("{xrd}")
                    Decimal("{fee}")
                ;
                TAKE_FROM_WORKTOP
                    Address("{xrd}")
                    Decimal("{fee}")
                    Bucket("bucket1")
                ;
                CALL_METHOD
                    Address("{ac}")
                    "contribute_recovery_fee"
                    Bucket("bucket1")
                ;
                "#,
                ac = ac_address,
                fee_payer = lock_fee_data.fee_payer_address,
                fee = fee.to_string(),
                xrd = xrd_address_str(),
            ),
        );
    }

    #[test]
    fn securified_fee_payer_with_primary_role_locked_cannot_withdraw() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer.clone(),
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("15"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::ten(),
            true,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithPrimary,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "lock_recovery_fee"
                    Decimal("{fee}")
                ;
                "#,
                ac = ac_address,
                fee = fee.to_string(),
            ),
        );
    }

    #[test]
    fn securified_fee_payer_without_primary_role_locks_against_ac_with_top_up()
    {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer.clone(),
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("10"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::ten(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "lock_recovery_fee"
                    Decimal("{fee}")
                ;
                CALL_METHOD
                    Address("{ac}")
                    "create_proof"
                ;
                CALL_METHOD
                    Address("{fee_payer}")
                    "withdraw"
                    Address("{xrd}")
                    Decimal("{fee}")
                ;
                TAKE_FROM_WORKTOP
                    Address("{xrd}")
                    Decimal("{fee}")
                    Bucket("bucket1")
                ;
                CALL_METHOD
                    Address("{ac}")
                    "contribute_recovery_fee"
                    Bucket("bucket1")
                ;
                "#,
                ac = ac_address,
                fee_payer = lock_fee_data.fee_payer_address,
                fee = fee.to_string(),
                xrd = xrd_address_str(),
            ),
        );
    }

    #[test]
    fn securified_fee_payer_without_quick_confirm_skips_top_up() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer.clone(),
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("10"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::ten(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryDelayedCompletion,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "lock_recovery_fee"
                    Decimal("{fee}")
                ;
                "#,
                ac = ac_address,
                fee = fee.to_string(),
            ),
        );
    }

    #[test]
    fn securified_fee_payer_insufficient_balance_skips_top_up() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer.clone(),
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(Decimal192::one());

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address.clone(),
            Decimal192::ten(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "lock_recovery_fee"
                    Decimal("{fee}")
                ;
                "#,
                ac = ac_address,
                fee = fee.to_string(),
            ),
        );
    }

    #[test]
    fn securified_fee_payer_with_insufficient_ac_balance_keeps_manifest() {
        let manifest = empty_manifest();
        let fee = Decimal192::five();
        let fee_payer = AccountAddress::sample_mainnet();
        let ac_address = AccessControllerAddress::sample_mainnet();
        let mut lock_fee_data = LockFeeData::new_with_securified_fee_payer(
            fee_payer,
            ac_address.clone(),
            fee,
        );
        lock_fee_data.fee_payer_xrd_balance = Some(decimal("10"));

        let securified_entity_address =
            AddressOfAccountOrPersona::from(fee_payer);
        let ac_state_details = sample_ac_state_details(
            ac_address,
            Decimal192::one(),
            false,
            false,
        );

        let updated_manifest = manifest.apply_lock_fee_instruction(
            securified_entity_address,
            &lock_fee_data,
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryDelayedCompletion,
        );

        assert_eq!(updated_manifest, manifest);
    }
}
