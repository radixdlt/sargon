use crate::prelude::*;
use sargon::AccountOrPersona as InternalAccountOrPersona;
use sargon::Identifiable;

/// Either an `Account` or a `Persona`.
#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Enum)]
pub enum AccountOrPersona {
    /// An `Account`
    ///
    /// Note:
    /// This case/variant can not be named `account`/ `Account` due
    /// to Kotlin UniFFI limitation.
    AccountEntity(Account),

    /// A `Persona`
    ///
    /// Note:
    /// This is named `personaEntity` / `PersonaEntity` to match
    /// `accountEntity` / `AccountEntity` which can not be named
    /// `account`/ `Account` due to Kotlin UniFFI limitation.
    PersonaEntity(Persona),
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet() -> AccountOrPersona {
    InternalAccountOrPersona::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet_other() -> AccountOrPersona {
    InternalAccountOrPersona::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet_third() -> AccountOrPersona {
    InternalAccountOrPersona::sample_mainnet_third().into()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet() -> AccountOrPersona {
    InternalAccountOrPersona::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet_other() -> AccountOrPersona {
    InternalAccountOrPersona::sample_stokenet_other().into()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet_third() -> AccountOrPersona {
    InternalAccountOrPersona::sample_stokenet_third().into()
}

#[uniffi::export]
pub fn account_or_persona_get_id(
    entity: &AccountOrPersona,
) -> AddressOfAccountOrPersona {
    entity.into_internal().id().into()
}
