use crate::prelude::*;

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
