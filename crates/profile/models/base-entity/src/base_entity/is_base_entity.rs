use crate::prelude::*;

pub trait HasEntityAddress {
    fn address_erased(&self) -> AddressOfAccountOrPersona;
}

impl<T: IsBaseEntity> HasEntityAddress for T {
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        self.address().into()
    }
}

/// A trait bridging AccountOrPersona, Account and Persona.
pub trait IsBaseEntity:
    HasEntityKindObjectSafe + IsNetworkAware + HasSecurityState
{
    type Address: IsBaseEntityAddress
        + PartialEq
        + Eq
        + std::hash::Hash
        + std::fmt::Display
        + Into<AddressOfAccountOrPersona>
        + std::fmt::Debug;

    fn address(&self) -> Self::Address;

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    fn flags(&self) -> EntityFlags;

    fn is_hidden(&self) -> bool {
        self.flags().into_iter().contains(&EntityFlag::HiddenByUser)
    }

    fn is_tombstoned(&self) -> bool {
        self.flags()
            .into_iter()
            .contains(&EntityFlag::TombstonedByUser)
    }
}

impl<T: HasSecurityState> HasProvisionalSecurifiedConfig for T {
    fn get_provisional(&self) -> Option<ProvisionalSecurifiedConfig> {
        self.security_state().get_provisional()
    }

    fn set_provisional(
        &mut self,
        provisional_securified_config: impl Into<
            Option<ProvisionalSecurifiedConfig>,
        >,
    ) {
        let mut security_state = self.security_state();
        security_state.set_provisional(provisional_securified_config);
        self.set_security_state_unchecked(security_state);
    }
}

/// A trait bridging Account and Persona.
pub trait IsEntityWithoutConcreteTypes:
    IsBaseEntity
    + HasEntityKind
    + Identifiable
    + std::hash::Hash
    + Eq
    + std::fmt::Debug
    + Clone
{
    type Path: IsEntityPath;

    fn with_veci_and_name(
        veci: HDFactorInstanceTransactionSigning<Self::Path>,
        name: DisplayName,
    ) -> Self;
}
