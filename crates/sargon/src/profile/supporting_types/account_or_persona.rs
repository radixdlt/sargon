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

impl HasEntitySecurityState for AccountOrPersona {
    fn security_state(&self) -> EntitySecurityState {
        match self {
            Self::AccountEntity(account) => account.security_state.clone(),
            Self::PersonaEntity(persona) => persona.security_state.clone(),
        }
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
    pub(crate) fn sample_mainnet() -> Self {
        Self::from(Account::sample_mainnet())
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::from(Persona::sample_mainnet_other())
    }

    pub(crate) fn sample_mainnet_third() -> Self {
        Self::from(Account::sample_mainnet_third())
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::from(Account::sample_stokenet())
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::from(Persona::sample_stokenet_other())
    }

    pub(crate) fn sample_stokenet_third() -> Self {
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
        assert_eq!(format!("{}", SUT::sample()), "Alice | account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87");
        assert_eq!(
            format!("{}", SUT::sample_other()),
            "Batman | identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw"
        );
    }

    #[test]
    fn test_virtual_hierarchical_deterministic_factor_instances() {
        // Transaction Signing factor instance
        let mut sut = SUT::sample();
        let mut factor_instances = sut
            .virtual_hierarchical_deterministic_factor_instances(
                CAP26KeyKind::TransactionSigning,
            );
        assert_eq!(factor_instances.len(), 1);
        assert_eq!(
            factor_instances.iter().next().unwrap().clone(),
            HierarchicalDeterministicFactorInstance::sample()
        );

        // No factor instances
        sut = SUT::sample_other();
        factor_instances = sut
            .virtual_hierarchical_deterministic_factor_instances(
                CAP26KeyKind::AuthenticationSigning,
            );
        assert!(factor_instances.is_empty());
    }
}
