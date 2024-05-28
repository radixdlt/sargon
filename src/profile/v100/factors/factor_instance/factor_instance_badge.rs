use crate::prelude::*;

/// Either a "physical" badge (NFT) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(
    Serialize,
    Deserialize,
    EnumAsInner,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
#[serde(untagged, remote = "Self")]
pub enum FactorInstanceBadge {
    Virtual {
        #[serde(rename = "virtualSource")]
        value: FactorInstanceBadgeVirtualSource,
    },
}

impl HasSampleValues for FactorInstanceBadge {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        FactorInstanceBadge::Virtual {
            value: FactorInstanceBadgeVirtualSource::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        FactorInstanceBadge::Virtual {
            value: FactorInstanceBadgeVirtualSource::sample_other(),
        }
    }
}

impl From<FactorInstanceBadgeVirtualSource> for FactorInstanceBadge {
    fn from(value: FactorInstanceBadgeVirtualSource) -> Self {
        Self::Virtual { value }
    }
}

impl From<HierarchicalDeterministicPublicKey> for FactorInstanceBadge {
    fn from(value: HierarchicalDeterministicPublicKey) -> Self {
        Self::Virtual {
            value: value.into(),
        }
    }
}

impl<'de> Deserialize<'de> for FactorInstanceBadge {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(flatten, with = "FactorInstanceBadge")]
            value: FactorInstanceBadge,
        }
        Wrapper::deserialize(deserializer).map(|w| w.value)
    }
}

impl Serialize for FactorInstanceBadge {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("FactorInstanceBadge", 2)?;
        match self {
            FactorInstanceBadge::Virtual { value } => {
                let discriminant = "virtualSource";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorInstanceBadge::sample(),
            FactorInstanceBadge::sample()
        );
        assert_eq!(
            FactorInstanceBadge::sample_other(),
            FactorInstanceBadge::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorInstanceBadge::sample(),
            FactorInstanceBadge::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorInstanceBadge::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
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
			}
            "#,
        );
    }

    #[test]
    fn into_from_hd_pubkey() {
        let sut: FactorInstanceBadge =
            HierarchicalDeterministicPublicKey::sample().into();
        assert_eq!(
            sut,
            FactorInstanceBadge::Virtual {
                value: FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                    value: HierarchicalDeterministicPublicKey::sample()
                }
            }
        )
    }

    #[test]
    fn into_from_virtual_source() {
        let sut: FactorInstanceBadge =
            FactorInstanceBadgeVirtualSource::sample().into();
        assert_eq!(
            sut,
            FactorInstanceBadge::Virtual {
                value: FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                    value: HierarchicalDeterministicPublicKey::sample()
                }
            }
        )
    }
}
