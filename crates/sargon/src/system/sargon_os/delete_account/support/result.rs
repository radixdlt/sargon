use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeleteAccountResult {
    pub manifest: TransactionManifest,
    pub non_transferable_resources: Vec<ResourceAddress>,
}

impl DeleteAccountResult {
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

impl HasSampleValues for DeleteAccountResult {
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
