#![cfg(test)]
use crate::prelude::*;

#[allow(clippy::upper_case_acronyms)]
type SUT = MatrixBuilder;

fn make() -> SUT {
    SUT::new()
}

#[test]
fn empty_primary_is_err() {
    let sut = make();
    let res = sut.build();
    assert_eq!(
        res,
        MatrixBuilderBuildResult::Err(
            MatrixBuilderValidation::RoleInIsolation {
                role: RoleKind::Primary,
                violation: RoleBuilderValidation::NotYetValid(
                    NotYetValidReason::RoleMustHaveAtLeastOneFactor
                )
            }
        )
    )
}

#[test]
fn empty_recovery_is_err() {
    let mut sut = make();
    sut.add_factor_source_to_primary_override(FactorSourceID::sample_ledger())
        .unwrap();
    let res = sut.build();
    assert_eq!(
        res,
        MatrixBuilderBuildResult::Err(
            MatrixBuilderValidation::RoleInIsolation {
                role: RoleKind::Recovery,
                violation: RoleBuilderValidation::NotYetValid(
                    NotYetValidReason::RoleMustHaveAtLeastOneFactor
                )
            }
        )
    )
}

#[test]
fn empty_confirmation_is_err() {
    let mut sut = make();
    sut.add_factor_source_to_primary_override(FactorSourceID::sample_ledger())
        .unwrap();

    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_arculus())
        .unwrap();
    let res = sut.build();
    assert_eq!(
        res,
        MatrixBuilderBuildResult::Err(
            MatrixBuilderValidation::RoleInIsolation {
                role: RoleKind::Confirmation,
                violation: RoleBuilderValidation::NotYetValid(
                    NotYetValidReason::RoleMustHaveAtLeastOneFactor
                )
            }
        )
    )
}

#[test]
fn set_number_of_days_cannot_be_zero() {
    let mut sut = make();

    // Primary
    sut.add_factor_source_to_primary_threshold(FactorSourceID::sample_device())
        .unwrap();

    sut.set_threshold(Threshold::Specific(1)).unwrap();

    // Recovery
    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_ledger())
        .unwrap();

    // Confirmation
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_password(),
    )
    .unwrap();

    sut.time_until_delayed_confirmation_is_callable = TimePeriod::with_days(0); // bypass validation

    // Build
    let validation = MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::Basic(MatrixRolesInCombinationBasicViolation::NumberOfDaysUntilTimeBasedConfirmationMustBeGreaterThanZero)
        );
    assert_eq!(sut.validate(), Err(validation));
    let res = sut.build();
    assert_eq!(res, Err(validation));
}

#[test]
fn set_number_of_days_42() {
    let mut sut = make();

    // Primary
    sut.add_factor_source_to_primary_threshold(FactorSourceID::sample_device())
        .unwrap();

    sut.set_threshold(Threshold::Specific(1)).unwrap();

    // Recovery
    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_ledger())
        .unwrap();

    // Confirmation
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_password(),
    )
    .unwrap();

    sut.set_time_until_delayed_confirmation_is_callable(TimePeriod::with_days(
        42,
    ))
    .unwrap();

    // Build
    assert!(sut.validate().is_ok());
    let built = sut.build().unwrap();
    pretty_assertions::assert_eq!(
        built,
        MatrixOfFactorSourceIds::with_roles_and_days(
            PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_device(),],
                [],
            ),
            RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger(),
            ],),
            ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_password()
            ],),
            TimePeriod::with_days(42),
        )
    );
}

#[test]
fn timed_confirm_default() {
    assert_eq!(
        SUT::DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE,
        TimePeriod::with_days(14)
    );
}

#[test]
fn set_number_of_days_if_not_set_uses_default() {
    let mut sut = make();

    // Primary
    sut.add_factor_source_to_primary_threshold(FactorSourceID::sample_device())
        .unwrap();

    sut.set_threshold(Threshold::Specific(1)).unwrap();

    // Recovery
    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_ledger())
        .unwrap();

    // Confirmation
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_password(),
    )
    .unwrap();

    // Build
    assert!(sut.validate().is_ok());
    let built = sut.build().unwrap();
    pretty_assertions::assert_eq!(
        built,
        MatrixOfFactorSourceIds::with_roles_and_days(
            PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_device(),],
                [],
            ),
            RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger(),
            ],),
            ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_password()
            ],),
            SUT::DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE,
        )
    );
}

