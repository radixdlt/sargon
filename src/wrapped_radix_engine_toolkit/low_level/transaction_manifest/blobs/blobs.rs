use crate::prelude::*;

use std::collections::BTreeMap;
use transaction::model::{BlobV1 as ScryptoBlob, BlobsV1 as ScryptoBlobs};

/// Vec of Blobs
#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct Blobs {
    pub(crate) secret_magic: BlobsSecretMagic,
}

impl From<BlobsSecretMagic> for Blobs {
    fn from(value: BlobsSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

#[uniffi::export]
pub fn blobs_list_of_blobs(blobs: &Blobs) -> Vec<Blob> {
    blobs.blobs()
}

impl Blobs {
    pub fn blobs(&self) -> Vec<Blob> {
        self.secret_magic.blobs()
    }

    pub fn new<I>(blobs: I) -> Self
    where
        I: IntoIterator<Item = Blob>,
    {
        BlobsSecretMagic::new(blobs).into()
    }
}

impl Default for Blobs {
    /// Empty blobs
    fn default() -> Self {
        Self {
            secret_magic: BlobsSecretMagic {
                secret_magic: Vec::new(),
            },
        }
    }
}

impl From<ScryptoBlobsMap> for Blobs {
    fn from(value: ScryptoBlobsMap) -> Self {
        Blobs {
            secret_magic: value.into(),
        }
    }
}

impl From<Blobs> for ScryptoBlobsMap {
    fn from(value: Blobs) -> Self {
        value
            .secret_magic
            .clone()
            .secret_magic
            .into_iter()
            .map(|b| {
                let bytes = b.secret_magic.to_vec();
                (
                    radix_engine::types::Hash::from(hash_of(bytes.clone())),
                    bytes,
                )
            })
            .collect()
    }
}

// To From `ScryptoBlobs` (via `BlobsSecretMagic`)
impl From<Blobs> for ScryptoBlobs {
    fn from(value: Blobs) -> Self {
        value.secret_magic.into()
    }
}

impl From<ScryptoBlobs> for Blobs {
    fn from(value: ScryptoBlobs) -> Self {
        Self {
            secret_magic: value.into(),
        }
    }
}

impl HasSampleValues for Blobs {
    fn sample() -> Self {
        BlobsSecretMagic::sample().into()
    }

    fn sample_other() -> Self {
        BlobsSecretMagic::sample_other().into()
    }
}

#[uniffi::export]
pub fn new_blobs_from_blob_list(blobs: Vec<Blob>) -> Blobs {
    Blobs::new(blobs)
}

#[uniffi::export]
pub fn new_blobs_sample() -> Blobs {
    Blobs::sample()
}

#[uniffi::export]
pub fn new_blobs_sample_other() -> Blobs {
    Blobs::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Blobs;

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
    fn blobs() {
        assert_eq!(
            SUT::sample()
                .blobs()
                .into_iter()
                .map(|b| b.secret_magic)
                .collect_vec(),
            [
                BagOfBytes::sample_aced(),
                BagOfBytes::sample_babe(),
                BagOfBytes::sample_cafe(),
                BagOfBytes::sample_dead(),
            ]
        );
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| Into::<SUT>::into(Into::<ScryptoBlobs>::into(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn to_from_scrypto_blobs_map() {
        let roundtrip =
            |s: SUT| Into::<SUT>::into(Into::<ScryptoBlobsMap>::into(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Blobs;

    #[test]
    fn sample() {
        assert_eq!(new_blobs_sample(), SUT::sample());
    }

    #[test]
    fn sample_other() {
        assert_eq!(new_blobs_sample_other(), SUT::sample_other());
    }

    #[test]
    fn test_blobs_list_of_blobs() {
        assert_eq!(blobs_list_of_blobs(&new_blobs_sample()).len(), 4);
    }

    #[test]
    fn test_new_blobs_from_blob_list() {
        assert_eq!(
            new_blobs_from_blob_list(vec![
                Blob::sample(),
                Blob::sample_other(),
            ])
            .blobs(),
            [Blob::sample(), Blob::sample_other(),]
        );
    }
}
