// only used here...
use radix_rust::{LabelResolver, LabelledResolvable, LabelledResolveFrom};

use crate::prelude::*;

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

impl LabelledResolvable for &Bucket {
    type ResolverOutput = ScryptoManifestBucket;
}

impl LabelledResolveFrom<&Bucket> for ScryptoManifestBucket {
    fn labelled_resolve_from(
        value: &Bucket,
        resolver: &impl LabelResolver<Self::ResolverOutput>,
    ) -> Self {
        resolver.resolve_label_into(&value.name)
    }
}
