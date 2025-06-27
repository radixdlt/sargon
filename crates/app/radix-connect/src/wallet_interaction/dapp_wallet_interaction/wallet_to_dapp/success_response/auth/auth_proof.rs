use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionAuthProof {
    #[serde_as(as = "DisplayFromStr")]
    pub public_key: PublicKey,
    pub curve: SLIP10Curve,
    #[serde_as(as = "DisplayFromStr")]
    pub signature: Signature,
}

impl WalletToDappInteractionAuthProof {
    pub fn new(
        public_key: impl Into<PublicKey>,
        signature: impl Into<Signature>,
    ) -> Self {
        let public_key = public_key.into();
        let signature = signature.into();
        let curve = public_key.curve();
        assert_eq!(signature.curve(), curve, "Discrepancy between the curve of the public key and the curve of the signature.");
        Self {
            public_key,
            curve,
            signature,
        }
    }
}

impl From<SignatureWithPublicKey> for WalletToDappInteractionAuthProof {
    fn from(signature_with_public_key: SignatureWithPublicKey) -> Self {
        Self::new(
            signature_with_public_key.public_key(),
            signature_with_public_key.signature(),
        )
    }
}

impl HasSampleValues for WalletToDappInteractionAuthProof {
    fn sample() -> Self {
        Self::new(PublicKey::sample(), Signature::sample())
    }

    fn sample_other() -> Self {
        Self::new(PublicKey::sample_other(), Signature::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionAuthProof;

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
    #[should_panic]
    fn panics_if_curve_discrepancy() {
        let _ = SUT::new(PublicKey::sample(), Signature::sample_other());
    }

    #[test]
    fn from_signature_with_public_key() {
        // Ed25519
        let signature_with_public_key = SignatureWithPublicKey::sample();
        let sut = SUT::from(signature_with_public_key);
        assert_eq!(sut.public_key, signature_with_public_key.public_key());
        assert_eq!(sut.signature, signature_with_public_key.signature());

        // Secp256k1
        let signature_with_public_key = SignatureWithPublicKey::sample_other();
        let sut = SUT::from(signature_with_public_key);
        assert_eq!(sut.public_key, signature_with_public_key.public_key());
        assert_eq!(sut.signature, signature_with_public_key.signature());
    }
}
