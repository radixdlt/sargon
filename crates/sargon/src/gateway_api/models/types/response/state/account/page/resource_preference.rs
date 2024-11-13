use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountResourcePreference {
    pub resource_address: ResourceAddress,
    #[serde(rename = "resource_preference_rule")]
    pub status: AccountResourcePreferenceRule,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum AccountResourcePreferenceRule {
    Allowed,
    Disallowed,
}

impl HasSampleValues for AccountResourcePreference {
    fn sample() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_xrd(),
            status: AccountResourcePreferenceRule::Allowed,
        }
    }

    fn sample_other() -> Self {
        Self {
            resource_address: ResourceAddress::sample_stokenet_candy(),
            status: AccountResourcePreferenceRule::Disallowed,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountResourcePreference;

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
                "resource_preference_rule": "Allowed"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r##"
            {
                "resource_address": "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3",
                "resource_preference_rule": "Disallowed"
            }
            "##,
        );
    }
}
