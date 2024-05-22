use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
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
        curve: SLIP10Curve,
        signature: impl Into<Signature>,
    ) -> Self {
        Self {
            public_key: public_key.into(),
            curve,
            signature: signature.into(),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionAuthProof {
    fn sample() -> Self {
        Self::new(
            PublicKey::sample(),
            SLIP10Curve::sample(),
            Signature::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            PublicKey::sample_other(),
            SLIP10Curve::sample_other(),
            Signature::sample_other(),
        )
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
}
