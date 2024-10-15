use crate::prelude::*;
use sargon::DappToWalletInteractionProofOfOwnershipRequestItem as InternalDappToWalletInteractionProofOfOwnershipRequestItem;

/// A request to prove ownership of `Accounts` and/or a `Persona`.
#[derive(Clone, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionProofOfOwnershipRequestItem {
    /// The challenge that must be signed to prove ownership.
    pub challenge: DappToWalletInteractionAuthChallengeNonce,

    /// The list of `AccountAddress`es for which the wallet must prove ownership.
    pub account_addresses: Option<Vec<AccountAddress>>,

    /// The `IdentityAddress` for which the wallet must prove ownership.
    pub identity_address: Option<IdentityAddress>,
}

impl DappToWalletInteractionProofOfOwnershipRequestItem {
    pub fn into_internal(&self) -> InternalDappToWalletInteractionProofOfOwnershipRequestItem {
        self.clone().into()
    }
}

impl From<InternalDappToWalletInteractionProofOfOwnershipRequestItem> for DappToWalletInteractionProofOfOwnershipRequestItem {
    fn from(internal: InternalDappToWalletInteractionProofOfOwnershipRequestItem) -> Self {
        Self {
            challenge: internal.challenge.into(),
            account_addresses: internal.account_addresses.map(|addresses| addresses.into_vec()),
            identity_address: internal.identity_address.map(|address| address.into()),
        }
    }
}

impl Into<InternalDappToWalletInteractionProofOfOwnershipRequestItem> for DappToWalletInteractionProofOfOwnershipRequestItem {
    fn into(self) -> InternalDappToWalletInteractionProofOfOwnershipRequestItem {
        InternalDappToWalletInteractionProofOfOwnershipRequestItem {
            challenge: self.challenge.into(),
            account_addresses: self.account_addresses.map(|addresses| addresses.into_internal_vec()),
            identity_address: self.identity_address.map(|address| address.into()),
        }
    }
}