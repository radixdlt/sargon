#![cfg(test)]

use crate::prelude::*;

use NotYetValidReason::*;

type MutRes = RoleBuilderMutateResult;

#[test]
fn validate_override_for_ever_invalid() {
    let sut = PrimaryRoleBuilder::with_factors(
        0,
        vec![],
        vec![
            FactorSourceID::sample_ledger(),
            FactorSourceID::sample_ledger(),
        ],
    );
    let res = sut.validate();
    assert_eq!(
        res,
        MutRes::forever_invalid(
            ForeverInvalidReason::FactorSourceAlreadyPresent
        )
    );
}

#[test]
fn validate_threshold_for_ever_invalid() {
    let sut = PrimaryRoleBuilder::with_factors(
        1,
        vec![
            FactorSourceID::sample_ledger(),
            FactorSourceID::sample_ledger(),
        ],
        vec![],
    );
    let res = sut.validate();
    assert_eq!(
        res,
        MutRes::forever_invalid(
            ForeverInvalidReason::FactorSourceAlreadyPresent
        )
    );
}

#[test]
fn confirmation_validate_basic_violation() {
    let sut = ConfirmationRoleBuilder::with_factors(
        1,
        vec![],
        vec![FactorSourceID::sample_ledger()],
    );
    let res = sut.validate();
    assert_eq!(
        res,
        MutRes::basic_violation(BasicViolation::ConfirmationCannotSetThreshold)
    );
}

#[test]
fn recovery_validate_basic_violation() {
    let sut = RecoveryRoleBuilder::with_factors(
        1,
        vec![],
        vec![FactorSourceID::sample_ledger()],
    );
    let res = sut.validate();
    assert_eq!(
        res,
        MutRes::basic_violation(BasicViolation::RecoveryCannotSetThreshold)
    );
}

#[test]
fn primary_validate_not_yet_valid_for_threshold_greater_than_threshold_factors()
{
    let sut = PrimaryRoleBuilder::with_factors(
        1,
        vec![],
        vec![FactorSourceID::sample_ledger()],
    );
    let res = sut.validate();
    assert_eq!(
        res,
        MutRes::not_yet_valid(ThresholdHigherThanThresholdFactorsLen)
    );
}
