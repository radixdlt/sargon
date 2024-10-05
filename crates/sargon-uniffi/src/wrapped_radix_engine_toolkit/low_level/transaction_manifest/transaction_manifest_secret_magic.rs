use crate::prelude::*;
use sargon::TransactionManifestSecretMagic as InternalTransactionManifestSecretMagic;

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

impl From<InternalTransactionManifestSecretMagic> for TransactionManifestSecretMagic {
    fn from(value: InternalTransactionManifestSecretMagic) -> Self {
        Self {
            instructions: value.instructions.into(),
            blobs: value.blobs.into(),
        }
    }
}

impl Into<InternalTransactionManifestSecretMagic> for TransactionManifestSecretMagic {
    fn into(self) -> InternalTransactionManifestSecretMagic {
        InternalTransactionManifestSecretMagic {
            instructions: self.instructions.into(),
            blobs: self.blobs.into(),
        }
    }
}