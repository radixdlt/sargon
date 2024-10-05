use crate::prelude::*;
use sargon::BlobsSecretMagic as InternalBlobsSecretMagic;

/// Vec of Blobs
#[derive(
    Clone, PartialEq, Eq, Serialize, Deserialize, Debug, uniffi::Record,
)]
#[serde(transparent)]
pub struct BlobsSecretMagic {
    pub(crate) secret_magic: Vec<Blob>,
}

impl From<InternalBlobsSecretMagic> for BlobsSecretMagic {
    fn from(value: InternalBlobsSecretMagic) -> Self {
        Self {
            secret_magic: value.secret_magic.into_iter().map(Blob::from).collect(),
        }
    }
}

impl Into<InternalBlobsSecretMagic> for BlobsSecretMagic {
    fn into(self) -> InternalBlobsSecretMagic {
        InternalBlobsSecretMagic {
            secret_magic: self.secret_magic.into_iter().map(Into::into).collect(),
        }
    }
}