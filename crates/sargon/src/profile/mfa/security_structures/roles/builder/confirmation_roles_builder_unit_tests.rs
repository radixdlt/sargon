#![cfg(test)]

use crate::prelude::*;

#[allow(clippy::upper_case_acronyms)]

type MutRes = RoleBuilderMutateResult;

#[test]
fn new_builder_confirmation() {
    assert_eq!(
        ConfirmationRoleBuilder::new().role(),
        RoleKind::Confirmation
    );
}

#[test]
fn empty_is_err_confirmation() {
    let sut = ConfirmationRoleBuilder::new();
    let res = sut.build();
    assert_eq!(
        res,
        Result::not_yet_valid(NotYetValidReason::RoleMustHaveAtLeastOneFactor)
    );
}

#[allow(clippy::upper_case_acronyms)]
type SUT = ConfirmationRoleBuilder;

fn make() -> SUT {
    SUT::new()
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
    ok(SecurityQuestions);
    ok(Password);
    ok(OffDeviceMnemonic);
    not_ok(TrustedContact);
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
    fn set_threshold_is_unsupported() {
        let mut sut = make();
        assert_eq!(
            sut.set_threshold(1),
            MutRes::basic_violation(
                BasicViolation::ConfirmationCannotSetThreshold
            )
        );
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
            ConfirmationRoleWithFactorSourceIds::override_only([sample()])
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
        let built = sut.build().unwrap();
        assert!(built.get_threshold_factors().is_empty());
        assert_eq!(
            built,
            ConfirmationRoleWithFactorSourceIds::override_only([
                sample(),
                sample_other()
            ])
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
            ConfirmationRoleWithFactorSourceIds::override_only([sample(),])
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
            ConfirmationRoleWithFactorSourceIds::override_only([
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
            ConfirmationRoleWithFactorSourceIds::override_only([sample(),])
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
            ConfirmationRoleWithFactorSourceIds::override_only([
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
            ConfirmationRoleWithFactorSourceIds::override_only([sample(),])
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
            ConfirmationRoleWithFactorSourceIds::override_only([
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
                ForeverInvalidReason::ConfirmationRoleTrustedContactNotSupported
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
                ForeverInvalidReason::ConfirmationRoleTrustedContactNotSupported
            )
        );
    }
}

mod password_in_isolation {
    use super::*;

    fn sample() -> FactorSourceID {
        FactorSourceID::sample_password()
    }

    fn sample_other() -> FactorSourceID {
        FactorSourceID::sample_password_other()
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
            ConfirmationRoleWithFactorSourceIds::override_only([sample(),])
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
            ConfirmationRoleWithFactorSourceIds::override_only([
                sample(),
                sample_other()
            ])
        );
    }
}
