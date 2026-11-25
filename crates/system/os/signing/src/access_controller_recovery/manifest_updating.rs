use crate::prelude::*;

/// Who pays the fee of an access controller transaction the wallet signs, as
/// the user picked it when reviewing: an account, or the XRD vault of an
/// access controller. The wallet never switches to the access controller on
/// its own.
#[derive(Clone, PartialEq)]
pub enum AccessControllerTransactionFeePayer {
    /// An account, with the access controller that controls it if securified.
    Account(LockFeeData),
    /// The XRD vault of an access controller.
    AccessController {
        address: AccessControllerAddress,
        fee: Decimal192,
    },
}

impl AccessControllerTransactionFeePayer {
    /// The fee payer the user picked for `intent`, which must lock the fee.
    pub fn of(intent: &TransactionIntent, profile: &Profile) -> Result<Self> {
        match intent.manifest.extract_fee_payer() {
            Some(FeePayerOfManifest::Account { address, fee }) => {
                Ok(Self::Account(LockFeeData::new_with_account(
                    profile.account_by_address(address)?,
                    fee,
                    Decimal192::zero(), // The balance isn't needed.
                )))
            }
            Some(FeePayerOfManifest::AccessController { address, fee }) => {
                Ok(Self::AccessController { address, fee })
            }
            None => Err(CommonError::Unknown {
                error_message: "The transaction has no fee payer".to_owned(),
            }),
        }
    }

    /// The account that pays, unless an access controller does.
    pub fn account(&self) -> Option<AccountAddress> {
        match self {
            Self::Account(lock_fee_data) => {
                Some(lock_fee_data.fee_payer_address)
            }
            Self::AccessController { .. } => None,
        }
    }
}

/// Where the fee of a recovery is locked, for a role combination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeeLockPlacement {
    /// Before the recovery: the payer is the access controller, another
    /// account, or the account being recovered with its current primary role.
    BeforeRecovery,
    /// After the recovery is quick confirmed: the account being recovered
    /// pays with a proof of its new primary role.
    AfterConfirmation,
    /// The account being recovered pays, but neither its current primary role
    /// nor a new one is available: the user must pick another fee payer.
    Impossible,
}

