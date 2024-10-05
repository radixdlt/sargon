use crate::prelude::*;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
)]
#[serde(tag = "discriminator")]
pub enum ResourceOrNonFungible {
    #[serde(rename = "resourceAddress")]
    Resource { value: ResourceAddress },

    #[serde(rename = "nonFungibleGlobalID")]
    NonFungible { value: NonFungibleGlobalId },
}

impl std::fmt::Display for ResourceOrNonFungible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resource { value } => write!(f, "Resource: {}", value),
            Self::NonFungible { value } => write!(f, "NonFungible: {}", value),
        }
    }
}

impl Identifiable for ResourceOrNonFungible {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl From<(ScryptoResourceOrNonFungible, NetworkID)> for ResourceOrNonFungible {
    fn from(value: (ScryptoResourceOrNonFungible, NetworkID)) -> Self {
        let (resource_or_non_fungible, network_id) = value;
        match resource_or_non_fungible {
            ScryptoResourceOrNonFungible::NonFungible(nf) => {
                Self::NonFungible {
                    value: (nf, network_id).into(),
                }
            }
            ScryptoResourceOrNonFungible::Resource(resource_address) => {
                Self::Resource {
                    value: (resource_address, network_id).into(),
                }
            }
        }
    }
}

impl HasSampleValues for ResourceOrNonFungible {
    fn sample() -> Self {
        Self::Resource {
            value: ResourceAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::NonFungible {
            value: NonFungibleGlobalId::sample(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceOrNonFungible;

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
    fn test_display() {
        assert_eq!(format!("{}", SUT::sample()), "Resource: resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
        assert_eq!(
            format!("{}", SUT::sample_other()),
            "NonFungible: resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>"
        );
    }

    #[test]
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "value" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
              "discriminator" : "resourceAddress"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_sample_other() {
        let sut = SUT::sample_other();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "value" : "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>",
              "discriminator" : "nonFungibleGlobalID"
            }
            "#,
        )
    }

    #[test]
    fn from_scrypto_non_fungible() {
        let global_id = NonFungibleGlobalId::sample();
        let scrypto = ScryptoResourceOrNonFungible::NonFungible(
            ScryptoNonFungibleGlobalId::new(
                global_id.resource_address.into(),
                global_id.non_fungible_local_id.into(),
            ),
        );
        assert_eq!(
            SUT::from((scrypto.clone(), NetworkID::Mainnet)),
            SUT::sample_other()
        );

        // Not equals when wrong network
        assert_ne!(
            SUT::from((scrypto, NetworkID::Stokenet)),
            SUT::sample_other()
        );
    }

    #[test]
    fn from_scrypto_fungible() {
        let resource_address = ResourceAddress::sample_stokenet_gum();
        let scrypto =
            ScryptoResourceOrNonFungible::Resource(resource_address.into());
        let expected = SUT::Resource {
            value: resource_address,
        };
        assert_eq!(
            SUT::from((scrypto.clone(), NetworkID::Stokenet)),
            expected.clone()
        );

        // Not equals, when wrong NetworkID
        assert_ne!(SUT::from((scrypto, NetworkID::Mainnet)), expected);
    }
}