#[test]
fn sample_factor_cannot_be_both_in_threshold_and_override() {
    let mut sut = make();
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_override(fs).unwrap();
    let res = sut.add_factor_source_to_primary_override(fs);
    assert!(res.is_err());
}

#[test]
fn single_factor_in_primary_threshold_cannot_be_in_recovery() {
    let mut sut = make();
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_threshold(fs).unwrap();
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_arculus_other(),
    )
    .unwrap();
    sut.set_threshold(Threshold::Specific(1)).unwrap();

    // ACT
    sut.add_factor_source_to_recovery_override(fs).unwrap();
    let res = sut.validate();
    assert_eq!(res, Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
        )));

    sut.add_factor_source_to_primary_threshold(FactorSourceID::sample_arculus())
        .unwrap();

    let built = sut.build().unwrap();
    pretty_assertions::assert_eq!(
        built.primary(),
        &PrimaryRoleWithFactorSourceIds::with_factors(
            1,
            [
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_arculus()
            ],
            []
        )
    );
    pretty_assertions::assert_eq!(
        built.recovery(),
        &RecoveryRoleWithFactorSourceIds::override_only([
            FactorSourceID::sample_ledger()
        ]),
    );

    pretty_assertions::assert_eq!(
        built.confirmation(),
        &ConfirmationRoleWithFactorSourceIds::override_only([
            FactorSourceID::sample_arculus_other()
        ])
    )
}

#[test]
fn single_factor_in_primary_override_cannot_be_in_recovery() {
    // ARRANGE
    let mut sut = make();
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_arculus(),
    )
    .unwrap();

    // ACT
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_override(fs).unwrap();
    sut.add_factor_source_to_recovery_override(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(res, Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
        )));
}

#[test]
fn single_factor_in_primary_threshold_cannot_be_in_confirmation() {
    // ARRANGE
    let mut sut = make();
    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_arculus())
        .unwrap();
    _ = sut.set_threshold(Threshold::Specific(1));

    // ACT
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_threshold(fs).unwrap();
    sut.add_factor_source_to_confirmation_override(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(res, Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
        )));
}

#[test]
fn single_factor_in_primary_override_cannot_be_in_confirmation() {
    // ARRANGE
    let mut sut = make();
    sut.add_factor_source_to_recovery_override(FactorSourceID::sample_arculus())
        .unwrap();

    // ACT
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_override(fs).unwrap();
    sut.add_factor_source_to_confirmation_override(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(res, Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
        )));
}

#[test]
fn add_factor_to_recovery_then_same_to_confirmation_is_err() {
    // ARRANGE
    let mut sut = make();
    sut.add_factor_source_to_primary_override(FactorSourceID::sample_arculus())
        .unwrap();

    // ACT
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_confirmation_override(fs).unwrap();
    sut.add_factor_source_to_recovery_override(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(
        res,
        Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::ForeverInvalid(
                MatrixRolesInCombinationForeverInvalid::RecoveryAndConfirmationFactorsOverlap
            )
        ))
    );
}

#[test]
fn add_factor_to_confirmation_then_same_to_override_when_validated_is_err() {
    // ARRANGE
    let mut sut = make();
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_primary_override(FactorSourceID::sample_arculus())
        .unwrap();

    // ACT
    sut.add_factor_source_to_recovery_override(fs).unwrap();
    sut.add_factor_source_to_confirmation_override(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(
        res,
        Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::ForeverInvalid(
                MatrixRolesInCombinationForeverInvalid::RecoveryAndConfirmationFactorsOverlap
            )
        ))
    );
}

#[test]
fn add_factor_to_confirmation_then_same_to_primary_threshold_is_not_yet_valid()
{
    // ARRANGE
    let mut sut = make();
    sut.add_factor_source_to_confirmation_override(
        FactorSourceID::sample_arculus(),
    )
    .unwrap();
    _ = sut.set_threshold(Threshold::Specific(1));

    // ACT
    let fs = FactorSourceID::sample_ledger();
    sut.add_factor_source_to_recovery_override(fs).unwrap();
    sut.add_factor_source_to_primary_threshold(fs).unwrap();

    // ASSERT
    let res = sut.validate();
    assert_eq!(res, Err(MatrixBuilderValidation::CombinationViolation(
            MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole)
        )));
}

