use crate::{
    create_application_for_securified_entity_with_manifests, prelude::*,
};

/// Security shield application is the application of a security shield
/// to some entity. This entity can be either an account or a persona.
/// Essentially holds four
/// different kinds of value, application for:
/// - unsecurified account
/// - unsecurified persona
/// - securified account
/// - securified persona
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplication {
    /// Application for an unsecurified entity.
    ForUnsecurifiedEntity(SecurityShieldApplicationForUnsecurifiedEntity),

    /// Application for a securified entity.
    ForSecurifiedEntity(SecurityShieldApplicationForSecurifiedEntity),
}

/// An application of a security shield to an unsecurified entity
/// holds a single manifest for exercising the primary role (since
/// no other roles are available for unsecurified entities).
///
/// Split into Account vs Persona since for Persona a TX fee payer
/// and AccessControl XRD vault top-up account is needed.
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForUnsecurifiedEntity {
    /// Application for an unsecurified account.
    Account(SecurityShieldApplicationForUnsecurifiedAccount),
    /// Application for an unsecurified persona - the associated type
    /// specifies the account that will pay the TX fee and top up the
    /// AccessControl XRD vault.
    Persona(SecurityShieldApplicationForUnsecurifiedPersona),
}
impl SecurityShieldApplicationForUnsecurifiedEntity {
    pub fn manifest(&self) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) => {
                &a.modified_manifest
            }
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => {
                &p.modified_manifest
            }
        }
    }
}

create_application_for_securified_entity_with_manifests! {
    SecurityShieldApplicationForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}

create_application_for_securified_entity_with_manifests! {
    SecurityShieldApplicationForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}

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
