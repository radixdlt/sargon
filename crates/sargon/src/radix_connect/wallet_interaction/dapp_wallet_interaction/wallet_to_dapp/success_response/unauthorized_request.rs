use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionUnauthorizedRequestResponseItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_accounts:
        Option<WalletToDappInteractionAccountsRequestResponseItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_persona_data:
        Option<WalletToDappInteractionPersonaDataRequestResponseItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_ownership:
        Option<WalletToDappInteractionProofOfOwnershipRequestResponseItem>,
}

impl WalletToDappInteractionUnauthorizedRequestResponseItems {
    pub fn new(
        one_time_accounts: impl Into<
            Option<WalletToDappInteractionAccountsRequestResponseItem>,
        >,
        one_time_persona_data: impl Into<
            Option<WalletToDappInteractionPersonaDataRequestResponseItem>,
        >,
        proof_of_ownership: impl Into<
            Option<WalletToDappInteractionProofOfOwnershipRequestResponseItem>,
        >,
    ) -> Self {
        Self {
            one_time_accounts: one_time_accounts.into(),
            one_time_persona_data: one_time_persona_data.into(),
            proof_of_ownership: proof_of_ownership.into(),
        }
    }
}

impl HasSampleValues
    for WalletToDappInteractionUnauthorizedRequestResponseItems
{
    fn sample() -> Self {
        Self::new(
            WalletToDappInteractionAccountsRequestResponseItem::sample(),
            WalletToDappInteractionPersonaDataRequestResponseItem::sample(),
            WalletToDappInteractionProofOfOwnershipRequestResponseItem::sample(
            ),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            WalletToDappInteractionAccountsRequestResponseItem::sample_other(),
            WalletToDappInteractionPersonaDataRequestResponseItem::sample_other(
            ),
            WalletToDappInteractionProofOfOwnershipRequestResponseItem::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionUnauthorizedRequestResponseItems;

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
