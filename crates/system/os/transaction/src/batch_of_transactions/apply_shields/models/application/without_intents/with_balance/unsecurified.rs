use crate::prelude::*;

// ========================
// UNSECURIFIED
// ========================

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForUnsecurifiedEntity {
    Account(ApplicationInputForUnsecurifiedAccount),
    Persona(ApplicationInputForUnsecurifiedPersona),
}

impl From<ApplicationInputForUnsecurifiedAccount>
    for ApplicationInputForUnsecurifiedEntity
{
    fn from(value: ApplicationInputForUnsecurifiedAccount) -> Self {
        Self::Account(value)
    }
}
impl From<ApplicationInputForUnsecurifiedPersona>
    for ApplicationInputForUnsecurifiedEntity
{
    fn from(value: ApplicationInputForUnsecurifiedPersona) -> Self {
        Self::Persona(value)
    }
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedAccount {
    pub entity_input: UnsecurifiedAccountEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedPersona {
    pub entity_input: UnsecurifiedPersona,
    pub paying_account: ApplicationInputPayingAccount,
}

// ========================
// ENTITY INPUT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsecurifiedAccountEntityInput {
    /// The entity applying the shield
    pub unsecurified_entity: UnsecurifiedAccount,

    /// XRD balance of the entity applying the shield
    pub xrd_balance_of_account: Decimal,
}
