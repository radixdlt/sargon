use crate::prelude::*;

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

    pub fn fee_tip(&self) -> Option<Decimal> {
        match self {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) => {
                a.fee_tip()
            }
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => {
                p.fee_tip()
            }
        }
    }
}

pub type SecurityShieldApplicationForSecurifiedAccount =
    AbstractSecurityShieldApplicationForSecurifiedEntityWithManifest<
        SecurityShieldApplicationForSecurifiedAccountWithPayingAccount,
    >;

pub type SecurityShieldApplicationForSecurifiedPersona =
    AbstractSecurityShieldApplicationForSecurifiedEntityWithManifest<
        SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount,
    >;

pub type AbstractSecurityShieldApplicationForSecurifiedEntityWithManifest<E> =
    AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
        E,
        TransactionManifest,
    >;
