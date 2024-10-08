use crate::prelude::*;
use sargon::AccountOrPersona as InternalAccountOrPersona;

/// Either an `Account` or a `Persona`.
#[derive(
    Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
)]
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

impl From<InternalAccountOrPersona> for AccountOrPersona {
    fn from(value: InternalAccountOrPersona) -> Self {
        match value {
            InternalAccountOrPersona::AccountEntity(account) => {
                AccountOrPersona::AccountEntity(account.into())
            }
            InternalAccountOrPersona::PersonaEntity(persona) => {
                AccountOrPersona::PersonaEntity(persona.into())
            }
        }
    }
}

impl Into<InternalAccountOrPersona> for AccountOrPersona {
    fn into(self) -> InternalAccountOrPersona {
        match self {
            AccountOrPersona::AccountEntity(account) => {
                InternalAccountOrPersona::AccountEntity(account.into())
            }
            AccountOrPersona::PersonaEntity(persona) => {
                InternalAccountOrPersona::PersonaEntity(persona.into())
            }
        }
    }
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
    entity.into::<InternalAccountOrPersona>().id().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrPersona;

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_or_persona_sample_mainnet(),
                new_account_or_persona_sample_mainnet_other(),
                new_account_or_persona_sample_mainnet_third(),
                new_account_or_persona_sample_stokenet(),
                new_account_or_persona_sample_stokenet_other(),
                new_account_or_persona_sample_stokenet_third(),
                // duplicates should be removed
                new_account_or_persona_sample_mainnet(),
                new_account_or_persona_sample_mainnet_other(),
                new_account_or_persona_sample_mainnet_third(),
                new_account_or_persona_sample_stokenet(),
                new_account_or_persona_sample_stokenet_other(),
                new_account_or_persona_sample_stokenet_third(),
            ])
            .len(),
            6
        )
    }

    #[test]
    fn test_get_id() {
        let sut = SUT::sample();
        assert_eq!(sut.id(), account_or_persona_get_id(&sut));
    }
}
