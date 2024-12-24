use crate::prelude::*;

/// Either an `Account` or a `Persona`.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, EnumAsInner,
)]
pub enum AccountOrPersona {
    /// An `Account`
    AccountEntity(Account),

    /// A `Persona`
    PersonaEntity(Persona),
}

impl HasEntityKindObjectSafe for AccountOrPersona {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        match self {
            Self::AccountEntity(account) => account.get_entity_kind(),
            Self::PersonaEntity(persona) => persona.get_entity_kind(),
        }
    }
}
impl HasSecurityState for AccountOrPersona {
    fn security_state(&self) -> EntitySecurityState {
        match self {
            Self::AccountEntity(a) => a.security_state(),
            Self::PersonaEntity(p) => p.security_state(),
        }
    }
    fn set_security_state_unchecked(&mut self, new_state: EntitySecurityState) {
        match self {
            Self::AccountEntity(a) => a.set_security_state_unchecked(new_state),
            Self::PersonaEntity(p) => p.set_security_state_unchecked(new_state),
        }
    }
}

impl IsKeySpaceAware for AccountOrPersona {
    fn key_space(&self) -> KeySpace {
        if self.security_state().is_securified() {
            KeySpace::Securified
        } else if self.is_unsecurified(IsHardened(true)) {
            KeySpace::Unsecurified { is_hardened: true }
        } else if self.is_unsecurified(IsHardened(false)) {
            KeySpace::Unsecurified { is_hardened: false }
        } else {
            unreachable!("should never happen")
        }
    }
}

impl AccountOrPersona {
    pub fn is_unsecurified(&self, is_hardened: IsHardened) -> bool {
        match self.security_state() {
            EntitySecurityState::Unsecured { value: uec } => {
                uec.transaction_signing
                    .derivation_path()
                    .index()
                    .is_hardened()
                    == is_hardened.0
            }
            _ => false,
        }
    }

    pub fn matches_key_space(&self, key_space: KeySpace) -> bool {
        match key_space {
            KeySpace::Securified => self.is_securified(),
            KeySpace::Unsecurified { is_hardened } => {
                self.is_unsecurified(IsHardened(is_hardened))
            }
        }
    }
}

impl IsBaseEntity for AccountOrPersona {
    type Address = AddressOfAccountOrPersona;

    fn address(&self) -> Self::Address {
        match self {
            Self::AccountEntity(account) => {
                AddressOfAccountOrPersona::Account(account.address)
            }
            Self::PersonaEntity(persona) => {
                AddressOfAccountOrPersona::Identity(persona.address)
            }
        }
    }

    fn flags(&self) -> EntityFlags {
        match self {
            Self::AccountEntity(a) => a.flags.clone(),
            Self::PersonaEntity(p) => p.flags.clone(),
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

    pub fn entity_security_state(&self) -> EntitySecurityState {
        match self {
            AccountOrPersona::AccountEntity(account) => {
                account.security_state.clone()
            }
            AccountOrPersona::PersonaEntity(persona) => {
                persona.security_state.clone()
            }
        }
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        match self {
            AccountOrPersona::AccountEntity(account) => {
                AddressOfAccountOrPersona::Account(account.address)
            }
            AccountOrPersona::PersonaEntity(persona) => {
                AddressOfAccountOrPersona::Identity(persona.address)
            }
        }
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
    fn get_entity_kind() {
        assert_eq!(SUT::sample().get_entity_kind(), CAP26EntityKind::Account);
        assert_eq!(
            SUT::sample_other().get_entity_kind(),
            CAP26EntityKind::Identity
        );
    }

    #[test]
    fn get_flags() {
        assert_eq!(SUT::sample().flags(), Account::sample_mainnet().flags());
        assert_eq!(
            SUT::sample_other().flags(),
            Persona::sample_mainnet_other().flags()
        );
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
}
