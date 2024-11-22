use crate::prelude::*;
use sargon::HierarchicalDeterministicFactorInstance as InternalHierarchicalDeterministicFactorInstance;
use sargon::Owned as InternalOwned;

type InternalOwnedFactorInstance =
    InternalOwned<InternalHierarchicalDeterministicFactorInstance>;

/// Concrete implementation of `sargon::Owned<HierarchicalDeterministicFactorInstance>`.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct OwnedFactorInstance {
    /// The known owner - an account or persona - of `value`.
    pub owner: AddressOfAccountOrPersona,
    /// An HD Factor Instance known to be owned by `owner` - an account or persona.
    pub factor_instance: HierarchicalDeterministicFactorInstance,
}

impl OwnedFactorInstance {

    pub fn into_internal(&self) -> InternalOwnedFactorInstance {
        self.clone().into()
    }

}

impl From<InternalOwnedFactorInstance> for OwnedFactorInstance {
    fn from(value: InternalOwnedFactorInstance) -> Self {
        Self {
            owner: value.owner.into(),
            factor_instance: value.value.into(),
        }
    }
}

impl From<OwnedFactorInstance> for InternalOwnedFactorInstance {
    fn from(value: OwnedFactorInstance) -> Self {
        Self::new(
            value.owner.into_internal(),
            value.factor_instance.into_internal(),
        )
    }
}
