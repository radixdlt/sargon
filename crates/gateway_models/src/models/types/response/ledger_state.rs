use crate::prelude::*;

/// The ledger state against which the response was generated. Can be used to detect if the Network Gateway is returning up-to-date information.
#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    uniffi::Record,
)]
pub struct LedgerState {
    /// The logical name of the network
    pub network: String,

    /// The state version of the ledger. Each transaction increments the state version by 1.
    pub state_version: u64,

    /// The proposer round timestamp of the consensus round when this transaction was committed to ledger. This is not guaranteed to be strictly increasing, as it is computed as an average across the validator set. If this is significantly behind the current timestamp, the Network Gateway is likely reporting out-dated information, or the network has stalled.  
    pub proposer_round_timestamp: String,

    /// The epoch number of the ledger at this state version.
    pub epoch: u64,

    /// The consensus round in the epoch that this state version was committed in.
    pub round: u64,
}
