use crate::prelude::*;

/// A trait bridging AccountOrPersona, Account and Persona.
pub trait IsBaseEntity:
    HasEntityKindObjectSafe + IsNetworkAware + HasSecurityState
{
    type Address: IsBaseEntityAddress
        + PartialEq
        + Eq
        + std::hash::Hash
        + std::fmt::Display
        + std::fmt::Debug;

    fn address(&self) -> Self::Address;

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    fn flags(&self) -> EntityFlags;

    fn is_hidden(&self) -> bool {
        self.flags()
            .into_iter()
            .contains(&EntityFlag::DeletedByUser)
    }
}

impl<T: IsBaseEntity> IsNetworkAware for T {
    fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }
}

/// A trait bridging Account and Persona.
pub trait IsEntity:
    IsBaseEntity
    + HasEntityKind
    + Identifiable
    + std::hash::Hash
    + Eq
    + std::fmt::Debug
    + Clone
    + TryFrom<AccountOrPersona, Error = CommonError>
    + TryInto<Account>
    + TryInto<Persona>
    + Into<AccountOrPersona>
{
    type Path: IsEntityPath;

    fn profile_modified_event(
        is_update: bool,
        addresses: IndexSet<Self::Address>,
    ) -> Option<EventProfileModified>;

    fn with_veci_and_name(
        veci: HDFactorInstanceTransactionSigning<Self::Path>,
        name: DisplayName,
    ) -> Self;
}
impl TryInto<Account> for Persona {
    type Error = CommonError;

    fn try_into(self) -> Result<Account> {
        Err(CommonError::ExpectedAccountButGotPersona {
            address: self.address().to_string(),
        })
    }
}

impl TryInto<Persona> for Account {
    type Error = CommonError;

    fn try_into(self) -> Result<Persona> {
        Err(CommonError::ExpectedPersonaButGotAccount {
            address: self.address().to_string(),
        })
    }
}
