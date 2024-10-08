use crate::prelude::*;
use sargon::Blob as InternalBlob;
use sargon::BagOfBytes as InternalBagOfBytes;

/// Blob is a wrapper a bag of bytes
#[derive(
    Clone,
    PartialEq,
    Eq,
    uniffi::Record,
    
    
)]
pub struct Blob {
    pub(crate) secret_magic: BagOfBytes,
}

impl From<InternalBlob> for Blob {
    fn from(value: InternalBlob) -> Self {
        Self {
            secret_magic: value.secret_magic.into(),
        }
    }
}

impl Into<InternalBlob> for Blob {
    fn into(self) -> InternalBlob {
        InternalBlob(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn new_blob_from_bytes(bytes: BagOfBytes) -> Blob {
    InternalBlob::from(bytes.into_internal()).into()
}

#[uniffi::export]
pub fn blob_to_bytes(blob: &Blob) -> BagOfBytes {
    blob.secret_magic.clone()
}

#[uniffi::export]
pub fn blob_to_string(blob: &Blob) -> String {
    blob.into_internal().to_string()
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

    // #[test]
    // fn json_roundtrip() {
    //     let model = SUT::sample();
    //     assert_json_value_eq_after_roundtrip(
    //         &model,
    //         json!("acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced"),
    //     );
    // }

    #[test]
    fn test_to_hex() {
        let sample_blob = SUT::sample();
        let hex = sample_blob.to_hex();
        let expected_hex = sample_blob.secret_magic.to_hex();
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

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Blob;

    #[test]
    fn test_blob_to_string() {
        assert_eq!(
            blob_to_string(&SUT::sample()),
            "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced"
        );
    }

    #[test]
    fn test_new_blob_from_bytes() {
        let bytes = BagOfBytes::from_hex("dead").unwrap();
        assert_eq!(new_blob_from_bytes(bytes.clone()).secret_magic, bytes);
    }

    #[test]
    fn test_blob_to_bytes() {
        assert_eq!(blob_to_bytes(&SUT::sample()), BagOfBytes::sample_aced());
    }
}
