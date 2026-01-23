use serde::ser::SerializeStruct;

use crate::prelude::*;

/// Either a "physical" badge (resource) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(
    Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq, Hash,
)]
#[serde(untagged, remote = "Self")]
pub enum FactorInstanceBadge {
    Virtual {
        #[serde(rename = "virtualSource")]
        value: FactorInstanceBadgeVirtualSource,
    },
    // Physical {
    //     value: ResourceAddress,
    // },
}

impl HasSampleValues for FactorInstanceBadge {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_virtual()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_virtual_other()
    }
}

impl FactorInstanceBadge {
    /// A sample used to facilitate unit tests.
    fn sample_virtual() -> Self {
        Self::Virtual {
            value: FactorInstanceBadgeVirtualSource::sample(),
        }
    }

    #[allow(unused)]
    /// A sample used to facilitate unit tests.
    fn sample_virtual_other() -> Self {
        Self::Virtual {
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

// impl From<ResourceAddress> for FactorInstanceBadge {
//     fn from(value: ResourceAddress) -> Self {
//         Self::Physical { value }
//     }
// }

impl From<FactorInstanceBadge> for ScryptoResourceOrNonFungible {
    fn from(value: FactorInstanceBadge) -> Self {
        match value {
            FactorInstanceBadge::Virtual {
                value:
                    FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                        value,
                    },
            } => ScryptoResourceOrNonFungible::NonFungible(
                ScryptoNonFungibleGlobalId::from_public_key(
                    ScryptoPublicKey::from(value.public_key),
                ),
            ),
            // FactorInstanceBadge::Physical { value } => {
            //     ScryptoResourceOrNonFungible::Resource(
            //         ScryptoResourceAddress::new_or_panic(value.0.node_id().0),
            //     )
            // }
        }
    }
}

impl<'de> Deserialize<'de> for FactorInstanceBadge {
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("FactorInstanceBadge", 2)?;
        match self {
            Self::Virtual { value } => {
                let discriminant = "virtualSource";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            } // Self::Physical { value } => {
              //     let discriminant = "physical";
              //     state.serialize_field("discriminator", discriminant)?;
              //     state.serialize_field(discriminant, value)?;
              // }
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

    // #[test]
    // fn a_correct_resource_or_non_fungible_is_derived_from_a_physical_factor_instance_badge(
    // ) {
    //     let sut = FactorInstanceBadge::sample_physical();
    //     let resource_or_non_fungible = ScryptoResourceOrNonFungible::from(sut);
    //     assert_eq!(
    //         resource_or_non_fungible,
    //         ScryptoResourceOrNonFungible::Resource(XRD)
    //     );
    // }

    #[test]
    fn a_correct_resource_or_non_fungible_is_derived_from_a_virtual_factor_instance_badge(
    ) {
        let sut = FactorInstanceBadge::sample_virtual();
        let resource_or_non_fungible = ScryptoResourceOrNonFungible::from(sut);
        assert_eq!(
            resource_or_non_fungible,
            ScryptoResourceOrNonFungible::NonFungible(
                ScryptoNonFungibleGlobalId::from_public_key(
                    &ScryptoPublicKey::from(
                        HierarchicalDeterministicPublicKey::sample().public_key
                    )
                )
            )
        );
    }
}
