use crate::prelude::*;
use sargon::CreateDeleteAccountManifestOutcome as InternalCreateDeleteAccountManifestOutcome;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct CreateDeleteAccountManifestOutcome {
    pub manifest: TransactionManifest,
    pub non_transferable_resources: Vec<ResourceAddress>,
}
