use crate::prelude::*;

/// A request to prove ownership of `Accounts` and/or a `Persona`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionProofOfOwnershipRequestItem {
    /// The challenge that must be signed to prove ownership.
    pub challenge: DappToWalletInteractionAuthChallengeNonce,

    /// The list of `AccountAddress`es for which the wallet must prove ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_addresses: Option<Vec<AccountAddress>>,

    /// The `IdentityAddress` for which the wallet must prove ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_address: Option<IdentityAddress>,
}

impl DappToWalletInteractionProofOfOwnershipRequestItem {
    pub fn new(
        challenge: impl Into<DappToWalletInteractionAuthChallengeNonce>,
        account_addresses: impl Into<Option<Vec<AccountAddress>>>,
        identity_address: impl Into<Option<IdentityAddress>>,
    ) -> Self {
        Self {
            challenge: challenge.into(),
            account_addresses: account_addresses.into(),
            identity_address: identity_address.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionProofOfOwnershipRequestItem {
    fn sample() -> Self {
        Self::new(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            vec![AccountAddress::sample(), AccountAddress::sample_other()],
            IdentityAddress::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
            vec![AccountAddress::sample_other()],
            IdentityAddress::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionProofOfOwnershipRequestItem;

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
                "challenge": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead",
                "accountAddresses": [
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
                    "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
                ],
                "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g"
            }
            "#,
        );
    }
}
