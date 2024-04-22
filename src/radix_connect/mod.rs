mod interaction_id;
mod interaction_version;
mod wallet_account;
mod wallet_interaction;
mod wallet_persona;

pub use interaction_id::*;
pub use interaction_version::*;
pub use wallet_account::*;
pub use wallet_interaction::*;
pub use wallet_persona::*;

use crate::prelude::*;

#[uniffi::export]
pub fn decode_dapp_to_wallet_interaciton(
    json: BagOfBytes,
) -> Result<DappToWalletInteraction> {
    DappToWalletInteraction::from_json_bag_of_bytes(json.to_vec())
}