use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionAuthUsePersonaRequestItem {
    pub identity_address: IdentityAddress,
}

impl HasSampleValues for DappToWalletInteractionAuthUsePersonaRequestItem {
    fn sample() -> Self {
        Self {
            identity_address: IdentityAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            identity_address: IdentityAddress::sample_other(),
        }
    }
}
