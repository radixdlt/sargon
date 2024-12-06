#![cfg(test)]

use crate::prelude::*;

use NotYetValidReason::*;
type Validation = RoleBuilderValidation;

#[allow(clippy::upper_case_acronyms)]

type MutRes = RoleBuilderMutateResult;

#[test]
fn new_builder_primary() {
    assert_eq!(PrimaryRoleBuilder::new().role(), RoleKind::Primary);
}

#[test]
fn empty_is_err_primary() {
    let sut = PrimaryRoleBuilder::new();
    let res = sut.build();
    assert_eq!(
        res,
        Result::not_yet_valid(NotYetValidReason::RoleMustHaveAtLeastOneFactor)
    );
}

mod primary_test_helper_functions {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrimaryRoleBuilder;

    #[test]
    fn factor_sources_not_of_kind_to_list_of_kind_in_override() {
        let mut sut = SUT::new();
        sut.add_factor_source_to_override(FactorSourceID::sample_device())
            .unwrap();
        sut.add_factor_source_to_override(FactorSourceID::sample_ledger())
            .unwrap();
        sut.add_factor_source_to_override(FactorSourceID::sample_arculus())
            .unwrap();

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::Device,
            FactorListKind::Override,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_arculus()
            ]
        );

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::LedgerHQHardwareWallet,
            FactorListKind::Override,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_arculus()
            ]
        );

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::ArculusCard,
            FactorListKind::Override,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_ledger()
            ]
        );
    }

    #[test]
    fn factor_sources_not_of_kind_to_list_of_kind_in_threshold() {
        let mut sut = SUT::new();
        sut.add_factor_source_to_threshold(FactorSourceID::sample_device())
            .unwrap();
        sut.add_factor_source_to_threshold(FactorSourceID::sample_ledger())
            .unwrap();
        sut.add_factor_source_to_threshold(FactorSourceID::sample_arculus())
            .unwrap();

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::Device,
            FactorListKind::Threshold,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_arculus()
            ]
        );

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::LedgerHQHardwareWallet,
            FactorListKind::Threshold,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_arculus()
            ]
        );

        let xs = sut.factor_sources_not_of_kind_to_list_of_kind(
            FactorSourceKind::ArculusCard,
            FactorListKind::Threshold,
        );
        assert_eq!(
            xs,
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_ledger()
            ]
        );
    }
}

#[allow(clippy::upper_case_acronyms)]
type SUT = PrimaryRoleBuilder;

fn make() -> SUT {
    SUT::new()
}

