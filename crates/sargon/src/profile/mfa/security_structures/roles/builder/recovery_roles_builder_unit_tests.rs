#![cfg(test)]

use crate::prelude::*;

type MutRes = RoleBuilderMutateResult;

#[test]
fn new_builder_recovery() {
    assert_eq!(RecoveryRoleBuilder::new().role(), RoleKind::Recovery);
}

#[test]
fn empty_is_err_recovery() {
    let sut = RecoveryRoleBuilder::new();
    let res = sut.build();
    assert_eq!(
        res,
        Result::not_yet_valid(NotYetValidReason::RoleMustHaveAtLeastOneFactor)
    );
}

#[allow(clippy::upper_case_acronyms)]
type SUT = RecoveryRoleBuilder;

fn make() -> SUT {
    SUT::new()
}

fn list() -> FactorListKind {
    FactorListKind::Override
}

#[test]
fn validation_for_addition_of_factor_source_of_kind_to_list() {
    use FactorSourceKind::*;
    let sut = make();
    let not_ok = |kind: FactorSourceKind| {
        let res = sut
            .validation_for_addition_of_factor_source_of_kind_to_override(kind);
        assert!(res.is_err());
    };
    let ok = |kind: FactorSourceKind| {
        let res = sut
            .validation_for_addition_of_factor_source_of_kind_to_override(kind);
        assert!(res.is_ok());
    };
    ok(Device);
    ok(LedgerHQHardwareWallet);
    ok(ArculusCard);
    ok(TrustedContact);
    ok(OffDeviceMnemonic);

    not_ok(Password);
    not_ok(SecurityQuestions);
}

#[test]
fn set_threshold_is_unsupported() {
    let mut sut = make();
    assert_eq!(
        sut.set_threshold(1),
        MutRes::basic_violation(BasicViolation::RecoveryCannotSetThreshold)
    );
}

mod device_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_device()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_device_other()
    }

    #[test]
    fn allowed_as_first_and_only() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([sample()])
        );
    }

    #[test]
    fn two_of_same_kind_allowed() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();
        sut.add_factor_source(sample_other()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([
                sample(),
                sample_other()
            ],)
        );
    }

    #[test]
    fn validation_for_addition_of_factor_source_for_each() {
        let sut = make();
        let xs = sut.validation_for_addition_of_factor_source_for_each(
            list(),
            &IndexSet::from_iter([sample(), sample_other()]),
        );
        assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            vec![
                FactorSourceInRoleBuilderValidationStatus::ok(
                    RoleKind::Recovery,
                    sample()
                ),
                FactorSourceInRoleBuilderValidationStatus::ok(
                    RoleKind::Recovery,
                    sample_other(),
                )
            ]
        );
    }
}

mod ledger_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_ledger()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_ledger_other()
    }

    #[test]
    fn allowed_as_first_and_only() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([sample()],)
        );
    }

    #[test]
    fn two_of_same_kind_allowed() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();
        sut.add_factor_source(sample_other()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([
                sample(),
                sample_other()
            ])
        );
    }
}

mod arculus_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_arculus()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_arculus_other()
    }

    #[test]
    fn allowed_as_first_and_only() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([sample(),])
        );
    }

    #[test]
    fn two_of_same_kind_allowed() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();
        sut.add_factor_source(sample_other()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([
                sample(),
                sample_other()
            ])
        );
    }
}

mod off_device_mnemonic_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_off_device()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_off_device_other()
    }

    #[test]
    fn allowed_as_first_and_only() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([sample()])
        );
    }

    #[test]
    fn two_of_same_kind_allowed() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();
        sut.add_factor_source(sample_other()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([
                sample(),
                sample_other()
            ])
        );
    }
}

mod trusted_contact_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_trusted_contact()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_trusted_contact_other()
    }

    #[test]
    fn allowed_as_first_and_only() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([sample(),])
        );
    }

    #[test]
    fn two_of_same_kind_allowed() {
        // Arrange
        let mut sut = make();

        // Act
        sut.add_factor_source(sample()).unwrap();
        sut.add_factor_source(sample_other()).unwrap();

        // Assert
        assert_eq!(
            sut.build().unwrap(),
            RoleWithFactorSourceIds::recovery_with_factors([
                sample(),
                sample_other()
            ])
        );
    }
}

mod password_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_password()
    }

    #[test]
    fn unsupported() {
        // Arrange
        let mut sut = make();

        // Act
        let res = sut.add_factor_source(sample());

        // Assert
        assert_eq!(
            res,
            MutRes::forever_invalid(
                ForeverInvalidReason::RecoveryRolePasswordNotSupported
            )
        );
    }

    #[test]
    fn valid_then_invalid_because_unsupported() {
        // Arrange
        let mut sut = make();

        sut.add_factor_source(FactorSourceID::sample_ledger())
            .unwrap();
        sut.add_factor_source(FactorSourceID::sample_arculus())
            .unwrap();

        // Act
        let res = sut.add_factor_source(sample());

        // Assert
        assert_eq!(
            res,
            MutRes::forever_invalid(
                ForeverInvalidReason::RecoveryRolePasswordNotSupported
            )
        );
    }
}

mod security_questions_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_security_questions()
    }
    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_security_questions_other()
    }

    #[test]
    fn unsupported() {
        // Arrange
        let mut sut = make();

        // Act
        let res = sut.add_factor_source(sample());

        // Assert
        assert_eq!(
            res,
            MutRes::forever_invalid(
                ForeverInvalidReason::RecoveryRoleSecurityQuestionsNotSupported
            )
        );
    }

    #[test]
    fn valid_then_invalid_because_unsupported() {
        // Arrange
        let mut sut = make();

        sut.add_factor_source(FactorSourceID::sample_ledger())
            .unwrap();
        sut.add_factor_source(FactorSourceID::sample_arculus())
            .unwrap();

        // Act
        let res = sut.add_factor_source(sample_other());

        // Assert
        let reason =
            ForeverInvalidReason::RecoveryRoleSecurityQuestionsNotSupported;
        let err = MutRes::forever_invalid(reason);
        assert_eq!(res, err);

        // .. erroneous action above did not change the state of the builder (SUT),
        // so we can build and `sample` is not present in the built result.
        assert_eq!(
            sut.build(),
            Ok(RoleWithFactorSourceIds::recovery_with_factors([
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_arculus()
            ]))
        );
    }
}