mod remove {
    use super::*;

    #[test]
    fn not_found() {
        let mut sut = make();
        let res =
            sut.remove_factor_from_all_roles(&FactorSourceID::sample_device());
        assert_eq!(
            res,
            Err(MatrixBuilderValidation::CombinationViolation(
                MatrixRolesInCombinationViolation::Basic(
                    MatrixRolesInCombinationBasicViolation::FactorSourceNotFoundInAnyRole
                )
            ))
        );
    }

    #[test]
    fn remove_from_primary_threshold_is_ok() {
        let mut sut = make();
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        assert_eq!(
            sut.primary_role.get_threshold_factors(),
            &[FactorSourceID::sample_device()]
        );
        let res =
            sut.remove_factor_from_all_roles(&FactorSourceID::sample_device());
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn remove_from_primary_override_is_ok() {
        let mut sut = make();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        let res = sut.remove_factor_from_primary(
            &FactorSourceID::sample_device(),
            FactorListKind::Override,
        );
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn remove_all_from_primary_override_is_ok() {
        let mut sut = make();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_ledger(),
        )
        .unwrap();
        let res = sut.remove_all_factors_from_primary_override();
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn remove_from_recovery_is_ok() {
        let mut sut = make();
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        let res = sut.remove_factor_from_recovery(
            &FactorSourceID::sample_device(),
            FactorListKind::Override,
        );
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn remove_from_confirmation_is_ok() {
        let mut sut = make();
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        let res = sut.remove_factor_from_confirmation(
            &FactorSourceID::sample_device(),
            FactorListKind::Override,
        );
        assert_eq!(res, Ok(()));
    }
}

mod validation_for_addition_of_factor_source_for_each {
    use super::*;

    mod primary {

        use super::*;

        #[test]
        fn empty() {
            let sut = make();
            let xs = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                &IndexSet::new(),
            );
            assert_eq!(xs, IndexSet::new());
        }

        #[test]
        fn device_threshold_3x_first_ok_second_not() {
            let mut sut = make();
            let xs = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                &IndexSet::from_iter([
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_device_other(),
                ]),
            );
            assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                vec![
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device()
                    ),
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device_other(),
                    )
                ]
            );

            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            let xs = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                &IndexSet::from_iter([
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_device_other(),
                ]),
            );

            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                vec![
                    FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                        RoleKind::Primary,
                        FactorSourceID::sample_device(),
                        ForeverInvalidReason::FactorSourceAlreadyPresent
                    ),
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device_other(),
                    ),
                ]
            );
        }

        #[test]
        fn device_threshold_override_2x_first_ok_second_not() {
            let mut sut = make();
            let xs = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                &IndexSet::from_iter([
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_device_other(),
                ]),
            );
            assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                vec![
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device()
                    ),
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device_other(),
                    )
                ]
            );

            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            let xs = sut.validation_for_addition_of_factor_source_to_primary_override_for_each(
                &IndexSet::from_iter([
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_device_other(),
                ]),
            );

            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                vec![
                    FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                        RoleKind::Primary,
                        FactorSourceID::sample_device(),
                        ForeverInvalidReason::FactorSourceAlreadyPresent
                    ),
                    FactorSourceInRoleBuilderValidationStatus::ok(
                        RoleKind::Primary,
                        FactorSourceID::sample_device_other(),
                    ),
                ]
            );
        }
    }

    mod recovery {
        use super::*;

        fn role() -> RoleKind {
            RoleKind::Recovery
        }

        #[test]
        fn empty() {
            let sut = make();
            let xs = sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(
                &IndexSet::new(),
            );
            assert_eq!(xs, IndexSet::new());
        }

        #[test]
        fn supported() {
            let sut = make();
            let fsids = vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other(),
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other(),
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_arculus_other(),
                FactorSourceID::sample_off_device(),
                FactorSourceID::sample_off_device_other(),
                FactorSourceID::sample_trusted_contact(),
                FactorSourceID::sample_trusted_contact_other(),
            ];
            let xs = sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(
                &IndexSet::from_iter(fsids.clone()),
            );
            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                fsids
                    .into_iter()
                    .map(|fsid| FactorSourceInRoleBuilderValidationStatus::ok(
                        role(),
                        fsid
                    ))
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn password_and_security_questions_not_supported() {
            let sut = make();
            let xs = sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(
                &IndexSet::from_iter([
                    FactorSourceID::sample_password(),
                    FactorSourceID::sample_password_other(),
                    FactorSourceID::sample_security_questions(),
                    FactorSourceID::sample_security_questions_other(),
                ]),
            );
            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                [
                    FactorSourceID::sample_password(),
                    FactorSourceID::sample_password_other(),
                    FactorSourceID::sample_security_questions(),
                    FactorSourceID::sample_security_questions_other(),
                ]
                .into_iter()
                .map(
                    |fsid| FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                        role(),
                        fsid,
                        if fsid.get_factor_source_kind() == FactorSourceKind::SecurityQuestions {
                            ForeverInvalidReason::RecoveryRoleSecurityQuestionsNotSupported
                        } else {
                            ForeverInvalidReason::RecoveryRolePasswordNotSupported
                        }
                    )
                )
                .collect::<Vec<_>>()
            );
        }
    }

    mod confirmation {
        use super::*;

        fn role() -> RoleKind {
            RoleKind::Confirmation
        }

        #[test]
        fn empty() {
            let sut = make();
            let xs = sut
                .validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                    &IndexSet::new(),
                );
            assert_eq!(xs, IndexSet::new());
        }

        #[test]
        fn supported() {
            let sut = make();
            let fsids = vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other(),
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other(),
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_arculus_other(),
                FactorSourceID::sample_security_questions(),
                FactorSourceID::sample_security_questions_other(),
                FactorSourceID::sample_password(),
                FactorSourceID::sample_password_other(),
                FactorSourceID::sample_off_device(),
                FactorSourceID::sample_off_device_other(),
            ];
            let xs = sut
                .validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                    &IndexSet::from_iter(fsids.clone()),
                );
            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                fsids
                    .into_iter()
                    .map(|fsid| FactorSourceInRoleBuilderValidationStatus::ok(
                        role(),
                        fsid
                    ))
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn password_and_security_questions_not_supported() {
            let sut = make();
            let xs = sut
                .validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                    &IndexSet::from_iter([
                        FactorSourceID::sample_trusted_contact(),
                        FactorSourceID::sample_trusted_contact_other(),
                    ]),
                );
            pretty_assertions::assert_eq!(
                xs.into_iter().collect::<Vec<_>>(),
                [
                    FactorSourceID::sample_trusted_contact(),
                    FactorSourceID::sample_trusted_contact_other(),
                ]
                .into_iter()
                .map(
                    |fsid| FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                        role(),
                        fsid,
                        ForeverInvalidReason::ConfirmationRoleTrustedContactNotSupported
                    )
                )
                .collect::<Vec<_>>()
            );
        }
    }
}

