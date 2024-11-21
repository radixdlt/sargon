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

impl MatrixOfFactorInstances {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.all_factors()
            .into_iter()
            .flat_map(|f| f.try_as_hd_factor_instances().ok())
            .filter(|f| f.factor_source_id == factor_source_id)
            .map(|f| f.derivation_path())
            .map(|p| assert_matches.matches(&p))
            .map(|p| p.index())
            .max()
    }
}

impl SecuredEntityControl {
    pub fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.security_structure
            .matrix_of_factors
            .highest_derivation_path_index(factor_source_id, assert_matches)
    }
}
