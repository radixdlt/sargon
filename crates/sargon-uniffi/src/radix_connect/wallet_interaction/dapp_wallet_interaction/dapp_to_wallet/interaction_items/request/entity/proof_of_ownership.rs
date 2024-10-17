use crate::prelude::*;
use sargon::DappToWalletInteractionProofOfOwnershipRequestItem as InternalDappToWalletInteractionProofOfOwnershipRequestItem;

/// A request to prove ownership of `Accounts` and/or a `Persona`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionProofOfOwnershipRequestItem {
    /// The challenge that must be signed to prove ownership.
    pub challenge: DappToWalletInteractionAuthChallengeNonce,

    /// The list of `AccountAddress`es for which the wallet must prove ownership.
    pub account_addresses: Option<Vec<AccountAddress>>,

    /// The `IdentityAddress` for which the wallet must prove ownership.
    pub identity_address: Option<IdentityAddress>,
}