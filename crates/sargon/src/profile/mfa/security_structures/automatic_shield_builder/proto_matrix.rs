use crate::prelude::*;

use RoleKind::*;

/// A tiny holder of factors for each Role.
/// Used by the AutomaticShieldBuilder to keep track of which factors are assigned to which role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ProtoMatrix {
    pub(super) primary: IndexSet<FactorSourceID>,
    pub(super) recovery: IndexSet<FactorSourceID>,
    pub(super) confirmation: IndexSet<FactorSourceID>,
}

impl ProtoMatrix {
    pub(super) fn new(primary: IndexSet<FactorSourceID>) -> Self {
        Self {
            primary,
            recovery: IndexSet::new(),
            confirmation: IndexSet::new(),
        }
    }

    pub(super) fn factors_for_role(
        &self,
        role: RoleKind,
    ) -> &IndexSet<FactorSourceID> {
        match role {
            Primary => &self.primary,
            Recovery => &self.recovery,
            Confirmation => &self.confirmation,
        }
    }

    pub(super) fn add_factors_for_role(
        &mut self,
        role: RoleKind,
        factors: IndexSet<FactorSourceID>,
    ) {
        match role {
            Primary => self.primary.extend(factors),
            Recovery => self.recovery.extend(factors),
            Confirmation => self.confirmation.extend(factors),
        }
    }
}