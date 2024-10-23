use crate::prelude::*;
use sargon::IntentHeaderV2 as InternalIntentHeaderV2;

/// Represents the header of an intent in V2, containing network ID,
/// epoch range, optional proposer timestamps, and an intent discriminator.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct IntentHeaderV2 {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub min_proposer_timestamp_inclusive: Option<Instant>,
    pub max_proposer_timestamp_exclusive: Option<Instant>,

    /// This field is intended to enable a network user to generate an identical intent with
    /// a new hash. Users can simply set this randomly if they wish to. A u64 is large
    /// enough to avoid any risk of collision over the course of a single epoch anyway.
    ///
    /// This field's name intent_discriminator is the new name for what was the nonce field in
    /// IntentV1. This was poorly named, as it caused confusion with an Ethereum-style nonce.
    pub intent_discriminator: IntentDiscriminator,
}

#[uniffi::export]
pub fn new_intent_header_v2_sample() -> IntentHeaderV2 {
    InternalIntentHeaderV2::sample().into()
}

#[uniffi::export]
pub fn new_intent_header_v2_sample_other() -> IntentHeaderV2 {
    InternalIntentHeaderV2::sample_other().into()
}
