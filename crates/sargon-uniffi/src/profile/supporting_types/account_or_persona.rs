use crate::prelude::*;

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

impl Identifiable for AccountOrPersona {
    type ID = AddressOfAccountOrPersona;

    fn id(&self) -> Self::ID {
        match self {
            Self::AccountEntity(account) => {
                AddressOfAccountOrPersona::Account(account.address)
            }
            Self::PersonaEntity(persona) => {
                AddressOfAccountOrPersona::Identity(persona.address)
            }
        }
    }
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet() -> AccountOrPersona {
    AccountOrPersona::sample_mainnet()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet_other() -> AccountOrPersona {
    AccountOrPersona::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_mainnet_third() -> AccountOrPersona {
    AccountOrPersona::sample_mainnet_third()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet() -> AccountOrPersona {
    AccountOrPersona::sample_stokenet()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet_other() -> AccountOrPersona {
    AccountOrPersona::sample_stokenet_other()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_stokenet_third() -> AccountOrPersona {
    AccountOrPersona::sample_stokenet_third()
}

#[uniffi::export]
pub fn account_or_persona_get_id(
    entity: &AccountOrPersona,
) -> <AccountOrPersona as Identifiable>::ID {
    entity.id()
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
