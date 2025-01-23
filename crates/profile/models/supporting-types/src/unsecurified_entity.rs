use crate::prelude::*;

/// An unsecurified entity

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
pub struct AbstractUnsecurifiedEntity<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> where
    E::Address: Into<AddressOfAccountOrPersona>,
{
    pub entity: E,
    unsecured_entity_control: UnsecuredEntityControl,
    veci: VirtualEntityCreatingInstance,
    pub provisional_securified_config: Option<ProvisionalSecurifiedConfig>,
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> Identifiable
    for AbstractUnsecurifiedEntity<E>
where
    E::Address: Into<AddressOfAccountOrPersona>,
{
    type ID = AddressOfAccountOrPersona;
    fn id(&self) -> Self::ID {
        self.entity.address().into()
    }
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone>
    AbstractUnsecurifiedEntity<E>
where
    E::Address: Into<AddressOfAccountOrPersona>,
{
    pub fn with_unsecured_entity_control(
        entity: E,
        unsecured_entity_control: UnsecuredEntityControl,
    ) -> Self {
        Self {
            entity: entity.clone(),
            unsecured_entity_control: unsecured_entity_control.clone(),
            veci: VirtualEntityCreatingInstance::new(
                unsecured_entity_control.transaction_signing,
                Into::<AddressOfAccountOrPersona>::into(entity.address()),
            ),
            provisional_securified_config: unsecured_entity_control
                .provisional_securified_config,
        }
    }

    /// # Throws
    /// Throws if the entity is securified
    pub fn new(entity: E) -> Result<Self> {
        match entity.security_state() {
            EntitySecurityState::Unsecured {
                value: unsecured_entity_control,
            } => Ok(Self::with_unsecured_entity_control(
                entity,
                unsecured_entity_control,
            )),
            EntitySecurityState::Securified { .. } => {
                Err(CommonError::AESDecryptionFailed)
            }
        }
    }

    pub fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.veci.clone().address()
    }

    pub fn veci(&self) -> VirtualEntityCreatingInstance {
        self.veci.clone()
    }
}

pub type AnyUnsecurifiedEntity = AbstractUnsecurifiedEntity<AccountOrPersona>;

pub type UnsecurifiedAccount = AbstractUnsecurifiedEntity<Account>;
pub type UnsecurifiedPersona = AbstractUnsecurifiedEntity<Persona>;

impl TryFrom<AnyUnsecurifiedEntity> for UnsecurifiedAccount {
    type Error = CommonError;

    fn try_from(value: AnyUnsecurifiedEntity) -> Result<Self> {
        match value.entity {
            AccountOrPersona::AccountEntity(account) => Self::new(account),
            AccountOrPersona::PersonaEntity(_) => {
                Err(CommonError::ExpectedAccountButGotPersona {
                    address: value.entity.address().to_string(),
                })
            }
        }
    }
}
impl From<UnsecurifiedAccount> for AnyUnsecurifiedEntity {
    fn from(value: UnsecurifiedAccount) -> Self {
        Self::with_unsecured_entity_control(
            value.entity.into(),
            value.unsecured_entity_control,
        )
    }
}

impl From<UnsecurifiedPersona> for AnyUnsecurifiedEntity {
    fn from(value: UnsecurifiedPersona) -> Self {
        Self::with_unsecured_entity_control(
            value.entity.into(),
            value.unsecured_entity_control,
        )
    }
}

impl TryFrom<AnyUnsecurifiedEntity> for UnsecurifiedPersona {
    type Error = CommonError;

    fn try_from(value: AnyUnsecurifiedEntity) -> Result<Self> {
        match value.entity {
            AccountOrPersona::PersonaEntity(persona) => Self::new(persona),
            AccountOrPersona::AccountEntity(_) => {
                Err(CommonError::ExpectedPersonaButGotAccount {
                    address: value.entity.address().to_string(),
                })
            }
        }
    }
}

impl HasSampleValues for AnyUnsecurifiedEntity {
    fn sample() -> Self {
        Self::new(Account::sample().into()).unwrap()
    }

    fn sample_other() -> Self {
        Self::new(Account::sample_other().into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AnyUnsecurifiedEntity;

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
    fn network_id() {
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Mainnet);
    }
}
