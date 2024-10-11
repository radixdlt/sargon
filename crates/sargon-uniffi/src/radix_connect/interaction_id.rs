use crate::prelude::*;
use sargon::WalletInteractionId as InternalWalletInteractionId;

/// We ought to make `WalletInteractionId` a UUID.
/// Temporarily, it will be a String because the iOS wallet has specific logic that uses custom IDs for wallet interactions.
#[derive(
    
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
     uniffi::Record,
)]
pub struct WalletInteractionId {
    pub id: String,
}

delegate_display_debug_into!(WalletInteractionId, InternalWalletInteractionId);

impl From<InternalWalletInteractionId> for WalletInteractionId {
    fn from(value: InternalWalletInteractionId) -> Self {
        Self {
            id: value.0,
        }
    }
}

impl Into<InternalWalletInteractionId> for WalletInteractionId {
    fn into(self) -> InternalWalletInteractionId {
        InternalWalletInteractionId(self.id)
    }
}