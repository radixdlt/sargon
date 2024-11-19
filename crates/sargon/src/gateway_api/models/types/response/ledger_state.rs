use crate::prelude::*;

/// The ledger state against which the response was generated. Can be used to detect if the Network Gateway is returning up-to-date information.
#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
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

impl LedgerState {
    pub fn new(
        network: impl Into<String>,
        state_version: u64,
        proposer_round_timestamp: impl Into<String>,
        epoch: u64,
        round: u64,
    ) -> Self {
        Self {
            network: network.into(),
            state_version,
            proposer_round_timestamp: proposer_round_timestamp.into(),
            epoch,
            round,
        }
    }
}

impl HasSampleValues for LedgerState {
    fn sample() -> Self {
        Self::new("stokenet", 1, "2021-01-01T00:00:00Z", 1, 1)
    }

    fn sample_other() -> Self {
        Self::new("stokenet", 2, "2022-02-02T00:00:00Z", 2, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerState;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
        {
            "network": "stokenet",
            "state_version": 1,
            "proposer_round_timestamp": "2021-01-01T00:00:00Z",
            "epoch": 1,
            "round": 1
        }
        "#,
        );
    }
}
