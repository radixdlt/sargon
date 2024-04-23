use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: WalletToDappInteractionAuthProof,
}

impl WalletToDappInteractionAccountProof {
    pub fn new(
        account_address: impl Into<AccountAddress>,
        proof: WalletToDappInteractionAuthProof,
    ) -> Self {
        Self {
            account_address: account_address.into(),
            proof,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionAccountProof {
    fn sample() -> Self {
        Self::new(AccountAddress::sample(), WalletToDappInteractionAuthProof::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_other(),
            WalletToDappInteractionAuthProof::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAccountProof;

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
