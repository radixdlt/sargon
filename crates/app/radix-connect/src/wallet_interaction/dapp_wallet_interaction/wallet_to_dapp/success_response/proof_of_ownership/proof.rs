use crate::prelude::*;

/// A proof of ownership of either an `Account` or a `Persona`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

impl From<(AddressOfAccountOrPersona, SignatureWithPublicKey)>
    for WalletToDappInteractionProofOfOwnership
{
    fn from(
        value: (AddressOfAccountOrPersona, SignatureWithPublicKey),
    ) -> Self {
        let (owner, signature_with_public_key) = value;
        match owner {
            AddressOfAccountOrPersona::Account(account_address) => {
                WalletToDappInteractionProofOfOwnership::Account(
                    WalletToDappInteractionAccountProof::new(
                        account_address,
                        signature_with_public_key.into(),
                    ),
                )
            }
            AddressOfAccountOrPersona::Identity(identity_address) => {
                WalletToDappInteractionProofOfOwnership::Persona(
                    WalletToDappInteractionPersonaProof::new(
                        identity_address,
                        signature_with_public_key.into(),
                    ),
                )
            }
        }
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
    fn from_account_address_and_signature() {
        let signature_with_public_key = SignatureWithPublicKey::sample();
        let account_address = AccountAddress::sample();
        let result =
            SUT::from((account_address.into(), signature_with_public_key));
        match result {
            SUT::Account(account_proof) => {
                assert_eq!(account_proof.account_address, account_address);
                assert_eq!(
                    account_proof.proof,
                    signature_with_public_key.into()
                );
            }
            _ => panic!("Expected Account proof"),
        }
    }

    #[test]
    fn from_persona_address_and_signature() {
        let signature_with_public_key = SignatureWithPublicKey::sample();
        let identity_address = IdentityAddress::sample();
        let result =
            SUT::from((identity_address.into(), signature_with_public_key));
        match result {
            SUT::Persona(persona_proof) => {
                assert_eq!(persona_proof.identity_address, identity_address);
                assert_eq!(
                    persona_proof.proof,
                    signature_with_public_key.into()
                );
            }
            _ => panic!("Expected Persona proof"),
        }
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
