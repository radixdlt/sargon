use crate::prelude::*;

/// A request to prove ownership of a given list of `Accounts`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAccountsProof {
    /// The list of `AccountAddress`es for which the wallet must prove ownership.
    pub account_addresses: Vec<AccountAddress>,

    /// The challenge that must be signed to prove ownership.
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
}

impl DappToWalletInteractionAccountsProof {
    pub fn new(
        account_addresses: Vec<AccountAddress>,
        challenge: impl Into<DappToWalletInteractionAuthChallengeNonce>,
    ) -> Self {
        Self {
            account_addresses,
            challenge: challenge.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionAccountsProof {
    fn sample() -> Self {
        Self::new(
            vec![AccountAddress::sample(), AccountAddress::sample_other()],
            DappToWalletInteractionAuthChallengeNonce::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            vec![AccountAddress::sample_other()],
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionAccountsProof;

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
            "accountAddresses": [
                "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
                "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
            ],
            "challenge": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        }
        "#,
        );
    }
}
