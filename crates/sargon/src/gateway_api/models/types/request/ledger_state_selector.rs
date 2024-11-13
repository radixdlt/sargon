use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerStateSelector {
    /// If provided, the latest ledger state lower than or equal to the given state version is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) state_version: Option<u64>,

    /// If provided, the latest ledger state lower than or equal to the given round timestamp is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timestamp: Option<String>,

    /// If provided, the ledger state lower than or equal to the given epoch at round 0 is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) epoch: Option<u64>,

    /// If provided must be accompanied with epoch, the ledger state lower than or equal to the given epoch and round is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) round: Option<u64>,
}

impl LedgerStateSelector {
    pub fn new(
        state_version: impl Into<Option<u64>>,
        timestamp: impl Into<Option<String>>,
        epoch: impl Into<Option<u64>>,
        round: impl Into<Option<u64>>,
    ) -> Self {
        Self {
            state_version: state_version.into(),
            timestamp: timestamp.into(),
            epoch: epoch.into(),
            round: round.into(),
        }
    }
}

impl HasSampleValues for LedgerStateSelector {
    fn sample() -> Self {
        Self::new(1, "2021-01-01T00:00:00Z".to_string(), 1, 1)
    }

    fn sample_other() -> Self {
        Self::new(2, "2022-02-02T00:00:00Z".to_string(), 2, 2)
    }
}
