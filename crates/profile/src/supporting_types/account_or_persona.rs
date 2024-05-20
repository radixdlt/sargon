use crate::prelude::*;

/// Either an `Account` or a `Persona`.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
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

impl IsNetworkAware for AccountOrPersona {
    fn network_id(&self) -> NetworkID {
        match self {
            Self::AccountEntity(account) => account.network_id,
            Self::PersonaEntity(persona) => persona.network_id,
        }
    }
}

impl From<Account> for AccountOrPersona {
    fn from(value: Account) -> Self {
        Self::AccountEntity(value)
    }
}

impl From<Persona> for AccountOrPersona {
    fn from(value: Persona) -> Self {
        Self::PersonaEntity(value)
    }
}

impl std::fmt::Display for AccountOrPersona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountEntity(value) => write!(f, "{}", value),
            Self::PersonaEntity(value) => write!(f, "{}", value),
        }
    }
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

impl HasSampleValues for AccountOrPersona {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl AccountOrPersona {
    pub fn sample_mainnet() -> Self {
        Self::from(Account::sample_mainnet())
    }

    pub fn sample_mainnet_other() -> Self {
        Self::from(Persona::sample_mainnet_other())
    }

    pub fn sample_mainnet_third() -> Self {
        Self::from(Account::sample_mainnet_third())
    }

    pub fn sample_stokenet() -> Self {
        Self::from(Account::sample_stokenet())
    }

    pub fn sample_stokenet_other() -> Self {
        Self::from(Persona::sample_stokenet_other())
    }

    pub fn sample_stokenet_third() -> Self {
        Self::from(Account::sample_stokenet_third())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrPersona;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn test_id() {
        assert_eq!(SUT::sample().id(), Account::sample().address.into());
        assert_eq!(
            SUT::sample_other().id(),
            Persona::sample_mainnet_other().address.into()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SUT::sample()), "Alice | account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");
        assert_eq!(
            format!("{}", SUT::sample_other()),
            "Batman | identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
        );
    }
}
