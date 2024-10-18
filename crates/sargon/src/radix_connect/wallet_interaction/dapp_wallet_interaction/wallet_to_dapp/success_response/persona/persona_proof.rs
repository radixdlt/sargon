use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionPersonaProof {
    pub identity_address: IdentityAddress,
    pub proof: WalletToDappInteractionAuthProof,
}

impl WalletToDappInteractionPersonaProof {
    pub fn new(
        identity_address: impl Into<IdentityAddress>,
        proof: WalletToDappInteractionAuthProof,
    ) -> Self {
        Self {
            identity_address: identity_address.into(),
            proof,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionPersonaProof {
    fn sample() -> Self {
        Self::new(
            IdentityAddress::sample(),
            WalletToDappInteractionAuthProof::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IdentityAddress::sample_other(),
            WalletToDappInteractionAuthProof::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionPersonaProof;

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
