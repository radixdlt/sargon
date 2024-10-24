use crate::prelude::*;
use sargon::DappWalletInteractionPersona as InternalDappWalletInteractionPersona;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappWalletInteractionPersona {
    pub identity_address: IdentityAddress,
    pub label: String,
}
