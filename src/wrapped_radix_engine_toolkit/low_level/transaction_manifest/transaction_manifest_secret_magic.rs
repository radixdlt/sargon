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
