use crate::prelude::*;

/// A response with the list of proofs of ownership for `Accounts`/`Personas`
/// and the challenge that was signed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionProofOfOwnershipRequestResponseItem {
    pub challenge: DappToWalletInteractionAuthChallengeNonce,

    pub proofs: Vec<WalletToDappInteractionProofOfOwnership>,
}

impl WalletToDappInteractionProofOfOwnershipRequestResponseItem {
    pub fn new(
        challenge: impl Into<DappToWalletInteractionAuthChallengeNonce>,
        proofs: impl Into<Vec<WalletToDappInteractionProofOfOwnership>>,
    ) -> Self {
        Self {
            challenge: challenge.into(),
            proofs: proofs.into(),
        }
    }
}

impl HasSampleValues
    for WalletToDappInteractionProofOfOwnershipRequestResponseItem
{
    fn sample() -> Self {
        Self::new(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            vec![
                WalletToDappInteractionProofOfOwnership::sample(),
                WalletToDappInteractionProofOfOwnership::sample_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
            vec![WalletToDappInteractionProofOfOwnership::sample_other()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionProofOfOwnershipRequestResponseItem;

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
                "proofs": [
                    {
                        "accountAddress": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
                        "proof": {
                            "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                            "curve": "curve25519",
                            "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
                        }
                    },
                    {
                        "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
                        "proof": {
                            "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                            "curve": "curve25519",
                            "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
                        }
                    }
                ]
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
            {
                "challenge": "fadefadefadefadefadefadefadefadefadefadefadefadefadefadefadefade",
                "proofs": [
                    {
                        "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
                        "proof": {
                            "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                            "curve": "curve25519",
                            "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
                        }
                    }
                ]
            }
            "#,
        );
    }
}
