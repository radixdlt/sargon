use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentRequestItem as InternalDappToWalletInteractionSubintentRequestItem;

decl_version_type!(Subintent);

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSubintentRequestItem {
    pub version: SubintentVersion,

    pub unvalidated_manifest: UnvalidatedTransactionManifest,

    pub message: Option<String>,

    pub expiration: Option<DappToWalletInteractionSubintentExpiration>,
}
