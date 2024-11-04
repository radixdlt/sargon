use std::hash::Hash;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssertMatches {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub entity_kind: CAP26EntityKind,
    pub key_space: KeySpace,
}
impl AssertMatches {
    pub fn matches(&self, path: &DerivationPath) -> DerivationPath {
        assert_eq!(self.entity_kind, path.get_entity_kind());
        assert_eq!(self.network_id, path.network_id());
        assert_eq!(self.key_kind, path.get_key_kind());
        assert_eq!(self.key_space, path.key_space());
        path.clone()
    }
}
trait HighestDerivationPathIndex {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent>;
}
impl HighestDerivationPathIndex for MatrixOfFactorInstances {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        // self.all_factors()
        //     .into_iter()
        //     .filter(|f| f.factor_source_id == factor_source_id)
        //     .map(|f| f.derivation_path())
        //     .map(|p| assert_matches.matches(&p))
        //     .map(|p| p.index)
        //     .max()
        todo!()
    }
}
impl HighestDerivationPathIndex for SecuredEntityControl {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        // self.matrix
        // .highest_derivation_path_index(factor_source_id, assert_matches)
        todo!()
    }
}

pub trait IsSecurifiedEntity:
    Hash + Eq + Clone + IsNetworkAware + HasEntityKind + TryFrom<AccountOrPersona>
{
    type BaseEntity: IsEntity + std::hash::Hash + Eq;

    fn securified_entity_control(&self) -> SecuredEntityControl;

    fn new(
        name: impl Into<DisplayName>,
        address: <Self::BaseEntity as IsEntity>::Address,
        securified_entity_control: SecuredEntityControl,
    ) -> Self;

    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.securified_entity_control()
            .highest_derivation_path_index(factor_source_id, assert_matches)
    }
}
