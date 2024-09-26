use crate::prelude::*;

/// A request to prove ownership of either a set of `Accounts` or a `Persona`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Enum)]
#[serde(tag = "discriminator")]
#[serde(rename_all = "camelCase")]
pub enum DappToWalletInteractionProofOfOwnershipRequestItem {
    #[serde(rename = "accountsProofOfOwnership")]
    Accounts(DappToWalletInteractionAccountsProofOfOwnershipRequestItem),

    #[serde(rename = "personaProofOfOwnership")]
    Persona(DappToWalletInteractionPersonaProofOfOwnershipRequestItem),
}

impl From<DappToWalletInteractionAccountsProofOfOwnershipRequestItem>
    for DappToWalletInteractionProofOfOwnershipRequestItem
{
    fn from(
        value: DappToWalletInteractionAccountsProofOfOwnershipRequestItem,
    ) -> Self {
        Self::Accounts(value)
    }
}

impl From<DappToWalletInteractionPersonaProofOfOwnershipRequestItem>
    for DappToWalletInteractionProofOfOwnershipRequestItem
{
    fn from(
        value: DappToWalletInteractionPersonaProofOfOwnershipRequestItem,
    ) -> Self {
        Self::Persona(value)
    }
}

impl HasSampleValues for DappToWalletInteractionProofOfOwnershipRequestItem {
    fn sample() -> Self {
        Self::Accounts(DappToWalletInteractionAccountsProofOfOwnershipRequestItem::sample())
    }

    fn sample_other() -> Self {
        Self::Persona(DappToWalletInteractionPersonaProofOfOwnershipRequestItem::sample())
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
    fn from_accounts() {
        assert_eq!(
            SUT::sample(),
            DappToWalletInteractionAccountsProofOfOwnershipRequestItem::sample().into()
        )
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
                "discriminator": "accountsProofOfOwnership",
                "accountAddresses": [
                    "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
                    "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
                ],
                "challenge": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
            {
                "discriminator": "personaProofOfOwnership",
                "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
                "challenge": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
            }
            "#,
        );
    }
}