impl FeeLockPlacement {
    pub fn new(
        fee_payer: &AccessControllerTransactionFeePayer,
        securified_entity_address: AddressOfAccountOrPersona,
        primary_role_locked: bool,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Self {
        let pays_for_itself = fee_payer
            .account()
            .is_some_and(|account| securified_entity_address == account.into());
        let can_use_primary_role = role_combination.can_exercise_primary_role()
            && !primary_role_locked;

        if !pays_for_itself || can_use_primary_role {
            Self::BeforeRecovery
        } else if role_combination.can_quick_confirm() {
            Self::AfterConfirmation
        } else {
            Self::Impossible
        }
    }
}

#[extend::ext]
pub impl TransactionManifest {
    /// Locks the fee of a recovery manifest from the fee payer the user
    /// picked, see [`FeeLockPlacement`]. It's the only thing added besides the
    /// recovery: the access controller's vault isn't topped up, since a
    /// manifest that also moves XRD isn't classified as a recovery, so its
    /// review wouldn't show what it does.
    fn apply_lock_fee_instruction(
        &self,
        fee_payer: &AccessControllerTransactionFeePayer,
        placement: FeeLockPlacement,
    ) -> Result<TransactionManifest> {
        match fee_payer {
            AccessControllerTransactionFeePayer::AccessController {
                address,
                fee,
            } => Ok(TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                self.clone(),
                *fee,
                *address,
            )),
            AccessControllerTransactionFeePayer::Account(lock_fee_data) => {
                match placement {
                    FeeLockPlacement::BeforeRecovery
                    | FeeLockPlacement::Impossible => {
                        self.modify_add_lock_fee(lock_fee_data.clone())
                    }
                    FeeLockPlacement::AfterConfirmation => {
                        let lock_fee = TransactionManifest::empty(
                            self.network_id(),
                        )
                        .modify_add_lock_fee(lock_fee_data.clone())?;
                        let builder =
                            ScryptoTransactionManifestBuilder::new()
                                .extend_builder_with_manifest(self.clone())
                                .extend_builder_with_manifest(lock_fee);
                        Ok(TransactionManifest::sargon_built(
                            builder,
                            self.network_id(),
                        ))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RolesExercisableInTransactionManifestCombination::*;

    const COMBINATIONS: [RolesExercisableInTransactionManifestCombination; 4] = [
        InitiateWithRecoveryCompleteWithConfirmation,
        InitiateWithRecoveryCompleteWithPrimary,
        InitiateWithRecoveryDelayedCompletion,
        InitiateWithPrimaryCompleteWithConfirmation,
    ];

    fn securified_account() -> SecurifiedAccount {
        SecurifiedAccount::sample()
    }

    fn paid_by_itself() -> AccessControllerTransactionFeePayer {
        let account = securified_account();
        AccessControllerTransactionFeePayer::Account(
            LockFeeData::new_with_securified_fee_payer(
                account.entity.address,
                account.access_controller_address(),
                Decimal192::five(),
            ),
        )
    }

    fn paid_by_another_account() -> AccessControllerTransactionFeePayer {
        AccessControllerTransactionFeePayer::Account(
            LockFeeData::new_with_unsecurified_fee_payer(
                AccountAddress::sample_other(),
                Decimal192::five(),
            ),
        )
    }

    fn paid_by_access_controller() -> AccessControllerTransactionFeePayer {
        AccessControllerTransactionFeePayer::AccessController {
            address: securified_account().access_controller_address(),
            fee: Decimal192::five(),
        }
    }

    fn placement(
        fee_payer: &AccessControllerTransactionFeePayer,
        primary_role_locked: bool,
        combination: RolesExercisableInTransactionManifestCombination,
    ) -> FeeLockPlacement {
        FeeLockPlacement::new(
            fee_payer,
            securified_account().entity.address.into(),
            primary_role_locked,
            combination,
        )
    }

    fn recovery(
        combination: RolesExercisableInTransactionManifestCombination,
    ) -> TransactionManifest {
        TransactionManifest::apply_security_shield_for_securified_entity(
            securified_account(),
            SecurityStructureOfFactorInstances::sample(),
            combination,
        )
    }

    #[test]
    fn another_payer_locks_the_fee_before_the_recovery() {
        for fee_payer in
            [paid_by_another_account(), paid_by_access_controller()]
        {
            for combination in COMBINATIONS {
                for locked in [false, true] {
                    assert_eq!(
                        placement(&fee_payer, locked, combination),
                        FeeLockPlacement::BeforeRecovery
                    );
                }
            }
        }
    }

    #[test]
    fn the_account_being_recovered_pays_with_a_primary_role_it_has() {
        let fee_payer = paid_by_itself();
        for (combination, locked, expected) in [
            // With its current primary role.
            (
                InitiateWithRecoveryCompleteWithPrimary,
                false,
                FeeLockPlacement::BeforeRecovery,
            ),
            (
                InitiateWithPrimaryCompleteWithConfirmation,
                false,
                FeeLockPlacement::BeforeRecovery,
            ),
            // With its new one, once confirmed.
            (
                InitiateWithRecoveryCompleteWithConfirmation,
                false,
                FeeLockPlacement::AfterConfirmation,
            ),
            (
                InitiateWithRecoveryCompleteWithPrimary,
                true,
                FeeLockPlacement::AfterConfirmation,
            ),
            // With neither.
            (
                InitiateWithRecoveryDelayedCompletion,
                false,
                FeeLockPlacement::Impossible,
            ),
            (
                InitiateWithRecoveryDelayedCompletion,
                true,
                FeeLockPlacement::Impossible,
            ),
        ] {
            assert_eq!(
                placement(&fee_payer, locked, combination),
                expected,
                "{combination:?}, locked: {locked}"
            );
        }
    }

    #[test]
    fn the_access_controller_pays_only_when_picked() {
        let manifest = recovery(InitiateWithRecoveryDelayedCompletion);

        let paid = manifest
            .apply_lock_fee_instruction(
                &paid_by_access_controller(),
                FeeLockPlacement::BeforeRecovery,
            )
            .unwrap();

        assert_eq!(
            paid.extract_fee_payer(),
            Some(FeePayerOfManifest::AccessController {
                address: securified_account().access_controller_address(),
                fee: Decimal192::five(),
            })
        );
        for fee_payer in [paid_by_another_account(), paid_by_itself()] {
            let paid = manifest
                .apply_lock_fee_instruction(
                    &fee_payer,
                    FeeLockPlacement::BeforeRecovery,
                )
                .unwrap();
            assert!(!paid.manifest_string().contains("lock_recovery_fee"));
        }
    }

    #[test]
    fn paying_after_confirmation_locks_the_fee_last() {
        let manifest = recovery(InitiateWithRecoveryCompleteWithConfirmation);

        let paid = manifest
            .apply_lock_fee_instruction(
                &paid_by_itself(),
                FeeLockPlacement::AfterConfirmation,
            )
            .unwrap();

        let instructions = paid.instructions();
        let recovery_instructions = manifest.instructions().len();
        assert_eq!(
            instructions[..recovery_instructions],
            manifest.instructions()[..]
        );
        let lock_fee = paid.manifest_string();
        let confirm = lock_fee
            .find("quick_confirm_recovery_role_recovery_proposal")
            .unwrap();
        assert!(lock_fee.find("\"create_proof\"").unwrap() > confirm);
        assert!(lock_fee.find("\"lock_fee\"").unwrap() > confirm);
    }

    /// Whoever pays, the manifest only recovers and pays the fee, so it's
    /// still classified as a recovery and shows all it does.
    #[test]
    fn paying_the_fee_keeps_the_recovery_classified() {
        for combination in COMBINATIONS {
            let class = if combination.can_quick_confirm() {
                ManifestClassification::AccessControllerRecovery
            } else {
                ManifestClassification::AccessControllerTimedRecovery
            };
            for fee_payer in [
                paid_by_itself(),
                paid_by_another_account(),
                paid_by_access_controller(),
            ] {
                let placement = placement(&fee_payer, false, combination);
                let paid = recovery(combination)
                    .apply_lock_fee_instruction(&fee_payer, placement)
                    .unwrap();
                let classes = paid.summary().unwrap().classification;
                assert!(
                    classes.contains(&class),
                    "{combination:?}: {classes:?}"
                );
                for not_shown in
                    ["withdraw", "contribute_recovery_fee", "SET_METADATA"]
                {
                    assert!(!paid.manifest_string().contains(not_shown));
                }
            }
        }
    }
}
