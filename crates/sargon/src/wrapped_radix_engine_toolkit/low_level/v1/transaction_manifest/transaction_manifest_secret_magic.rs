use crate::prelude::*;

/// An internal representation of a TransactionManifest,
/// which intentions is to allow the `struct TransactionManifest`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a single field:
/// `private let secretMagic: TransactionManifestSecretMagic`
#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct TransactionManifestSecretMagic {
    pub instructions: Instructions,
    pub blobs: Blobs,
    pub object_names: ManifestObjectNames,
}

impl TransactionManifestSecretMagic {
    pub fn new(
        instructions: Instructions,
        blobs: impl Into<Blobs>,
        object_names: ManifestObjectNames,
    ) -> Self {
        Self {
            instructions,
            blobs: blobs.into(),
            object_names,
        }
    }

    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstruction> {
        self.instructions.instructions()
    }
}

impl HasSampleValues for TransactionManifestSecretMagic {
    fn sample() -> Self {
        Self::new(
            Instructions::sample_mainnet(),
            Blobs::default(),
            ManifestObjectNames::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            Instructions::sample_simulator_other(),
            Blobs::default(),
            ManifestObjectNames::sample_other(),
        )
    }
}

#[allow(unused)]
impl TransactionManifestSecretMagic {
    pub(crate) fn sample_mainnet_without_lock_fee() -> Self {
        Self::new(
            Instructions::sample_mainnet_without_lock_fee(),
            Blobs::default(),
            ManifestObjectNames::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifestSecretMagic;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
