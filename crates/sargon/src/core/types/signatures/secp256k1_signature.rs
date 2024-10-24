use crate::prelude::*;

/// Represents an Secp256k1 signature.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Secp256k1Signature {
    // recovery id + signature
    pub bytes: Exactly65Bytes,
}

impl TryFrom<BagOfBytes> for Secp256k1Signature {
    type Error = CommonError;
    fn try_from(value: BagOfBytes) -> Result<Self> {
        Exactly65Bytes::try_from(value).map(Self::from)
    }
}

impl From<Exactly65Bytes> for Secp256k1Signature {
    fn from(value: Exactly65Bytes) -> Self {
        Self { bytes: value }
    }
}

impl From<ScryptoSecp256k1Signature> for Secp256k1Signature {
    fn from(value: ScryptoSecp256k1Signature) -> Self {
        Self::from(Exactly65Bytes::from(&value.0))
    }
}

impl From<Secp256k1Signature> for ScryptoSecp256k1Signature {
    fn from(value: Secp256k1Signature) -> Self {
        ScryptoSecp256k1Signature(*value.bytes.bytes())
    }
}

impl Secp256k1Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl HasSampleValues for Secp256k1Signature {
    fn sample() -> Self {
        "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12".parse().expect("Should construct valid sample values.")
    }

    fn sample_other() -> Self {
        "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().expect("Should construct valid sample values.")
    }
}

#[cfg(test)]
mod tests {
    use crate::HasSampleValues;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Secp256k1Signature;

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
        assert_eq!(SUT::from(ScryptoSecp256k1Signature::from(sut)), sut);
    }

    #[test]
    fn scrypto_roundtrip_start_scrypto() {
        let sig: ScryptoSecp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();
        assert_eq!(ScryptoSecp256k1Signature::from(SUT::from(sig)), sig);
    }

    #[test]
    fn to_hex() {
        assert_eq!(SUT::sample().to_hex(), "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12");
    }
}
