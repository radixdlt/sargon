use crate::prelude::*;

/// Represents an ED25519 signature.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Ed25519Signature {
    pub bytes: Exactly64Bytes,
}

impl From<Exactly64Bytes> for Ed25519Signature {
    fn from(value: Exactly64Bytes) -> Self {
        Self { bytes: value }
    }
}

impl TryFrom<BagOfBytes> for Ed25519Signature {
    type Error = CommonError;
    fn try_from(value: BagOfBytes) -> Result<Self> {
        Exactly64Bytes::try_from(value).map(Self::from)
    }
}

impl From<ScryptoEd25519Signature> for Ed25519Signature {
    fn from(value: ScryptoEd25519Signature) -> Self {
        Self::from(Exactly64Bytes::from(&value.0))
    }
}

impl From<Ed25519Signature> for ScryptoEd25519Signature {
    fn from(value: Ed25519Signature) -> Self {
        ScryptoEd25519Signature(*value.bytes.bytes())
    }
}

impl Ed25519Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn from_hex(hex: String) -> Result<Self> {
        Exactly64Bytes::from_str(hex.as_str())
            .map_err(|_| CommonError::InvalidEd25519SignatureFromString {
                bad_value: hex,
            })
            .map(|b| b.into())
    }
}

impl HasSampleValues for Ed25519Signature {
    fn sample() -> Self {
        "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().expect("Should produce a valid sample Ed25519Signature")
    }

    fn sample_other() -> Self {
        "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".parse().expect("Should produce a valid sample Ed25519Signature")
    }
}

#[cfg(test)]
mod tests {
    use crate::HasSampleValues;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Ed25519Signature;

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
    fn scrypto_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(SUT::from(ScryptoEd25519Signature::from(sut)), sut);
    }

    #[test]
    fn scrypto_roundtrip_start_scrypto() {
        let sig: ScryptoEd25519Signature = "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().unwrap();
        assert_eq!(ScryptoEd25519Signature::from(SUT::from(sig)), sig);
    }

    #[test]
    fn to_hex() {
        assert_eq!(SUT::sample_other().to_hex(), "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09");
    }

    #[test]
    fn from_hex() {
        assert_eq!(SUT::from_hex("06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".to_owned()), Ok(SUT::sample_other()));
    }
}
