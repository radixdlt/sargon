use crate::prelude::*;
use sargon::DappToWalletInteractionAuthUsePersonaRequestItem as InternalDappToWalletInteractionAuthUsePersonaRequestItem;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct DappToWalletInteractionAuthUsePersonaRequestItem {
    pub identity_address: IdentityAddress,
}
