use crate::HierarchicalDeterministicPublicKey;

use crate::HasPlaceholder;

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[serde(untagged, remote = "Self")]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        #[serde(rename = "hierarchicalDeterministicPublicKey")]
        value: HierarchicalDeterministicPublicKey,
    },
}

impl FactorInstanceBadgeVirtualSource {
    pub fn as_hierarchical_deterministic(&self) -> &HierarchicalDeterministicPublicKey {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic { value } => value,
        }
    }
}

impl From<HierarchicalDeterministicPublicKey> for FactorInstanceBadgeVirtualSource {
    fn from(value: HierarchicalDeterministicPublicKey) -> Self {
        Self::HierarchicalDeterministic { value }
    }
}

impl<'de> Deserialize<'de> for FactorInstanceBadgeVirtualSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorInstanceBadgeVirtualSource")]
            value: FactorInstanceBadgeVirtualSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.value)
    }
}

impl Serialize for FactorInstanceBadgeVirtualSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorInstanceBadgeVirtualSource", 2)?;
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic { value } => {
                let discriminant = "hierarchicalDeterministicPublicKey";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

impl HasPlaceholder for FactorInstanceBadgeVirtualSource {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::HierarchicalDeterministic {
            value: HierarchicalDeterministicPublicKey::placeholder(),
        }
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::HierarchicalDeterministic {
            value: HierarchicalDeterministicPublicKey::placeholder_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::FactorInstanceBadgeVirtualSource;

    #[test]
    fn equality() {
        assert_eq!(
            FactorInstanceBadgeVirtualSource::placeholder(),
            FactorInstanceBadgeVirtualSource::placeholder()
        );
        assert_eq!(
            FactorInstanceBadgeVirtualSource::placeholder_other(),
            FactorInstanceBadgeVirtualSource::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorInstanceBadgeVirtualSource::placeholder(),
            FactorInstanceBadgeVirtualSource::placeholder_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorInstanceBadgeVirtualSource::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
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
			}

            "#,
        );
    }
}
