use crate::prelude::*;

/// An internal representation of a TransactionManifestV2,
/// which intentions is to allow the `struct TransactionManifestV2`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a single field:
/// `private let secretMagic: TransactionManifestSecretMagicV2`
#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct TransactionManifestSecretMagicV2 {
    pub instructions: InstructionsV2,
    pub blobs: Blobs,
    pub children: ChildIntents,
    pub object_names: ManifestObjectNames,
}

impl TransactionManifestSecretMagicV2 {
    pub fn new(
        instructions: InstructionsV2,
        blobs: impl Into<Blobs>,
        children: ChildIntents,
        object_names: ManifestObjectNames,
    ) -> Self {
        Self {
            instructions,
            blobs: blobs.into(),
            children,
            object_names,
        }
    }

    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        self.instructions.instructions()
    }
}

impl HasSampleValues for TransactionManifestSecretMagicV2 {
    fn sample() -> Self {
        Self::new(
            InstructionsV2::sample(),
            Blobs::default(),
            ChildIntents::sample(),
            ManifestObjectNames::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            InstructionsV2::sample_other(),
            Blobs::default(),
            ChildIntents::sample_other(),
            ManifestObjectNames::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifestSecretMagicV2;

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