mod validation_of_addition_of_kind {
    use super::*;

    mod recovery {
        use super::*;

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_recovery_override_empty(
        ) {
            let sut = make();
            let test = |kind: FactorSourceKind, should_be_ok: bool| {
                let is_ok = sut
                    .validation_for_addition_of_factor_source_of_kind_to_recovery_override(kind)
                    .is_ok();
                assert_eq!(is_ok, should_be_ok);
            };
            test(FactorSourceKind::Device, true);
            test(FactorSourceKind::LedgerHQHardwareWallet, true);
            test(FactorSourceKind::ArculusCard, true);
            test(FactorSourceKind::SecurityQuestions, false);
            test(FactorSourceKind::Password, false);
            test(FactorSourceKind::OffDeviceMnemonic, true);
            test(FactorSourceKind::TrustedContact, true);
        }

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_recovery_override_single_recovery(
        ) {
            let mut sut = make();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            let test = |kind: FactorSourceKind, should_be_ok: bool| {
                let is_ok = sut
                    .validation_for_addition_of_factor_source_of_kind_to_recovery_override(kind)
                    .is_ok();
                assert_eq!(is_ok, should_be_ok);
            };
            test(FactorSourceKind::Device, true);
            test(FactorSourceKind::LedgerHQHardwareWallet, true);
            test(FactorSourceKind::ArculusCard, true);
            test(FactorSourceKind::SecurityQuestions, false);
            test(FactorSourceKind::Password, false);
            test(FactorSourceKind::OffDeviceMnemonic, true);
            test(FactorSourceKind::TrustedContact, true);
        }
    }

