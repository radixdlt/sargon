use crate::prelude::*;
use sargon::SargonOsSubintentManifestModify;

#[uniffi::export]
impl SargonOS {
    pub async fn modify_subintent_manifest(
        &self,
        subintent_manifest: SubintentManifest,
        guarantees: Vec<TransactionGuarantee>,
    ) -> Result<SubintentManifest> {
        self.wrapped
            .modify_subintent_manifest(
                subintent_manifest.into_internal(),
                guarantees.iter().map(|g| g.into_internal()),
            )
            .into_result()
    }
}
