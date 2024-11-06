use crate::prelude::*;

use super::quantities;

/// A DerivationPath which is not indexed. On a specific network.
#[derive(
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Debug,
    derive_more::Display,
)]
#[display("{}", self._to_str())]
#[debug("{}", self._to_str())]
pub struct IndexAgnosticPath {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub key_space: KeySpace,
}

impl FromStr for IndexAgnosticPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(HDPath::SEPARATOR).collect();
        if parts.len() != 4 {
            return Err(CommonError::Unknown);
        }
        let key_space_component = parts[3];
        if !key_space_component.ends_with(Self::COMPONENT_SUFFIX) {
            return Err(CommonError::Unknown);
        }
        let key_space_component =
            key_space_component.replace(Self::COMPONENT_SUFFIX, "");
        let key_space = KeySpace::from_str(&key_space_component)?;
        let parts = parts[..3].to_vec();
        let s = parts.join(HDPath::SEPARATOR);
        let hd_path = HDPath::from_bip32_string(s)?;
        let components = hd_path.components();

        let network_id = NetworkID::try_from(components[0])?;

        let entity_kind = CAP26EntityKind::try_from(components[1])?;
        let key_kind = CAP26KeyKind::try_from(components[2])?;

        Ok(Self::new(network_id, entity_kind, key_kind, key_space))
    }
}

impl IndexAgnosticPath {
    pub const COMPONENT_SUFFIX: &str = "?";
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

    fn _to_hd_path(&self) -> HDPath {
        index_agnostic(self.network_id, self.entity_kind, self.key_kind)
    }

    fn _to_str(&self) -> String {
        let base = self._to_hd_path().to_bip32_string_with(false);
        format!("{}/{}{}", base, self.key_space, Self::COMPONENT_SUFFIX)
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

impl HasSampleValues for IndexAgnosticPath {
    fn sample() -> Self {
        DerivationPreset::AccountVeci
            .index_agnostic_path_on_network(NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        DerivationPreset::IdentityMfa
            .index_agnostic_path_on_network(NetworkID::Stokenet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IndexAgnosticPath;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn try_from_success() {
        NetworkID::all().into_iter().for_each(|n| {
            let f = |preset: DerivationPreset| {
                let sut = preset.index_agnostic_path_on_network(n);
                let back_again = DerivationPreset::try_from(sut).unwrap();
                assert_eq!(back_again, preset);
            };

            DerivationPreset::all().into_iter().for_each(f);
        });
    }

    #[test]
    fn try_from_fail() {
        let path = SUT::new(
            NetworkID::Stokenet,
            CAP26EntityKind::Account,
            CAP26KeyKind::AuthenticationSigning,
            KeySpace::Unsecurified { is_hardened: true },
        );
        assert!(DerivationPreset::try_from(path).is_err());
    }

    #[test]
    fn string_round_trip() {
        let test = |sut: SUT| {
            let s = sut.to_string();
            let back_again = SUT::from_str(&s).unwrap();
            assert_eq!(sut, back_again);
        };
        test(SUT::sample());
        test(SUT::sample_other());
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample()), "hej")
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT = "Cool persona".parse().unwrap();

        assert_json_value_eq_after_roundtrip(&a, json!("Cool persona"));
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(&a, json!("Main account"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("   "));
    }
}
