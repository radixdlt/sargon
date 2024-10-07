use crate::prelude::*;

#[cfg(test)]
impl LedgerState {
    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        Self {
            network: NetworkID::Stokenet.logical_name(),
            state_version: 80577579,
            proposer_round_timestamp: "2024-10-07T15:41:07.259Z".to_string(),
            epoch: 41965,
            round: 894,
        }
    }
}
