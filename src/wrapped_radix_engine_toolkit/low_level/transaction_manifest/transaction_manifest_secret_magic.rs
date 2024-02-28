use crate::prelude::*;

/// Blob is just a bag of bytes (must be, for Kotlin compat)
pub type Blob = BagOfBytes;

/// Vec of Blobs
pub type Blobs = Vec<Blob>;

/// An internal representation of a TransactionManifest,
/// which intentions is to allow the `struct TransactionManifest`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a single field:
/// `private let secretMagic: TransactionManifestSecretMagic`
#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct TransactionManifestSecretMagic {
    pub instructions: Instructions,
    pub blobs: Blobs,
}

impl TransactionManifestSecretMagic {
    pub fn new(instructions: Instructions, blobs: Blobs) -> Self {
        Self {
            instructions,
            blobs,
        }
    }
}

impl HasPlaceholder for TransactionManifestSecretMagic {
    fn placeholder() -> Self {
        Self::new(Instructions::placeholder_simulator(), Vec::new())
    }

    fn placeholder_other() -> Self {
        Self::new(Instructions::placeholder_simulator_other(), Vec::new())
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
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }
}
