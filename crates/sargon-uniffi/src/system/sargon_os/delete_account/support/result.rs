use crate::prelude::*;
use sargon::DeleteAccountResult as InternalDeleteAccountResult;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct DeleteAccountResult {
    pub manifest: TransactionManifest,
    pub non_transferable_resources: Vec<ResourceAddress>,
}
