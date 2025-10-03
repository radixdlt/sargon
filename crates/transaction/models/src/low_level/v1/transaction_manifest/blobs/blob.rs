use crate::prelude::*;

/// Blob is a wrapper a bag of bytes
#[derive(
    Clone,
    PartialEq,
    Eq,
    DeserializeFromStr,
    SerializeDisplay,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Blob(pub BagOfBytes);

impl Blob {
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn from_hex(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl From<BagOfBytes> for Blob {
    fn from(value: BagOfBytes) -> Self {
        Self(value)
    }
}

impl From<Blob> for BagOfBytes {
    fn from(value: Blob) -> BagOfBytes {
        value.0
    }
}

impl From<ScryptoBlob> for Blob {
    fn from(value: ScryptoBlob) -> Self {
        Self(value.0.into())
    }
}

impl From<Blob> for ScryptoBlob {
    fn from(value: Blob) -> Self {
        ScryptoBlob(value.0.to_vec())
    }
}

impl From<&Vec<u8>> for Blob {
    fn from(value: &Vec<u8>) -> Self {
        Self(value.clone().into())
    }
}

impl From<Blob> for Vec<u8> {
    fn from(val: Blob) -> Self {
        val.0.to_vec()
    }
}

impl HasSampleValues for Blob {
    fn sample() -> Self {
        BagOfBytes::sample_aced().into()
    }

    fn sample_other() -> Self {
        BagOfBytes::from_hex(&"deadbeefabbafadecafe".repeat(100))
            .unwrap()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Blob;

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
    fn display() {
        assert_eq!(
            SUT::sample().to_string(),
            "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced"
        );
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::from(ScryptoBlob::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn to_from_bag_of_bytes() {
        let roundtrip = |s: SUT| SUT::from(BagOfBytes::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn from_vec() {
        let vec = vec![0xde, 0xad];
        assert_eq!(SUT::from(&vec).to_string(), "dead");
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced"),
        );
    }

    #[test]
    fn test_to_hex() {
        let sample_blob = SUT::sample();
        let hex = sample_blob.to_hex();
        let expected_hex = sample_blob.0.to_hex();
        assert_eq!(hex, expected_hex);
    }

    #[test]
    fn test_from_hex() {
        let hex_str =
            "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced";
        let blob = SUT::from_hex(hex_str).unwrap();
        let expected_blob = SUT::from(BagOfBytes::from_hex(hex_str).unwrap());
        assert_eq!(blob, expected_blob);
    }
}