    mod confirmation {
        use super::*;

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_confirmation_override_empty(
        ) {
            let sut = make();
            let test = |kind: FactorSourceKind, should_be_ok: bool| {
                let is_ok = sut
                    .validation_for_addition_of_factor_source_of_kind_to_confirmation_override(kind)
                    .is_ok();
                assert_eq!(is_ok, should_be_ok);
            };
            test(FactorSourceKind::Device, true);
            test(FactorSourceKind::LedgerHQHardwareWallet, true);
            test(FactorSourceKind::ArculusCard, true);
            test(FactorSourceKind::SecurityQuestions, true);
            test(FactorSourceKind::Password, true);
            test(FactorSourceKind::OffDeviceMnemonic, true);
            test(FactorSourceKind::TrustedContact, false);
        }

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_confirmation_override_single_recovery(
        ) {
            let mut sut = make();
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            let test = |kind: FactorSourceKind, should_be_ok: bool| {
                let is_ok = sut
                    .validation_for_addition_of_factor_source_of_kind_to_confirmation_override(kind)
                    .is_ok();
                assert_eq!(is_ok, should_be_ok);
            };
            test(FactorSourceKind::Device, true);
            test(FactorSourceKind::LedgerHQHardwareWallet, true);
            test(FactorSourceKind::ArculusCard, true);
            test(FactorSourceKind::SecurityQuestions, true);
            test(FactorSourceKind::Password, true);
            test(FactorSourceKind::OffDeviceMnemonic, true);
            test(FactorSourceKind::TrustedContact, false);
        }
    }

    mod primary {
        use super::*;

        #[test]
        fn ledger_threshold_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::LedgerHQHardwareWallet,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn ledger_threshold_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::LedgerHQHardwareWallet,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn ledger_override_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::LedgerHQHardwareWallet,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn ledger_override_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::LedgerHQHardwareWallet,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn arculus_threshold_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_arculus(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::ArculusCard,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn arculus_threshold_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_arculus(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::ArculusCard,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn arculus_override_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::ArculusCard,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn arculus_override_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::ArculusCard,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn security_questions_not_supported_threshold() {
            // ARRANGE
            let sut = make();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::SecurityQuestions,
            );
            assert!(res.is_err());
        }

        #[test]
        fn security_questions_not_supported_override() {
            // ARRANGE
            let sut = make();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::SecurityQuestions,
            );
            assert!(res.is_err());
        }

        #[test]
        fn trusted_contact_not_supported_threshold() {
            // ARRANGE
            let sut = make();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::TrustedContact,
            );
            assert!(res.is_err());
        }

        #[test]
        fn trusted_contact_not_supported_override() {
            // ARRANGE
            let sut = make();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::TrustedContact,
            );
            assert!(res.is_err());
        }

        #[test]
        fn passphrase_threshold_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_off_device(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::OffDeviceMnemonic,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn passphrase_threshold_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_off_device(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::OffDeviceMnemonic,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn passphrase_override_override() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_off_device(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::OffDeviceMnemonic,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn passphrase_override_threshold() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_off_device(),
            )
            .unwrap();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::OffDeviceMnemonic,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn thresehold_password_alone_is_err() {
            // ARRANGE
            let sut = make();

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Password,
            );
            assert!(res.is_err());
        }

        #[test]
        fn thresehold_password_not_alone() {
            // ARRANGE
            let mut sut = make();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_arculus(),
            )
            .unwrap();
            _ = sut.set_threshold(Threshold::Specific(2));

            // ASSERT
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Password,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_3x_threshold() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_2x_threshold_override() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_threshold_override_threshold() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_threshold_override_2x() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_3x_override() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_2x_override_threshold() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_override_threshold_2x() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }

        #[test]
        fn device_is_ok_for_second_override_threshold_override() {
            let mut sut = make();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );
            assert!(res.is_ok());
        }
    }
}

