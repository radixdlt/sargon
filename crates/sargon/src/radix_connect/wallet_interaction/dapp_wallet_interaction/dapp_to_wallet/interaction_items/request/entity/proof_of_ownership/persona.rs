use crate::prelude::*;

/// A request to prove ownership of a given list of `Persona`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionPersonaProofOfOwnershipRequestItem {
    /// The `IdentityAddress` for which the wallet must prove ownership.
    pub identity_address: IdentityAddress,

    /// The challenge that must be signed to prove ownership.
    pub challenge: DappToWalletInteractionAuthChallengeNonce,
}

impl DappToWalletInteractionPersonaProofOfOwnershipRequestItem {
    pub fn new(
        identity_address: IdentityAddress,
        challenge: impl Into<DappToWalletInteractionAuthChallengeNonce>,
    ) -> Self {
        Self {
            identity_address,
            challenge: challenge.into(),
        }
    }
}

impl HasSampleValues
    for DappToWalletInteractionPersonaProofOfOwnershipRequestItem
{
    fn sample() -> Self {
        Self::new(
            IdentityAddress::sample(),
            DappToWalletInteractionAuthChallengeNonce::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IdentityAddress::sample_other(),
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionPersonaProofOfOwnershipRequestItem;

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
            "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
            "challenge": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        }
        "#,
        );
    }
}
