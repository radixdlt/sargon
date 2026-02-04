use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentRequestItem as InternalDappToWalletInteractionSubintentRequestItem;

decl_version_type!(Subintent);

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSubintentRequestItem {
    pub version: SubintentVersion,

    pub unvalidated_manifest: UnvalidatedSubintentManifest,

    pub message: Option<String>,

    pub expiration: DappToWalletInteractionSubintentExpiration,
    pub header: Option<DappToWalletInteractionSubintentHeader>,
}
