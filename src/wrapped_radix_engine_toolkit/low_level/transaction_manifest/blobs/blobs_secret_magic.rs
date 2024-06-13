use crate::prelude::*;

/// Vec of Blobs
#[derive(
    Clone, PartialEq, Eq, Serialize, Deserialize, Debug, uniffi::Record,
)]
// #[serde_as]
// #[serde(transparent)]
pub struct BlobsSecretMagic {
    // #[serde_as(deserialize_as = "DefaultOnNull")]
    pub(crate) secret_magic: Vec<Blob>,
}

// impl Default for BlobsSecretMagic {
//     /// Empty blobs
//     fn default() -> Self {
//         Self {
//             secret_magic: Vec::new()
//         }
//     }
// }

impl BlobsSecretMagic {
    pub fn blobs(&self) -> Vec<Blob> {
        self.secret_magic.clone()
    }

    pub fn new<I>(blobs: I) -> Self
    where
        I: IntoIterator<Item = Blob>,
    {
        Self {
            secret_magic: blobs.into_iter().collect_vec(),
        }
    }

    pub(crate) fn from_bags<I>(bags: I) -> Self
    where
        I: IntoIterator<Item = BagOfBytes>,
    {
        Self::new(bags.into_iter().map(Blob::from))
    }
}

impl From<Vec<Blob>> for BlobsSecretMagic {
    fn from(value: Vec<Blob>) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<ScryptoBlobs> for BlobsSecretMagic {
    fn from(value: ScryptoBlobs) -> Self {
        Self::from(value.blobs.into_iter().map(|b| b.into()).collect_vec())
    }
}

pub(crate) type ScryptoBlobsMap = IndexMap<ScryptoHash, Vec<u8>>;

impl From<ScryptoBlobsMap> for BlobsSecretMagic {
    fn from(value: ScryptoBlobsMap) -> Self {
        Self::from(value.values().map(Blob::from).collect_vec())
    }
}

impl From<BlobsSecretMagic> for ScryptoBlobs {
    fn from(value: BlobsSecretMagic) -> Self {
        ScryptoBlobs {
            blobs: value
                .secret_magic
                .clone()
                .into_iter()
                .map(|b| b.into())
                .collect_vec(),
        }
    }
}

impl HasSampleValues for BlobsSecretMagic {
    fn sample() -> Self {
        Self::from_bags([
            BagOfBytes::sample_aced(),
            BagOfBytes::sample_babe(),
            BagOfBytes::sample_cafe(),
            BagOfBytes::sample_dead(),
        ])
    }

    fn sample_other() -> Self {
        Self::new([Blob::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BlobsSecretMagic;

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
        let roundtrip = |s: SUT| SUT::from(ScryptoBlobs::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
