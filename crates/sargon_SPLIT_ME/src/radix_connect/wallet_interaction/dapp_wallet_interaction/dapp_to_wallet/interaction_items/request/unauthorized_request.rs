use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionUnauthorizedRequestItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,
}

impl DappToWalletInteractionUnauthorizedRequestItems {
    pub fn new(
        one_time_accounts: impl Into<
            Option<DappToWalletInteractionAccountsRequestItem>,
        >,
        one_time_persona_data: impl Into<
            Option<DappToWalletInteractionPersonaDataRequestItem>,
        >,
    ) -> Self {
        Self {
            one_time_accounts: one_time_accounts.into(),
            one_time_persona_data: one_time_persona_data.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionUnauthorizedRequestItems {
    fn sample() -> Self {
        Self::new(
            DappToWalletInteractionAccountsRequestItem::sample(),
            DappToWalletInteractionPersonaDataRequestItem::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappToWalletInteractionAccountsRequestItem::sample_other(),
            DappToWalletInteractionPersonaDataRequestItem::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionUnauthorizedRequestItems;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
