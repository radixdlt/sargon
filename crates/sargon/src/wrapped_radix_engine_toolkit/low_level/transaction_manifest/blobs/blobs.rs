use crate::prelude::*;

/// Vec of Blobs
#[derive(
    Clone, PartialEq, Eq, Debug, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Blobs(pub Vec<Blob>);

impl Blobs {
    pub fn blobs(&self) -> Vec<Blob> {
        self.0
    }
}

impl Default for Blobs {
    /// Empty blobs
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<ScryptoBlobsMap> for Blobs {
    fn from(value: ScryptoBlobsMap) -> Self {
        Blobs(value.values().map(Blob::from).collect_vec())
    }
}

impl From<Blobs> for ScryptoBlobsMap {
    fn from(value: Blobs) -> Self {
        ScryptoBlobs {
            blobs: value
        .0
            .into_iter()
            .map(|b| {
                let bytes = b.0.to_vec();
                (ScryptoHash::from(hash_of(bytes.clone())), bytes)
            })
            .collect()
        }
    }
}

// To From `ScryptoBlobs` (via `BlobsSecretMagic`)
impl From<Blobs> for ScryptoBlobs {
    fn from(value: Blobs) -> Self {
        value.0.into_iter()
                .map(|b| b.into())
                .collect_vec(),
    }
}

impl From<ScryptoBlobs> for Blobs {
    fn from(value: ScryptoBlobs) -> Self {
        Self(value.into())
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

    // #[test]
    // fn blobs() {
    //     assert_eq!(
    //         SUT::sample()
    //             .blobs()
    //             .into_iter()
    //             .map(|b| b.secret_magic)
    //             .collect_vec(),
    //         [
    //             BagOfBytes::sample_aced(),
    //             BagOfBytes::sample_babe(),
    //             BagOfBytes::sample_cafe(),
    //             BagOfBytes::sample_dead(),
    //         ]
    //     );
    // }

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
        let deserialized_blobs: SUT = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized_blobs, SUT::sample());
    }

    #[test]
    #[should_panic]
    fn test_invalid_blobs_does_not_deserialize() {
        let json = "[1, 2]";
        let _: SUT = serde_json::from_str(json).unwrap();
    }
}
