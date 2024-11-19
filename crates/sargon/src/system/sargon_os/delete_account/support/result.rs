use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateDeleteAccountManifestResult {
    pub manifest: TransactionManifest,
    pub non_transferable_resources: Vec<ResourceAddress>,
}

impl CreateDeleteAccountManifestResult {
    pub fn new(
        manifest: TransactionManifest,
        non_transferable_resources: Vec<ResourceAddress>,
    ) -> Self {
        Self {
            manifest,
            non_transferable_resources,
        }
    }
}

impl HasSampleValues for CreateDeleteAccountManifestResult {
    fn sample() -> Self {
        Self::new(TransactionManifest::sample(), vec![])
    }

    fn sample_other() -> Self {
        Self::new(
            TransactionManifest::sample_other(),
            vec![ResourceAddress::sample_other()],
        )
    }
}
