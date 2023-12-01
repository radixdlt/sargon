use hierarchical_deterministic::{
    cap26::{
        cap26_key_kind::CAP26KeyKind,
        cap26_path::{cap26_path::CAP26Path, paths::account_path::AccountPath},
        cap26_repr::CAP26Repr,
    },
    derivation::{
        derivation::Derivation, derivation_path::DerivationPath,
        hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey,
        mnemonic_with_passphrase::MnemonicWithPassphrase,
    },
};
use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::{error::Error, network_id::NetworkID, types::keys::public_key::PublicKey};

use crate::v100::factors::factor_source_kind::FactorSourceKind;

use super::{
    factor_instance::{
        badge_virtual_source::FactorInstanceBadgeVirtualSource, factor_instance::FactorInstance,
        factor_instance_badge::FactorInstanceBadge,
    },
    factor_source_id::FactorSourceID,
    factor_source_id_from_hash::FactorSourceIDFromHash,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: PublicKey,
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicFactorInstance {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Self {
        Self {
            factor_source_id,
            public_key,
            derivation_path,
        }
    }

    pub fn try_from(
        factor_source_id: FactorSourceID,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Result<Self, Error> {
        let factor_source_id = factor_source_id
            .as_hash()
            .ok_or(Error::FactorSourceIDNotFromHash)?;
        Ok(Self::new(
            factor_source_id.clone(),
            public_key,
            derivation_path,
        ))
    }

    pub fn try_from_factor_instance(factor_instance: FactorInstance) -> Result<Self, Error> {
        let virtual_source = factor_instance
            .badge
            .as_virtual()
            .ok_or(Error::BadgeIsNotVirtualHierarchicalDeterministic)?;
        let badge = virtual_source
            .as_hierarchical_deterministic()
            .ok_or(Error::BadgeIsNotVirtualHierarchicalDeterministic)?;

        Self::try_from(
            factor_instance.factor_source_id,
            badge.public_key,
            badge.derivation_path.clone(),
        )
    }

    pub fn factor_instance(&self) -> FactorInstance {
        FactorInstance::new(
            self.factor_source_id.clone().into(),
            FactorInstanceBadge::Virtual(
                FactorInstanceBadgeVirtualSource::HierarchicalDeterministic(
                    HierarchicalDeterministicPublicKey::new(
                        self.public_key,
                        self.derivation_path.clone(),
                    ),
                ),
            ),
        )
    }
}

impl Serialize for HierarchicalDeterministicFactorInstance {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.factor_instance().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for HierarchicalDeterministicFactorInstance {
    fn deserialize<D: Deserializer<'de>>(
        d: D,
    ) -> Result<HierarchicalDeterministicFactorInstance, D::Error> {
        FactorInstance::deserialize(d).and_then(|fi| {
            HierarchicalDeterministicFactorInstance::try_from_factor_instance(fi)
                .map_err(de::Error::custom)
        })
    }
}

impl HierarchicalDeterministicFactorInstance {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let path = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        let private_key = mwp.derive_private_key(path.clone());

        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H");

        assert_eq!(
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            private_key.to_hex()
        );
        let public_key = private_key.public_key();
        assert_eq!(
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            public_key.to_hex()
        );
        let id =
            FactorSourceIDFromHash::from_mnemonic_with_passphrase(FactorSourceKind::Device, mwp);
        assert_eq!(
            id.to_string(),
            "device:3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
        );
        Self::new(
            id,
            public_key,
            DerivationPath::CAP26(CAP26Path::AccountPath(path)),
        )
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::HierarchicalDeterministicFactorInstance;

    #[test]
    fn json_roundtrip() {
        let model = HierarchicalDeterministicFactorInstance::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"badge": {
					"virtualSource": {
						"hierarchicalDeterministicPublicKey": {
							"publicKey": {
								"curve": "curve25519",
								"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
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
						"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
					},
					"discriminator": "fromHash"
				}
			}
            "#,
        );
    }
}
