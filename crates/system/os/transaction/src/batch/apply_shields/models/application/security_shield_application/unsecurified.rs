use crate::prelude::*;

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

impl From<SecurityShieldApplicationForUnsecurifiedAccount>
    for SecurityShieldApplicationForUnsecurifiedEntity
{
    fn from(account: SecurityShieldApplicationForUnsecurifiedAccount) -> Self {
        Self::Account(account)
    }
}
impl From<SecurityShieldApplicationForUnsecurifiedPersona>
    for SecurityShieldApplicationForUnsecurifiedEntity
{
    fn from(persona: SecurityShieldApplicationForUnsecurifiedPersona) -> Self {
        Self::Persona(persona)
    }
}
