use crate::prelude::*;

use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorsInvalidReason<E: Debug> {
    /// Invalid and cannot be made valid, this can happen if
    /// the violation is "only one Factor(Sources) of Kind" rule being broken.
    /// It does not matter if we add more Factor(Sources) to the MatrixOfFactor(Sources),
    /// the violation will still be there.
    #[error(transparent)]
    ForeverInvalid { violation: E },

    /// Not yet valid but can be made valid, this can happen if
    /// "never alone" rule being broken, but the validation might result in a valid
    /// MatrixOfFactor(Sources) if more Factor(Sources) are added.
    #[error(transparent)]
    NotYetValid { violation: E },
}

pub type AbstractFactorRulesValidation<E> =
    std::result::Result<(), FactorsInvalidReason<E>>;

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationPrimaryRoleInIsolation {
    #[error("Multiple Device Factors")]
    MultipleDeviceFactors,

    #[error("Threshold Factors must contain at least one other kind than Passphrase.")]
    ThresholdFactorsMustContainAtLeastOneOtherKindThanPassphrase,

    #[error("Threshold must be at least 2 when Passphrase is used.")]
    ThresholdMustBeAtLeastTwoWhenPassphraseIsUsed,

    #[error("Passphrase cannot be in override.")]
    PassphraseCannotBeInOverride,

    #[error("Security Questions cannot be used.")]
    SecurityQuestionsCannotBeUsed,
}

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationRecoveryRoleInIsolation {
    #[error("Contains Threshold Factors which is not allowed.")]
    RoleContainsThresholdFactors,

    #[error("Security Questions cannot be used.")]
    SecurityQuestionsCannotBeUsed,

    #[error("Passphrase cannot be used.")]
    PassphraseCannotBeUsed,
}

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationConfirmationRoleInIsolation {
    #[error("Contains Threshold Factors which is not allowed.")]
    RoleContainsThresholdFactors,
}

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationRoleInIsolation {
    #[error(transparent)]
    Primary(#[from] FactorRulesViolationPrimaryRoleInIsolation),
    #[error(transparent)]
    Recovery(#[from] FactorRulesViolationRecoveryRoleInIsolation),
    #[error(transparent)]
    Confirmation(#[from] FactorRulesViolationConfirmationRoleInIsolation),
}

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationOfStructure {
    #[error(transparent)]
    RoleInIsolation(#[from] FactorRulesViolationRoleInIsolation),
    #[error(transparent)]
    CombinedRulesViolation(#[from] FactorRulesViolationRolesCombined),
}

/// Each role in isolation is valid, but the combination of them is not.
#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolationRolesCombined {
    #[error("Unknown.")]
    Unknown,
}

impl From<FactorsInvalidReason<FactorRulesViolationRoleInIsolation>>
    for CommonError
{
    fn from(
        _err: FactorsInvalidReason<FactorRulesViolationRoleInIsolation>,
    ) -> Self {
        todo!()
    }
}

impl From<FactorsInvalidReason<FactorRulesViolationOfStructure>>
    for CommonError
{
    fn from(
        _err: FactorsInvalidReason<FactorRulesViolationOfStructure>,
    ) -> Self {
        todo!()
    }
}

impl From<FactorsInvalidReason<FactorRulesViolationRolesCombined>>
    for CommonError
{
    fn from(
        _err: FactorsInvalidReason<FactorRulesViolationRolesCombined>,
    ) -> Self {
        todo!()
    }
}

pub type PrimaryRoleInIsolationValidation =
    AbstractFactorRulesValidation<FactorRulesViolationPrimaryRoleInIsolation>;
pub type RecoveryRoleInIsolationValidation =
    AbstractFactorRulesValidation<FactorRulesViolationRecoveryRoleInIsolation>;
pub type ConfirmationRoleInIsolationValidation = AbstractFactorRulesValidation<
    FactorRulesViolationConfirmationRoleInIsolation,
>;

pub type RolesInIsolationValidation =
    AbstractFactorRulesValidation<FactorRulesViolationRoleInIsolation>;

pub type RolesCombinedValidation =
    AbstractFactorRulesValidation<FactorRulesViolationRolesCombined>;

pub type FactorRulesValidation =
    AbstractFactorRulesValidation<FactorRulesViolationOfStructure>;

pub trait AllowNotYetValid {
    fn allow_not_yet_valid(self) -> Self;
}
impl<E: Debug> AllowNotYetValid for AbstractFactorRulesValidation<E> {
    fn allow_not_yet_valid(self) -> Self {
        match self {
            Ok(_) => Ok(()),
            Err(e) => match e {
                FactorsInvalidReason::ForeverInvalid { violation } => {
                    Err(FactorsInvalidReason::ForeverInvalid { violation })
                }
                FactorsInvalidReason::NotYetValid { .. } => Ok(()),
            },
        }
    }
}

pub trait MapToRolesInIsolationValidation {
    fn into_roles(self) -> RolesInIsolationValidation;
}
impl<T: Debug + Into<FactorRulesViolationRoleInIsolation>>
    MapToRolesInIsolationValidation for AbstractFactorRulesValidation<T>
{
    fn into_roles(self) -> RolesInIsolationValidation {
        match self {
            Ok(_) => Ok(()),
            Err(e) => match e {
                FactorsInvalidReason::<T>::ForeverInvalid { violation } => {
                    Err(FactorsInvalidReason::<
                        FactorRulesViolationRoleInIsolation,
                    >::ForeverInvalid {
                        violation:
                            Into::<FactorRulesViolationRoleInIsolation>::into(
                                violation,
                            ),
                    })
                }
                FactorsInvalidReason::<T>::NotYetValid { violation } => {
                    Err(FactorsInvalidReason::<
                        FactorRulesViolationRoleInIsolation,
                    >::NotYetValid {
                        violation:
                            Into::<FactorRulesViolationRoleInIsolation>::into(
                                violation,
                            ),
                    })
                }
            },
        }
    }
}

pub trait MapToStructureValidation {
    fn into_structure(self) -> FactorRulesValidation;
}
impl<T: Debug + Into<FactorRulesViolationOfStructure>> MapToStructureValidation
    for AbstractFactorRulesValidation<T>
{
    fn into_structure(self) -> FactorRulesValidation {
        match self {
            Ok(_) => Ok(()),
            Err(e) => {
                match e {
                    FactorsInvalidReason::<T>::ForeverInvalid { violation } => {
                        Err(FactorsInvalidReason::<
                            FactorRulesViolationOfStructure,
                        >::ForeverInvalid {
                            violation:
                                Into::<FactorRulesViolationOfStructure>::into(
                                    violation,
                                ),
                        })
                    }
                    FactorsInvalidReason::<T>::NotYetValid { violation } => {
                        Err(FactorsInvalidReason::<
                            FactorRulesViolationOfStructure,
                        >::NotYetValid {
                            violation:
                                Into::<FactorRulesViolationOfStructure>::into(
                                    violation,
                                ),
                        })
                    }
                }
            }
        }
    }
}
