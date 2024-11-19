use crate::prelude::*;
use sargon::CreateDeleteAccountManifestResult as InternalCreateDeleteAccountManifestResult;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct CreateDeleteAccountManifestResult {
    pub manifest: TransactionManifest,
    pub non_transferable_resources: Vec<ResourceAddress>,
}
