use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UnvalidatedSubintentManifest {
    #[serde(rename = "subintentManifest")]
    pub subintent_manifest_string: String,

    #[serde(default)]
    pub blobs: Blobs,
}

impl UnvalidatedSubintentManifest {
    pub fn new(
        subintent_manifest_string: impl AsRef<str>,
        blobs: impl Into<Blobs>,
    ) -> Self {
        Self {
            subintent_manifest_string: subintent_manifest_string
                .as_ref()
                .to_owned(),
            blobs: blobs.into(),
        }
    }
}

impl From<SubintentManifest> for UnvalidatedSubintentManifest {
    fn from(subintent_manifest: SubintentManifest) -> Self {
        Self {
            subintent_manifest_string: subintent_manifest.manifest_string(),
            blobs: subintent_manifest.blobs().clone(),
        }
    }
}

impl HasSampleValues for UnvalidatedSubintentManifest {
    fn sample() -> Self {
        Self::new(
            SubintentManifest::sample().manifest_string(),
            Blobs::default(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SubintentManifest::sample_other().manifest_string(),
            Blobs::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnvalidatedSubintentManifest;

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
    fn from_subintent_manifest() {
        let subintent_manifest = SubintentManifest::sample();
        let unvalidated_subintent_manifest =
            UnvalidatedSubintentManifest::from(subintent_manifest.clone());
        assert_eq!(
            unvalidated_subintent_manifest.subintent_manifest_string,
            subintent_manifest.manifest_string()
        );
        assert_eq!(
            unvalidated_subintent_manifest.blobs,
            subintent_manifest.blobs().clone()
        );
    }
}