#[cfg(test)]
mod threshold_suite {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_device()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_ledger()
    }

    fn sample_third() -> FactorSourceID {
        FactorSourceID::sample_arculus()
    }

    #[test]
    fn remove_lowers_threshold_from_1_to_0() {
        let mut sut = make();
        let fs = sample();
        assert_eq!(sut.get_threshold(), 0);
        sut.add_factor_source_to_threshold(fs).unwrap(); // should automatically increase threshold to 1
        assert_eq!(sut.get_threshold(), 1);
        sut.remove_factor_source(&fs).unwrap();
        assert_eq!(sut.get_threshold(), 0);
    }

    #[test]
    fn remove_lowers_threshold_from_3_to_1() {
        let mut sut = make();
        let fs0 = sample();
        let fs1 = sample_other();
        sut.add_factor_source_to_threshold(fs0).unwrap();
        sut.add_factor_source_to_threshold(fs1).unwrap();
        sut.add_factor_source_to_threshold(
            FactorSourceID::sample_arculus_other(),
        )
        .unwrap();
        sut.set_threshold(2).unwrap();
        assert_eq!(sut.get_threshold(), 2);
        sut.remove_factor_source(&fs0).unwrap();
        assert_eq!(sut.get_threshold(), 2); // assert that we DIDN'T lower the threshold, since we have 2 factors
        sut.remove_factor_source(&fs1).unwrap();
        assert_eq!(sut.get_threshold(), 1); // assert that we DID lower the threshold now that we have 1 factor
    }

    #[test]
    fn remove_from_override_does_not_change_threshold() {
        let mut sut = make();
        sut.add_factor_source_to_threshold(sample()).unwrap();
        let _ = sut.build(); // build should not mutate neither consume
        sut.add_factor_source_to_threshold(sample_other()).unwrap();
        let fs = FactorSourceID::sample_arculus_other();
        sut.add_factor_source_to_override(fs).unwrap();
        let _ = sut.build(); // build should not mutate neither consume
        sut.set_threshold(2).unwrap();
        let _ = sut.build(); // build should not mutate neither consume
        assert_eq!(sut.get_threshold(), 2);
        sut.remove_factor_source(&fs).unwrap();
        assert_eq!(sut.get_threshold(), 2);

        let built = sut.build().unwrap();
        let built2 = sut.build().unwrap();
        assert_eq!(built.get_threshold(), 2);
        assert_eq!(built2, built); // can built many times

        assert_eq!(built.role(), RoleKind::Primary);

        assert_eq!(
            built.get_threshold_factors(),
            &vec![sample(), sample_other()]
        );

        assert_eq!(built.get_override_factors(), &Vec::new());
    }

    #[test]
    fn one_factor_then_set_threshold_to_one_is_ok() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source_to_threshold(sample_other()).unwrap();
        sut.set_threshold(1).unwrap();

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            1,
            [sample_other()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn zero_factor_then_set_threshold_to_one_is_not_yet_valid_then_add_one_factor_is_ok(
    ) {
        // Arrange
        let mut sut = make();

        // Act
        assert_eq!(
            sut.set_threshold(1),
            Err(Validation::NotYetValid(
                ThresholdHigherThanThresholdFactorsLen
            ))
        );
        sut.add_factor_source_to_threshold(sample_other()).unwrap();

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            1,
            [sample_other()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn zero_factor_then_set_threshold_to_two_is_not_yet_valid_then_add_two_factor_is_ok(
    ) {
        // Arrange
        let mut sut = make();

        // Act
        assert_eq!(
            sut.set_threshold(2),
            Err(Validation::NotYetValid(
                ThresholdHigherThanThresholdFactorsLen
            ))
        );
        sut.add_factor_source_to_threshold(sample()).unwrap();

        sut.add_factor_source_to_threshold(sample_other()).unwrap();

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            2,
            [sample(), sample_other()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn add_two_factors_then_set_threshold_to_two_is_ok() {
        // Arrange
        let mut sut = make();

        sut.add_factor_source_to_threshold(sample()).unwrap();
        sut.add_factor_source_to_threshold(sample_other()).unwrap();

        // Act
        assert_eq!(sut.set_threshold(2), Ok(()));

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            2,
            [sample(), sample_other()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn add_two_factors_then_set_threshold_to_three_is_not_yet_valid_then_add_third_factor_is_ok(
    ) {
        // Arrange
        let mut sut = make();

        sut.add_factor_source_to_threshold(sample()).unwrap();
        sut.add_factor_source_to_threshold(sample_other()).unwrap();

        // Act
        assert_eq!(
            sut.set_threshold(3),
            Err(Validation::NotYetValid(
                ThresholdHigherThanThresholdFactorsLen
            ))
        );

        sut.add_factor_source_to_threshold(sample_third()).unwrap();

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            3,
            [sample(), sample_other(), sample_third()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn one_factors_set_threshold_of_one_is_ok() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source_to_threshold(sample_other()).unwrap();
        sut.set_threshold(1).unwrap();

        // Assert
        let expected = RoleWithFactorSourceIds::primary_with_factors(
            1,
            [sample_other()],
            [],
        );
        assert_eq!(sut.build().unwrap(), expected);
    }

    #[test]
    fn one_override_factors_set_threshold_to_one_is_not_yet_valid() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source_to_override(sample_other()).unwrap();
        assert_eq!(
            sut.set_threshold(1),
            Err(Validation::NotYetValid(
                ThresholdHigherThanThresholdFactorsLen
            ))
        );

        // Assert

        assert_eq!(
            sut.build(),
            Err(Validation::NotYetValid(
                ThresholdHigherThanThresholdFactorsLen
            ))
        );
    }

    #[test]
    fn validation_for_addition_of_factor_source_for_each_before_after_adding_a_factor(
    ) {
        let mut sut = make();
        let fs0 = FactorSourceID::sample_ledger();
        let fs1 = FactorSourceID::sample_password();
        let fs2 = FactorSourceID::sample_arculus();
        let xs = sut.validation_for_addition_of_factor_source_for_each(
            FactorListKind::Threshold,
            &IndexSet::from_iter([fs0, fs1, fs2]),
        );
        assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            vec![
                FactorSourceInRoleBuilderValidationStatus::ok(RoleKind::Primary, fs0,),
                FactorSourceInRoleBuilderValidationStatus::not_yet_valid(
                    RoleKind::Primary,
                    fs1,
                    NotYetValidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
                ),
                FactorSourceInRoleBuilderValidationStatus::ok(RoleKind::Primary, fs2,),
            ]
        );
        _ = sut.add_factor_source_to_threshold(fs0);
        _ = sut.set_threshold(2);

        let xs = sut.validation_for_addition_of_factor_source_for_each(
            FactorListKind::Threshold,
            &IndexSet::from_iter([fs0, fs1, fs2]),
        );
        assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            vec![
                FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                    RoleKind::Primary,
                    fs0,
                    ForeverInvalidReason::FactorSourceAlreadyPresent
                ),
                FactorSourceInRoleBuilderValidationStatus::ok(
                    RoleKind::Primary,
                    fs1,
                ),
                FactorSourceInRoleBuilderValidationStatus::ok(
                    RoleKind::Primary,
                    fs2,
                ),
            ]
        );
    }
}

#[cfg(test)]
mod password {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_password()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_password_other()
    }

    #[test]
    fn test_suite_prerequisite() {
        assert_eq!(sample(), sample());
        assert_eq!(sample_other(), sample_other());
        assert_ne!(sample(), sample_other());
    }

    mod threshold_in_isolation {
        use super::*;

        #[test]
        fn duplicates_not_allowed() {
            let mut sut = make();
            sut.add_factor_source_to_threshold(FactorSourceID::sample_device())
                .unwrap();
            _ = sut.set_threshold(2);
            test_duplicates_not_allowed(
                sut,
                FactorListKind::Threshold,
                sample(),
            );
        }

        #[test]
        fn alone_is_not_ok() {
            // Arrange
            let mut sut = make();
            let _ = sut.set_threshold(1);
            // Act
            let res = sut.add_factor_source_to_threshold(sample());

            // Assert
            assert_eq!(
                res,
                MutRes::not_yet_valid(
                    NotYetValidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
                )
            );

            let validation = sut.validate();
            assert_eq!(
                validation,
                Result::not_yet_valid(
                    NotYetValidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
                )
            );
        }

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_list() {
            use FactorSourceKind::*;

            let not_ok = |kind: FactorSourceKind| {
                let sut = make();
                let res = sut
                    .validation_for_addition_of_factor_source_of_kind_to_list(
                        kind,
                        FactorListKind::Threshold,
                    );
                assert!(res.is_err());
            };

            let ok_with = |kind: FactorSourceKind, setup: fn(&mut SUT)| {
                let mut sut = make();
                setup(&mut sut);
                let res = sut
                    .validation_for_addition_of_factor_source_of_kind_to_list(
                        kind,
                        FactorListKind::Threshold,
                    );
                assert!(res.is_ok());
            };
            let ok = |kind: FactorSourceKind| {
                ok_with(kind, |_| {});
            };

            ok(LedgerHQHardwareWallet);
            ok(ArculusCard);
            ok(OffDeviceMnemonic);

            ok_with(Device, |sut| {
                sut.add_factor_source_to_threshold(
                    FactorSourceID::sample_ledger(),
                )
                .unwrap();
            });
            ok_with(Password, |sut| {
                sut.add_factor_source_to_threshold(
                    FactorSourceID::sample_device(),
                )
                .unwrap();
                _ = sut.set_threshold(2);
            });

            not_ok(SecurityQuestions);
            not_ok(TrustedContact);
        }
    }

    mod override_in_isolation {
        use super::*;

        #[test]
        fn unsupported() {
            // Arrange
            let mut sut = make();

            // Act
            let res = sut.add_factor_source_to_override(sample());

            // Assert
            assert_eq!(
                res,
                MutRes::forever_invalid(
                    ForeverInvalidReason::PrimaryCannotHavePasswordInOverrideList
                )
            );
        }

        #[test]
        fn valid_then_invalid_because_unsupported() {
            // Arrange
            let mut sut = make();
            sut.add_factor_source_to_override(FactorSourceID::sample_device())
                .unwrap();
            sut.add_factor_source_to_override(FactorSourceID::sample_ledger())
                .unwrap();
            sut.add_factor_source_to_override(FactorSourceID::sample_arculus())
                .unwrap();

            // Act
            let res = sut.add_factor_source_to_override(sample());

            // Assert
            assert_eq!(
                res,
                MutRes::forever_invalid(
                    ForeverInvalidReason::PrimaryCannotHavePasswordInOverrideList
                )
            );
        }

        #[test]
        fn validation_for_addition_of_factor_source_of_kind_to_override() {
            use FactorSourceKind::*;

            let not_ok = |kind: FactorSourceKind| {
                let sut = make();
                let res = sut.validation_for_addition_of_factor_source_of_kind_to_override(kind);
                assert!(res.is_err());
            };

            let ok_with = |kind: FactorSourceKind, setup: fn(&mut SUT)| {
                let mut sut = make();
                setup(&mut sut);
                let res = sut.validation_for_addition_of_factor_source_of_kind_to_override(kind);
                assert!(res.is_ok());
            };
            let ok = |kind: FactorSourceKind| {
                ok_with(kind, |_| {});
            };

            ok(LedgerHQHardwareWallet);
            ok(ArculusCard);
            ok(OffDeviceMnemonic);

            ok_with(Device, |sut| {
                sut.add_factor_source_to_override(
                    FactorSourceID::sample_ledger(),
                )
                .unwrap();
            });

            not_ok(Password);

            not_ok(SecurityQuestions);
            not_ok(TrustedContact);
        }
    }
}

#[cfg(test)]
mod ledger {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_ledger()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_ledger_other()
    }

    #[test]
    fn test_suite_prerequisite() {
        assert_eq!(sample(), sample());
        assert_eq!(sample_other(), sample_other());
        assert_ne!(sample(), sample_other());
    }

    mod threshold_in_isolation {
        use super::*;
        fn list() -> FactorListKind {
            FactorListKind::Threshold
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample());
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap();
            sut.set_threshold(1).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                1,
                [sample()],
                [],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }

        #[test]
        fn one_with_threshold_of_zero_is_err() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap(); // should automatically bump threshold to 1

            let _ = sut.set_threshold(0);

            // Assert
            assert_eq!(
                sut.build(),
                Err(RoleBuilderValidation::NotYetValid(
                    NotYetValidReason::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
                ))
            );
        }

        #[test]
        fn two_different_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap();
            sut.add_factor_source_to_threshold(sample_other()).unwrap();
            sut.set_threshold(2).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                2,
                [sample(), sample_other()],
                [],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }
    }

    mod override_in_isolation {
        use super::*;
        fn list() -> FactorListKind {
            FactorListKind::Override
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample());
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_override(sample()).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                0,
                [],
                [sample()],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }

        #[test]
        fn two_different_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_override(sample()).unwrap();
            sut.add_factor_source_to_override(sample_other()).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                0,
                [],
                [sample(), sample_other()],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }
    }
}

