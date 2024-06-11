use crate::prelude::*;

/// Vec of Blobs
#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct Blobs {
    pub(crate) secret_magic: BlobsSecretMagic,
}

impl Serialize for Blobs {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.secret_magic.secret_magic.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Blobs {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let option_blobs: Option<Vec<Blob>> =
            Option::deserialize(deserializer)?;
        let blobs = option_blobs.unwrap_or_default();
        let secret_magic: BlobsSecretMagic = blobs.into();
        Ok(secret_magic.into())
    }
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
                (ScryptoHash::from(hash_of(bytes.clone())), bytes)
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
        let roundtrip = |s: SUT| SUT::from(ScryptoBlobs::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn to_from_scrypto_blobs_map() {
        let roundtrip = |s: SUT| SUT::from(ScryptoBlobsMap::from(s));
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

    #[test]
    fn test_roundtrip_non_empty_blobs() {
        let json = r#"
        [
          "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced", 
          "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe",
          "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
          "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        ]
        "#;
        let deserialized_blobs: Blobs = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized_blobs, Blobs::sample());
    }

    #[test]
    fn test_deserialize_null_to_empty_blobs() {
        let json = r#"null"#;
        let deserialized_blobs: Blobs = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized_blobs, Blobs::default());
    }

    #[test]
    #[should_panic]
    fn test_invalid_blobs_does_not_deserialize() {
        let json = r#"[1, 2]"#;
        let _: Blobs = serde_json::from_str(json).unwrap();
    }
}
