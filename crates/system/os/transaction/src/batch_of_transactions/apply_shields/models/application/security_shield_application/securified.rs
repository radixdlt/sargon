use crate::prelude::*;

/// An application of a security shield to a securified entity
/// holds many tuples of manifests for each combination of role.
///
/// Split into Account vs Persona since for Persona a TX fee payer
/// and AccessControl XRD vault top-up account is needed.
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntity {
    /// Application for a securified account.
    Account(SecurityShieldApplicationForSecurifiedAccount),

    /// Application for a securified persona - the associated type
    /// specifies the account that will pay the TX fee and top up the
    /// AccessControl XRD vault.
    Persona(SecurityShieldApplicationForSecurifiedPersona),
}

impl SecurityShieldApplicationForSecurifiedEntity {
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                a.fee_tip_percentage()
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                p.fee_tip_percentage()
            }
        }
    }

    pub fn initiate_with_recovery_complete_with_confirmation(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_complete_with_confirmation
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_complete_with_confirmation
            }
        }
    }

    pub fn initiate_with_recovery_complete_with_primary(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_complete_with_primary
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_complete_with_primary
            }
        }
    }

    pub fn initiate_with_recovery_delayed_completion(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_delayed_completion
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_delayed_completion
            }
        }
    }

    pub fn initiate_with_primary_complete_with_confirmation(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_primary_complete_with_confirmation
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_primary_complete_with_confirmation
            }
        }
    }

    pub fn initiate_with_primary_delayed_completion(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_primary_delayed_completion
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_primary_delayed_completion
            }
        }
    }
}
