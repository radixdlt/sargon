use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssertMatches {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub entity_kind: CAP26EntityKind,
    pub key_space: KeySpace,
}

impl AssertMatches {
    /// Due to a legacy bug described in [VirtualEntityCreatingInstance::check_for_derivation_path_discrepancies]
    /// this assertion regarding entity kind should become a warning and should not fail
    /// any operation initiated by the user.
    pub fn matches(&self, path: &DerivationPath) -> DerivationPath {
        if self.entity_kind != path.get_entity_kind() {
            warn!(
                "Expected path should be of entity kind {} but received {}.",
                self.entity_kind,
                path.get_entity_kind()
            );
        }
        assert_eq!(self.network_id, path.network_id());
        assert_eq!(self.key_kind, path.get_key_kind());
        assert_eq!(self.key_space, path.key_space());
        path.clone()
    }
}

pub trait HighestDerivationPathIndex {
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
        highest_derivation_index_of(
            self.all_factors().into_iter().cloned(),
            factor_source_id,
            assert_matches,
        )
    }
}

pub fn highest_derivation_index_of(
    instances: impl IntoIterator<Item = impl Into<Option<FactorInstance>>>,
    factor_source_id: FactorSourceIDFromHash,
    assert_matches: AssertMatches,
) -> Option<HDPathComponent> {
    highest_derivation_index_of_hd_factors(
        instances
            .into_iter()
            .filter_map(Into::into)
            .flat_map(|f| f.try_as_hd_factor_instances().ok()),
        factor_source_id,
        assert_matches,
    )
}

pub fn highest_derivation_index_of_hd_factors(
    hd_instances: impl IntoIterator<
        Item = impl Into<Option<HierarchicalDeterministicFactorInstance>>,
    >,
    factor_source_id: FactorSourceIDFromHash,
    assert_matches: AssertMatches,
) -> Option<HDPathComponent> {
    hd_instances
        .into_iter()
        .filter_map(Into::into)
        .filter(|f| f.factor_source_id == factor_source_id)
        .map(|f| f.derivation_path())
        .map(|p| assert_matches.matches(&p))
        .map(|p| p.index())
        .max()
}

pub trait MaxDerivationEntityQuerying {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent>;
}

impl MaxDerivationEntityQuerying for ProvisionalSecurifiedConfig {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        match self {
            ProvisionalSecurifiedConfig::FactorInstancesDerived { value } => {
                value.highest_derivation_path_index(
                    factor_source_id,
                    assert_matches,
                )
            }
        }
    }
}

impl MaxDerivationEntityQuerying for SecurityStructureOfFactorInstances {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.matrix_of_factors
            .highest_derivation_path_index(factor_source_id, assert_matches)
    }
}

impl HighestDerivationPathIndex for SecuredEntityControl {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        let committed = self
            .security_structure
            .matrix_of_factors
            .highest_derivation_path_index(factor_source_id, assert_matches);

        let provisional =
            self.provisional_securified_config.as_ref().and_then(|psc| {
                psc.highest_derivation_path_index(
                    factor_source_id,
                    assert_matches,
                )
            });

        committed.max(provisional)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssertMatches;

    #[test]
    fn test_assert_matches_derivation_path_with_different_entity_kind_does_not_fail(
    ) {
        let sut = SUT {
            network_id: NetworkID::Mainnet,
            key_kind: CAP26KeyKind::TransactionSigning,
            entity_kind: CAP26EntityKind::Identity,
            key_space: KeySpace::Unsecurified { is_hardened: true },
        };

        let derivation_path_with_bug = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_local_key_space(0u32, IsSecurified(false)).unwrap(),
        )
        .derivation_path();

        let verified_path = sut.matches(&derivation_path_with_bug);

        assert_eq!(verified_path, derivation_path_with_bug)
    }
}
