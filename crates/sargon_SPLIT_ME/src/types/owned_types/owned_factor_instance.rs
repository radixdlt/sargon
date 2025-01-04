use std::borrow::Borrow;

use crate::prelude::*;

/// A `HierarchicalDeterministicFactorInstance` with a known owner - an account or persona.
pub type OwnedFactorInstance = Owned<HierarchicalDeterministicFactorInstance>;

impl OwnedFactorInstance {
    /// Constructs a new `OwnedFactorInstance`.
    pub fn owned_factor_instance(
        owner: AddressOfAccountOrPersona,
        factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(owner, factor_instance)
    }

    /// The owned `HierarchicalDeterministicFactorInstance`, the value of this `OwnedFactorInstance`.
    pub fn factor_instance(&self) -> &HierarchicalDeterministicFactorInstance {
        &self.value
    }

    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.factor_instance().factor_source_id
    }

    /// Checks if this `OwnedFactorInstance` was created from the factor source
    /// with id `factor_source_id`.
    pub fn by_factor_source(
        &self,
        factor_source_id: impl Borrow<FactorSourceIDFromHash>,
    ) -> bool {
        let factor_source_id = factor_source_id.borrow();
        self.factor_source_id() == *factor_source_id
    }
}

impl From<OwnedFactorInstance> for HierarchicalDeterministicFactorInstance {
    fn from(value: OwnedFactorInstance) -> Self {
        value.value
    }
}
