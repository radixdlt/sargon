use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAuthorizedRequestItems {
    pub auth: DappToWalletInteractionAuthRequestItem,
    pub reset: Option<DappToWalletInteractionResetRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_accounts: Option<DappToWalletInteractionAccountsRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts: Option<DappToWalletInteractionAccountsRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<DappToWalletInteractionPersonaDataRequestItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_ownership:
        Option<DappToWalletInteractionProofOfOwnershipRequestItem>,
}

impl DappToWalletInteractionAuthorizedRequestItems {
    pub fn new(
        auth: DappToWalletInteractionAuthRequestItem,
        reset: impl Into<Option<DappToWalletInteractionResetRequestItem>>,
        ongoing_accounts: impl Into<
            Option<DappToWalletInteractionAccountsRequestItem>,
        >,
        ongoing_persona_data: impl Into<
            Option<DappToWalletInteractionPersonaDataRequestItem>,
        >,
        one_time_accounts: impl Into<
            Option<DappToWalletInteractionAccountsRequestItem>,
        >,
        one_time_persona_data: impl Into<
            Option<DappToWalletInteractionPersonaDataRequestItem>,
        >,
        proof_of_ownership: impl Into<
            Option<DappToWalletInteractionProofOfOwnershipRequestItem>,
        >,
    ) -> Self {
        Self {
            auth,
            reset: reset.into(),
            ongoing_accounts: ongoing_accounts.into(),
            ongoing_persona_data: ongoing_persona_data.into(),
            one_time_accounts: one_time_accounts.into(),
            one_time_persona_data: one_time_persona_data.into(),
            proof_of_ownership: proof_of_ownership.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionAuthorizedRequestItems {
    fn sample() -> Self {
        Self::new(
            DappToWalletInteractionAuthRequestItem::sample(),
            DappToWalletInteractionResetRequestItem::sample(),
            DappToWalletInteractionAccountsRequestItem::sample(),
            DappToWalletInteractionPersonaDataRequestItem::sample(),
            DappToWalletInteractionAccountsRequestItem::sample(),
            DappToWalletInteractionPersonaDataRequestItem::sample(),
            DappToWalletInteractionProofOfOwnershipRequestItem::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappToWalletInteractionAuthRequestItem::sample_other(),
            DappToWalletInteractionResetRequestItem::sample_other(),
            DappToWalletInteractionAccountsRequestItem::sample_other(),
            DappToWalletInteractionPersonaDataRequestItem::sample_other(),
            DappToWalletInteractionAccountsRequestItem::sample_other(),
            DappToWalletInteractionPersonaDataRequestItem::sample_other(),
            DappToWalletInteractionProofOfOwnershipRequestItem::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAuthorizedRequestItems;

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
