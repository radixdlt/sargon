use std::hash::Hash;

use crate::prelude::*;

pub trait IsSecurifiedEntity: Hash + Eq + Clone + IsNetworkAware {
    type BaseEntity: IsBaseBaseEntity + std::hash::Hash + Eq;

    fn securified_entity_control(&self) -> SecuredEntityControl;

    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.securified_entity_control()
            .highest_derivation_path_index(factor_source_id, assert_matches)
    }
}
