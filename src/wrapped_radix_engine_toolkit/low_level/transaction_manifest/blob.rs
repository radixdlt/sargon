use crate::prelude::*;

use transaction::model::BlobV1 as ScryptoBlob;

/// Blob is a wrapper a bag of bytes
#[derive(Clone, PartialEq, Eq, Debug, derive_more::Display, uniffi::Record)]
pub struct Blob {
    pub(crate) secret_magic: BagOfBytes,
}

impl From<BagOfBytes> for Blob {
    fn from(value: BagOfBytes) -> Self {
        Self {
            secret_magic: value,
        }
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
pub fn blob_to_string(blob: &Blob) -> String {
    blob.to_string()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Blob;

    #[test]
    fn name() {
        assert_eq!(
            blob_to_string(&SUT::sample()),
            "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced"
        );
    }
}
