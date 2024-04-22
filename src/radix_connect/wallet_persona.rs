use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}

impl HasSampleValues for DappWalletInteractionPersona {
    fn sample() -> Self {
        Self {
            identity_address: IdentityAddress::sample(),
            label: "sample1".to_string(),
        }
    }

    fn sample_other() -> Self {
        Self {
            identity_address: IdentityAddress::sample_other(),
            label: "sample2".to_string(),
        }
    }
}
