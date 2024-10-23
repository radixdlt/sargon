use crate::prelude::*;
use sargon::IntentCoreV2 as InternalIntentCoreV2;

/// Represents the core of an intent in V2, including the header, manifest, and message.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct IntentCoreV2 {
    pub header: IntentHeaderV2,
    pub manifest: TransactionManifestV2,
    pub message: MessageV2,
}
