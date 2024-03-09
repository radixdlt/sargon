use crate::prelude::*;

use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum,
)]
#[serde(tag = "discriminator")]
pub enum ResourceOrNonFungible {
    #[serde(rename = "resourceAddress")]
    Resource { value: ResourceAddress },

    #[serde(rename = "nonFungibleGlobalID")]
    NonFungible { value: NonFungibleGlobalId },
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
    use crate::prelude::*;

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
}