mod shield_configs {
    use super::*;

    mod mvp {

        use super::*;

        #[test]
        fn config_1_1() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            let build0 = sut.build(); // build err
            assert!(build0.is_err());

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            let built0 = sut.build().unwrap();

            // Recovery - re
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            let built2 = sut.build().unwrap();
            assert_ne!(built0, built); // we changed recovery since!
            assert_eq!(built2, built);
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_device(),
                            FactorSourceID::sample_ledger()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device(),
                        FactorSourceID::sample_ledger(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password()
                    ],),
                )
            );
            assert_eq!(built, MatrixOfFactorSourceIds::sample_config_1_1());
        }

        #[test]
        fn config_1_2() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            let res = sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            );

            assert_eq!(
                    res,
                    Err(MatrixBuilderValidation::RoleInIsolation { role: RoleKind::Primary, violation: RoleBuilderValidation::NotYetValid(NotYetValidReason::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne)}
                ));
            sut.set_threshold(Threshold::Specific(2)).unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors(
                        2,
                        [
                            FactorSourceID::sample_ledger(),
                            FactorSourceID::sample_password()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device(),
                        FactorSourceID::sample_ledger(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password()
                    ],),
                )
            );
        }

        #[test]
        fn config_1_3() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            let res = sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            );

            assert_eq!(
                        res,
                        Err(MatrixBuilderValidation::RoleInIsolation { role: RoleKind::Primary, violation: RoleBuilderValidation::NotYetValid(NotYetValidReason::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne)}
                    ));

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_device(),
                            FactorSourceID::sample_password()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device(),
                        FactorSourceID::sample_ledger()
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password()
                    ],),
                )
            );
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_1_3()
            )
        }

        #[test]
        fn config_1_4() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [FactorSourceID::sample_device(),],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger()
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_1_4()
            )
        }

        #[test]
        fn config_1_5() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [FactorSourceID::sample_ledger(),],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device()
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_1_5()
            )
        }

        #[test]
        fn config_2_1() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_device(),
                            FactorSourceID::sample_ledger()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger(),
                        FactorSourceID::sample_ledger_other(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_2_1()
            )
        }

        #[test]
        fn config_2_2() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_ledger(),
                            FactorSourceID::sample_ledger_other()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger(),
                        FactorSourceID::sample_ledger_other(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_2_2()
            )
        }

        #[test]
        fn config_2_3() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [FactorSourceID::sample_ledger(),],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger_other(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_2_3()
            )
        }

        #[test]
        fn config_2_4() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [FactorSourceID::sample_device(),],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger_other()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_2_4()
            )
        }

        #[test]
        fn config_3_0() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_device(),
                            FactorSourceID::sample_ledger()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_ledger(),
                        FactorSourceID::sample_ledger_other(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device(),
                        FactorSourceID::sample_password()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_3_0()
            )
        }

        #[test]
        fn config_4_0() {
            let mut sut = make();

            // Primary
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Recovery
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .unwrap();
            sut.add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .unwrap();

            // Confirmation
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password(),
            )
            .unwrap();
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_password_other(),
            )
            .unwrap();
            sut.add_factor_source_to_confirmation_override(
                FactorSourceID::sample_off_device(),
            )
            .unwrap();

            // Build
            assert!(sut.validate().is_ok());
            let built = sut.build().unwrap();
            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::with_roles(
                    PrimaryRoleWithFactorSourceIds::with_factors_and_threshold(
                        Threshold::All,
                        [
                            FactorSourceID::sample_device(),
                            FactorSourceID::sample_ledger()
                        ],
                        [],
                    ),
                    RecoveryRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_device(),
                        FactorSourceID::sample_ledger(),
                    ],),
                    ConfirmationRoleWithFactorSourceIds::override_only([
                        FactorSourceID::sample_password(),
                        FactorSourceID::sample_password_other(),
                        FactorSourceID::sample_off_device()
                    ],),
                )
            );

            pretty_assertions::assert_eq!(
                built,
                MatrixOfFactorSourceIds::sample_config_4_0()
            )
        }
    }
}
