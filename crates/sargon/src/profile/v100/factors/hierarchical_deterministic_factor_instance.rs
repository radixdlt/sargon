use crate::prelude::*;
/// A virtual hierarchical deterministic `FactorInstance`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: HierarchicalDeterministicPublicKey,
}

impl HierarchicalDeterministicFactorInstance {
    pub fn derivation_path(&self) -> DerivationPath {
        self.public_key.derivation_path.clone()
    }

    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        public_key: HierarchicalDeterministicPublicKey,
    ) -> Self {
        Self {
            factor_source_id,
            public_key,
        }
    }

    pub fn with_key_and_path(
        factor_source_id: FactorSourceIDFromHash,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Self {
        Self::new(
            factor_source_id,
            HierarchicalDeterministicPublicKey::new(
                public_key,
                derivation_path,
            ),
        )
    }

    pub fn new_for_entity(
        factor_source_id: FactorSourceIDFromHash,
        entity_kind: CAP26EntityKind,
        index: HDPathComponent,
    ) -> Self {
        let derivation_path: DerivationPath = match entity_kind {
            CAP26EntityKind::Account => AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
            CAP26EntityKind::Identity => IdentityPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
        };

        let seed = factor_source_id.sample_associated_mnemonic().to_seed();
        let hd_private_key = seed.derive_private_key(&derivation_path);

        Self::new(factor_source_id, hd_private_key.public_key())
    }

    pub fn try_from(
        factor_source_id: FactorSourceID,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Result<Self> {
        let factor_source_id = factor_source_id
            .as_hash()
            .ok_or(CommonError::FactorSourceIDNotFromHash)?;
        Ok(Self::with_key_and_path(
            *factor_source_id,
            public_key,
            derivation_path,
        ))
    }

    pub fn try_from_factor_instance(
        factor_instance: FactorInstance,
    ) -> Result<Self> {
        let virtual_source = factor_instance
            .badge
            .as_virtual()
            .ok_or(CommonError::BadgeIsNotVirtualHierarchicalDeterministic)?;

        let badge = virtual_source.as_hierarchical_deterministic();

        Self::try_from(
            factor_instance.factor_source_id,
            badge.public_key,
            badge.derivation_path.clone(),
        )
    }

    pub fn factor_instance(&self) -> FactorInstance {
        FactorInstance::new(
            self.factor_source_id.into(),
            FactorInstanceBadge::Virtual {
                value: self.public_key.clone().into(),
            },
        )
    }

    pub fn key_kind(&self) -> Option<CAP26KeyKind> {
        match &self.derivation_path() {
            DerivationPath::CAP26 { value } => match value {
                CAP26Path::GetID { value: _ } => None,
                CAP26Path::Identity { value } => Some(value.key_kind()),
                CAP26Path::Account { value } => Some(value.key_kind()),
            },
            DerivationPath::BIP44Like { value: _ } => None,
        }
    }
}

impl Serialize for HierarchicalDeterministicFactorInstance {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.factor_instance().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for HierarchicalDeterministicFactorInstance {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        d: D,
    ) -> Result<HierarchicalDeterministicFactorInstance, D::Error> {
        FactorInstance::deserialize(d).and_then(|fi| {
            HierarchicalDeterministicFactorInstance::try_from_factor_instance(
                fi,
            )
            .map_err(de::Error::custom)
        })
    }
}

impl HasSampleValues for HierarchicalDeterministicFactorInstance {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_transaction_signing_0()
    }

    fn sample_other() -> Self {
        Self::sample_transaction_signing_1()
    }
}

impl HierarchicalDeterministicFactorInstance {
    /// A sample used to facilitate unit tests.
    pub fn sample_transaction_signing_0() -> Self {
        Self::sample_with_key_kind(CAP26KeyKind::TransactionSigning, 0)
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_transaction_signing_1() -> Self {
        Self::sample_with_key_kind(CAP26KeyKind::TransactionSigning, 1)
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_auth_signing() -> Self {
        Self::sample_with_key_kind(CAP26KeyKind::AuthenticationSigning, 0)
    }

    /// A sample used to facilitate unit tests.
    fn sample_with_key_kind(
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self {
        let path = AccountPath::new(NetworkID::Mainnet, key_kind, index);
        let mwp = MnemonicWithPassphrase::sample();
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(&path);
        let public_key = private_key.public_key();
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            &mwp,
        );
        Self::new(id, public_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(
            HierarchicalDeterministicFactorInstance::sample(),
            HierarchicalDeterministicFactorInstance::sample()
        );
        assert_eq!(
            HierarchicalDeterministicFactorInstance::sample_other(),
            HierarchicalDeterministicFactorInstance::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            HierarchicalDeterministicFactorInstance::sample(),
            HierarchicalDeterministicFactorInstance::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = HierarchicalDeterministicFactorInstance::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"badge": {
					"virtualSource": {
						"hierarchicalDeterministicPublicKey": {
							"publicKey": {
								"curve": "curve25519",
								"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
							},
							"derivationPath": {
								"scheme": "cap26",
								"path": "m/44H/1022H/1H/525H/1460H/0H"
							}
						},
						"discriminator": "hierarchicalDeterministicPublicKey"
					},
					"discriminator": "virtualSource"
				},
				"factorSourceID": {
					"fromHash": {
						"kind": "device",
						"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
					},
					"discriminator": "fromHash"
				}
			}
            "#,
        );
    }

    #[test]
    fn key_kind_bip44_is_none() {
        let derivation_path: DerivationPath = BIP44LikePath::sample().into();
        let sut = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            HierarchicalDeterministicPublicKey::new(
                PublicKey::sample_ed25519(),
                derivation_path,
            ),
        );
        assert_eq!(sut.key_kind(), None);
    }

    #[test]
    fn key_kind_identity() {
        let derivation_path: DerivationPath = IdentityPath::sample().into();
        let sut = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            HierarchicalDeterministicPublicKey::new(
                PublicKey::sample_ed25519(),
                derivation_path,
            ),
        );
        assert_eq!(sut.key_kind(), Some(CAP26KeyKind::TransactionSigning));
    }

    #[test]
    fn key_kind_cap26_getid_is_none() {
        let derivation_path: DerivationPath = GetIDPath::default().into();
        let sut = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            HierarchicalDeterministicPublicKey::new(
                PublicKey::sample_ed25519(),
                derivation_path,
            ),
        );
        assert_eq!(sut.key_kind(), None);
    }

    #[test]
    fn sample_auth() {
        assert_eq!(
            HierarchicalDeterministicFactorInstance::sample_auth_signing()
                .derivation_path()
                .to_string(),
            "m/44H/1022H/1H/525H/1678H/0H"
        );
    }
}
