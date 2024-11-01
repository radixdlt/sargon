use crate::prelude::*;

use super::quantities;

/// A DerivationPath which is not indexed. On a specific network.
#[derive(
    Clone, Copy, Hash, PartialEq, Eq, derive_more::Debug, derive_more::Display,
)]
#[display("{}/{}/{}/?{}", network_id, entity_kind, key_kind, key_space)]
#[debug("{:?}/{:?}/{:?}/?{}", network_id, entity_kind, key_kind, key_space)]
pub struct IndexAgnosticPath {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub key_space: KeySpace,
}

impl IndexAgnosticPath {
    pub fn new(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        key_space: KeySpace,
    ) -> Self {
        Self {
            network_id,
            entity_kind,
            key_kind,
            key_space,
        }
    }
}

impl From<(NetworkID, DerivationPreset)> for IndexAgnosticPath {
    fn from(
        (network_id, agnostic_path): (NetworkID, DerivationPreset),
    ) -> Self {
        Self::new(
            network_id,
            agnostic_path.entity_kind(),
            agnostic_path.key_kind(),
            agnostic_path.key_space(),
        )
    }
}

impl TryFrom<IndexAgnosticPath> for DerivationPreset {
    type Error = CommonError;
    /// Tries to convert an IndexAgnosticPath to a DerivationPreset,
    /// is failing if the path is not a standard DerivationPreset
    fn try_from(value: IndexAgnosticPath) -> Result<DerivationPreset> {
        match (value.entity_kind, value.key_kind, value.key_space) {
            (
                CAP26EntityKind::Account,
                CAP26KeyKind::TransactionSigning,
                KeySpace::Unsecurified { is_hardened: true },
            ) => Ok(DerivationPreset::AccountVeci),
            (
                CAP26EntityKind::Identity,
                CAP26KeyKind::TransactionSigning,
                KeySpace::Unsecurified { is_hardened: true },
            ) => Ok(DerivationPreset::IdentityVeci),
            (
                CAP26EntityKind::Account,
                CAP26KeyKind::TransactionSigning,
                KeySpace::Securified,
            ) => Ok(DerivationPreset::AccountMfa),
            (
                CAP26EntityKind::Identity,
                CAP26KeyKind::TransactionSigning,
                KeySpace::Securified,
            ) => Ok(DerivationPreset::IdentityMfa),
            _ => Err(CommonError::InvalidBIP32Path {
                bad_value:
                    "Invalid combination of entity_kind, key_kind and key_space"
                        .to_string(),
            }),
        }
    }
}

impl From<(IndexAgnosticPath, HDPathComponent)> for DerivationPath {
    fn from((path, index): (IndexAgnosticPath, HDPathComponent)) -> Self {
        assert_eq!(index.key_space(), path.key_space);
        let hardened =
            Hardened::try_from(index).expect("Expected only hardened indices.");
        match path.entity_kind {
            CAP26EntityKind::Account => Self::account(AccountPath::new(
                path.network_id,
                path.key_kind,
                hardened,
            )),
            CAP26EntityKind::Identity => Self::identity(IdentityPath::new(
                path.network_id,
                path.key_kind,
                hardened,
            )),
        }
    }
}

impl AccountPath {
    pub fn agnostic(&self) -> IndexAgnosticPath {
        IndexAgnosticPath {
            network_id: self.network_id,
            entity_kind: self.get_entity_kind(),
            key_kind: self.key_kind,
            key_space: self.index.key_space(),
        }
    }
}

impl IdentityPath {
    pub fn agnostic(&self) -> IndexAgnosticPath {
        IndexAgnosticPath {
            network_id: self.network_id,
            entity_kind: self.get_entity_kind(),
            key_kind: self.key_kind,
            key_space: self.index.key_space(),
        }
    }
}

impl DerivationPath {
    pub fn agnostic(&self) -> IndexAgnosticPath {
        match self {
            DerivationPath::Account { value } => value.agnostic(),
            DerivationPath::Identity { value } => value.agnostic(),
            DerivationPath::Bip44Like { value: _ } => {
                panic!("Bip44Like paths are not supported")
            }
        }
    }
}
impl HierarchicalDeterministicFactorInstance {
    pub fn agnostic_path(&self) -> IndexAgnosticPath {
        self.derivation_path().agnostic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = IndexAgnosticPath;

    #[test]
    fn try_from_success() {
        NetworkID::all().into_iter().for_each(|n| {
            let f = |preset: DerivationPreset| {
                let sut = preset.index_agnostic_path_on_network(n);
                let back_again = DerivationPreset::try_from(sut).unwrap();
                assert_eq!(back_again, preset);
            };

            DerivationPreset::all().into_iter().for_each(|p| f(p));
        });
    }

    #[test]
    fn try_from_fail() {
        let path = Sut::new(
            NetworkID::Stokenet,
            CAP26EntityKind::Account,
            CAP26KeyKind::AuthenticationSigning,
            KeySpace::Unsecurified { is_hardened: true },
        );
        assert!(DerivationPreset::try_from(path).is_err());
    }
}
