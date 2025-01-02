use crate::prelude::*;

use radix_engine_interface::blueprints::account::AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "badge_type")]
pub enum AccountAuthorizedDepositor {
    ResourceBadge {
        resource_address: ResourceAddress,
    },
    NonFungibleBadge {
        resource_address: ResourceAddress,
        non_fungible_id: String,
    },
}

impl TryFrom<AccountAuthorizedDepositor>
    for ScryptoAccountRemoveAuthorizedDepositorInput
{
    type Error = CommonError;
    fn try_from(value: AccountAuthorizedDepositor) -> Result<Self> {
        let resource_or_non_fungible = ResourceOrNonFungible::try_from(value)?;
        Ok(resource_or_non_fungible.into())
    }
}

impl TryFrom<AccountAuthorizedDepositor> for ResourceOrNonFungible {
    type Error = CommonError;
    fn try_from(value: AccountAuthorizedDepositor) -> Result<Self> {
        match value {
            AccountAuthorizedDepositor::ResourceBadge { resource_address } => {
                Ok(Self::Resource {
                    value: resource_address,
                })
            }
            AccountAuthorizedDepositor::NonFungibleBadge {
                resource_address,
                non_fungible_id,
            } => {
                if let Ok(non_fungible_id) =
                    NonFungibleLocalId::from_str(&non_fungible_id)
                {
                    Ok(Self::NonFungible {
                        value: NonFungibleGlobalId::new_unchecked(
                            resource_address,
                            non_fungible_id,
                        ),
                    })
                } else {
                    Err(CommonError::InvalidNonFungibleLocalIDString)
                }
            }
        }
    }
}

impl HasSampleValues for AccountAuthorizedDepositor {
    fn sample() -> Self {
        Self::ResourceBadge {
            resource_address: ResourceAddress::sample_stokenet_xrd(),
        }
    }

    fn sample_other() -> Self {
        Self::NonFungibleBadge {
            resource_address: ResourceAddress::sample_stokenet_nft_abandon(),
            non_fungible_id: "#1#".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAuthorizedDepositor;

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
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
                "resource_address": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                "badge_type": "ResourceBadge"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r##"
            {
                "resource_address": "resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x",
                "badge_type": "NonFungibleBadge",
                "non_fungible_id": "#1#"
            }
            "##,
        );
    }
}