#[cfg(test)]
mod arculus {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_arculus()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_arculus_other()
    }

    #[test]
    fn test_suite_prerequisite() {
        assert_eq!(sample(), sample());
        assert_eq!(sample_other(), sample_other());
        assert_ne!(sample(), sample_other());
    }

    mod threshold_in_isolation {
        use super::*;
        fn list() -> FactorListKind {
            FactorListKind::Threshold
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample());
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap();
            sut.set_threshold(1).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                1,
                [sample()],
                [],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }

        #[test]
        fn two_different_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap();
            sut.add_factor_source_to_threshold(sample_other()).unwrap();
            sut.set_threshold(1).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                1,
                [sample(), sample_other()],
                [],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }
    }

    mod override_in_isolation {
        use super::*;
        fn list() -> FactorListKind {
            FactorListKind::Override
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample());
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_override(sample()).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                0,
                [],
                [sample()],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }

        #[test]
        fn two_different_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_override(sample()).unwrap();
            sut.add_factor_source_to_override(sample_other()).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                0,
                [],
                [sample(), sample_other()],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }
    }
}

#[cfg(test)]
mod device_factor_source {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_device()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_device_other()
    }

    #[test]
    fn test_suite_prerequisite() {
        assert_eq!(sample(), sample());
        assert_eq!(sample_other(), sample_other());
        assert_ne!(sample(), sample_other());
    }

    #[cfg(test)]
    mod threshold_in_isolation {
        use super::*;

        fn list() -> FactorListKind {
            FactorListKind::Threshold
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample())
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_threshold(sample()).unwrap();
            sut.set_threshold(1).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                1,
                [sample()],
                [],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }

        #[test]
        fn two_different_is_err() {
            // Arrange
            let mut sut = make();

            sut.add_factor_source_to_threshold(sample()).unwrap();

            // Act
            let res = sut.add_factor_source_to_threshold(sample_other());

            // Assert
            assert!(matches!(
                res,
                MutRes::Err(Validation::ForeverInvalid(
                    ForeverInvalidReason::PrimaryCannotHaveMultipleDevices
                ))
            ));
        }
    }

    mod override_in_isolation {

        use super::*;

        fn list() -> FactorListKind {
            FactorListKind::Override
        }

        #[test]
        fn duplicates_not_allowed() {
            test_duplicates_not_allowed(make(), list(), sample())
        }

        #[test]
        fn one_is_ok() {
            // Arrange
            let mut sut = make();

            // Act
            sut.add_factor_source_to_override(sample()).unwrap();

            // Assert
            let expected = RoleWithFactorSourceIds::primary_with_factors(
                0,
                [],
                [sample()],
            );
            assert_eq!(sut.build().unwrap(), expected);
        }
    }
}
