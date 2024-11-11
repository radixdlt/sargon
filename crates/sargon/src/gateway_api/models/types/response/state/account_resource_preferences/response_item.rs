use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountResourcePreferencesResponseItem {
    pub resource_address: ResourceAddress,
    #[serde(rename = "resource_preference_rule")]
    pub status: AccountResourcePreferenceRule,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum AccountResourcePreferenceRule {
    Allowed,
    Disallowed,
}

impl HasSampleValues for AccountResourcePreferencesResponseItem {
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