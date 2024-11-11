use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountAuthorizedDepositorsResponseItem {
    pub resource_address: ResourceAddress,
    #[serde(flatten)]
    pub badge_type: AccountAuthorizedDepositorBadgeType,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum AccountAuthorizedDepositorBadgeType {
    ResourceBadge,
    NonFungibleBadge { non_fungible_id: String },
}

impl HasSampleValues for AccountAuthorizedDepositorsResponseItem {
    fn sample() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_xrd(),
            badge_type: AccountAuthorizedDepositorBadgeType::ResourceBadge
        }
    }

    fn sample_other() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_nft_abandon(),
            badge_type: AccountAuthorizedDepositorBadgeType::NonFungibleBadge {
                non_fungible_id: "#1#".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAuthorizedDepositorsResponseItem;

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
                "resource_address": "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3",
                "badge_type": "NonFungibleBadge",
                "non_fungible_id": "#1#"
            }
            "##,
        );
    }
}
