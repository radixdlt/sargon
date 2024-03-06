use crate::prelude::*;

use radix_engine::types::ManifestBucket as ScryptoManifestBucket;
use transaction::builder::{
    ExistingManifestBucket as ScryptoExistingManifestBucket,
    ManifestNameRegistrar as ScryptoManifestNameRegistrar,
    NewManifestBucket as ScryptoNewManifestBucket,
};

#[derive(Clone)]
pub(crate) struct Bucket {
    pub(crate) name: String,
}

impl AsRef<str> for Bucket {
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}
impl ScryptoNewManifestBucket for &Bucket {
    fn register(self, registrar: &ScryptoManifestNameRegistrar) {
        registrar.register_bucket(registrar.new_bucket(self.name.clone()));
    }
}

impl ScryptoExistingManifestBucket for &Bucket {
    fn resolve(
        self,
        registrar: &ScryptoManifestNameRegistrar,
    ) -> ScryptoManifestBucket {
        registrar.name_lookup().bucket(self)
    }
}
