use crate::prelude::*;

/// Blob is a wrapper a bag of bytes
#[derive(Clone, PartialEq, Eq, Debug, derive_more::Display, uniffi::Record)]
pub struct Blob {
    pub secret_magic: BagOfBytes,
}

impl Blob {
    pub fn to_hex(&self) -> String {
        self.secret_magic.to_hex()
    }
}

impl From<BagOfBytes> for Blob {
    fn from(value: BagOfBytes) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<Blob> for BagOfBytes {
    fn from(value: Blob) -> BagOfBytes {
        value.secret_magic
    }
}

impl From<ScryptoBlob> for Blob {
    fn from(value: ScryptoBlob) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}

impl From<Blob> for ScryptoBlob {
    fn from(value: Blob) -> Self {
        ScryptoBlob(value.secret_magic.to_vec())
    }
}

impl From<&Vec<u8>> for Blob {
    fn from(value: &Vec<u8>) -> Self {
        Self {
            secret_magic: value.clone().into(),
        }
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

#[uniffi::export]
pub fn new_blob_from_bytes(bytes: BagOfBytes) -> Blob {
    bytes.into()
}

#[uniffi::export]
pub fn blob_to_bytes(blob: &Blob) -> BagOfBytes {
    blob.secret_magic.clone()
}

#[uniffi::export]
pub fn blob_to_string(blob: &Blob) -> String {
    blob.to_string()
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
