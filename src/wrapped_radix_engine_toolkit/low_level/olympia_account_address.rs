use crate::prelude::*;

dummy_sargon!(OlympiaAccountAddress);


#[uniffi::export]
// FIXME: Rename to `olympia_account_address_from_public_key` after 1st wallet integration
pub fn derive_olympia_mainnet_account_address_from_public_key(
    public_key: Secp256k1PublicKey,
) -> Result<OlympiaAccountAddress> {
    todo!()
}
