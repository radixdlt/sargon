use crate::prelude::*;

/// A proof of ownership of either an `Account` or a `Persona`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Enum)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum WalletToDappInteractionProofOfOwnership {
    Account(WalletToDappInteractionAccountProof),

    Persona(WalletToDappInteractionPersonaProof),
}

impl From<WalletToDappInteractionAccountProof>
    for WalletToDappInteractionProofOfOwnership
{
    fn from(value: WalletToDappInteractionAccountProof) -> Self {
        Self::Account(value)
    }
}

impl From<WalletToDappInteractionPersonaProof>
    for WalletToDappInteractionProofOfOwnership
{
    fn from(value: WalletToDappInteractionPersonaProof) -> Self {
        Self::Persona(value)
    }
}

impl HasSampleValues for WalletToDappInteractionProofOfOwnership {
    fn sample() -> Self {
        Self::Account(WalletToDappInteractionAccountProof::sample())
    }

    fn sample_other() -> Self {
        Self::Persona(WalletToDappInteractionPersonaProof::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionProofOfOwnership;

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
    fn from_account() {
        assert_eq!(
            SUT::sample(),
            WalletToDappInteractionAccountProof::sample().into()
        )
    }

    #[test]
    fn from_persona() {
        assert_eq!(
            SUT::sample_other(),
            WalletToDappInteractionPersonaProof::sample().into()
        )
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
                "accountAddress": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
                "proof": {
                    "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                    "curve": "curve25519",
                    "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
                }
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
            {
                "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
                "proof": {
                    "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                    "curve": "curve25519",
                    "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
                }
            }
            "#,
        );
    }
}
