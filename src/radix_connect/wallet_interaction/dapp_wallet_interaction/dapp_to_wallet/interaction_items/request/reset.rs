use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionResetRequestItem {
    pub accounts: bool,
    pub persona_data: bool,
}

impl HasSampleValues for DappToWalletInteractionResetRequestItem {
    fn sample() -> Self {
        Self {
            accounts: true,
            persona_data: true,
        }
    }

    fn sample_other() -> Self {
        Self {
            accounts: false,
            persona_data: false,
        }
    }
}
