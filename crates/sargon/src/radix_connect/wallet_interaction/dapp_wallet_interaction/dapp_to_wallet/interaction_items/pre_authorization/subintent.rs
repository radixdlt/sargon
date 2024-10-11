use crate::prelude::*;

decl_version_type!(Subintent);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionSubintentRequestItem {
    pub version: SubintentVersion,

    #[serde(flatten, with = "UnvalidatedTransactionManifest")]
    pub unvalidated_manifest: UnvalidatedTransactionManifest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_subintent_hashes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<DappToWalletInteractionSubintentExpiration>,
}
