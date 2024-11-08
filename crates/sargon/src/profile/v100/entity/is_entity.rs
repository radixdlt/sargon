use crate::prelude::*;

pub trait HasSecurityState {
    fn security_state(&self) -> EntitySecurityState;
    fn try_get_secured_control(&self) -> Result<SecuredEntityControl> {
        self.security_state()
            .as_securified()
            .cloned()
            .ok_or(CommonError::SecurityStateNotSecurified)
    }

    fn try_get_unsecured_control(&self) -> Result<UnsecuredEntityControl> {
        self.security_state()
            .as_unsecured()
            .cloned()
            .ok_or(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
    }
}

pub trait IsBaseEntity:
    HasEntityKindObjectSafe + IsNetworkAware + HasSecurityState
{
    type Address: IsBaseEntityAddress
        + PartialEq
        + Eq
        + std::hash::Hash
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

pub trait IsEntity:
    IsBaseEntity
    + HasEntityKind
    + std::hash::Hash
    + Eq
    + std::fmt::Debug
    + Clone
    + TryFrom<AccountOrPersona, Error = CommonError>
{
    type Path: IsEntityPath;
    fn with_veci_and_name(
        veci: HDFactorInstanceTransactionSigning<Self::Path>,
        name: DisplayName,
    ) -> Self;
}
