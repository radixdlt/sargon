use crate::prelude::*;
use sargon::WalletInteractionId as InternalWalletInteractionId;

uniffi::custom_newtype!(WalletInteractionId, String);

/// We ought to make `WalletInteractionId` a UUID.
/// Temporarily, it will be a String because the iOS wallet has specific logic that uses custom IDs for wallet interactions.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2)]
pub struct WalletInteractionId(pub String);

delegate_display_debug_into!(WalletInteractionId, InternalWalletInteractionId);
