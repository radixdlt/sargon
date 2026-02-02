use serde::ser::SerializeStruct;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged, remote = "Self")]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        #[serde(rename = "hierarchicalDeterministicPublicKey")]
        value: HierarchicalDeterministicPublicKey,
    },
}

impl IsNetworkAware for FactorInstanceBadgeVirtualSource {
    fn network_id(&self) -> NetworkID {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                value,
            } => value.derivation_path.network_id(),
        }
    }
}

impl Identifiable for FactorInstanceBadgeVirtualSource {
    type ID = PublicKey;
    fn id(&self) -> Self::ID {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                value,
            } => value.public_key,
        }
    }
}

impl IsKeySpaceAware for FactorInstanceBadgeVirtualSource {
    fn key_space(&self) -> KeySpace {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                value,
            } => value.key_space(),
        }
    }
}

impl FactorInstanceBadgeVirtualSource {
    pub fn as_hierarchical_deterministic(
        &self,
    ) -> &HierarchicalDeterministicPublicKey {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                value,
            } => value,
        }
    }
}

impl From<HierarchicalDeterministicPublicKey>
    for FactorInstanceBadgeVirtualSource
{
    fn from(value: HierarchicalDeterministicPublicKey) -> Self {
        Self::HierarchicalDeterministic { value }
    }
}

impl<'de> Deserialize<'de> for FactorInstanceBadgeVirtualSource {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(flatten, with = "FactorInstanceBadgeVirtualSource")]
            value: FactorInstanceBadgeVirtualSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.value)
    }
}

impl Serialize for FactorInstanceBadgeVirtualSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer
            .serialize_struct("FactorInstanceBadgeVirtualSource", 2)?;
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                value,
            } => {
                let discriminant = "hierarchicalDeterministicPublicKey";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

impl HasSampleValues for FactorInstanceBadgeVirtualSource {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::HierarchicalDeterministic {
            value: HierarchicalDeterministicPublicKey::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::HierarchicalDeterministic {
            value: HierarchicalDeterministicPublicKey::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorInstanceBadgeVirtualSource::sample(),
            FactorInstanceBadgeVirtualSource::sample()
        );
        assert_eq!(
            FactorInstanceBadgeVirtualSource::sample_other(),
            FactorInstanceBadgeVirtualSource::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorInstanceBadgeVirtualSource::sample(),
            FactorInstanceBadgeVirtualSource::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorInstanceBadgeVirtualSource::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
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
			}

            "#,
        );
    }
}
