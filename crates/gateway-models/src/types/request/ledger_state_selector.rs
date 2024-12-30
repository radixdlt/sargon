use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerStateSelector {
    /// If provided, the latest ledger state lower than or equal to the given state version is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_version: Option<u64>,

    /// If provided, the latest ledger state lower than or equal to the given round timestamp is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// If provided, the ledger state lower than or equal to the given epoch at round 0 is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,

    /// If provided must be accompanied with epoch, the ledger state lower than or equal to the given epoch and round is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<u64>,
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

impl From<LedgerState> for LedgerStateSelector {
    fn from(ledger_state: LedgerState) -> Self {
        Self::new(ledger_state.state_version, None, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerStateSelector;

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
            "state_version": 1,
            "timestamp": "2021-01-01T00:00:00Z",
            "epoch": 1,
            "round": 1
        }
        "#,
        );
    }

    #[test]
    fn from_ledger_state() {
        let ledger_state = LedgerState::sample();
        let sut = SUT::from(ledger_state.clone());
        assert_eq!(sut.state_version, Some(ledger_state.state_version));
        assert_eq!(sut.timestamp, None);
        assert_eq!(sut.epoch, None);
        assert_eq!(sut.round, None);
    }
}
