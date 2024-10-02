use crate::prelude::*;

/// The preferences the user has configured off-ledger for a given `AuthorizedDapp`.
/// Allows users, for example, to hide direct deposit claims for a given Dapp.
#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Hash,
    derive_more::Display,
    Default,
    uniffi::Record,
)]
pub struct AuthorizedDappPreferences {
    #[serde(default)]
    pub deposits: AuthorizedDappPreferenceDeposits,
}

impl AuthorizedDappPreferences {
    pub fn new(deposits: AuthorizedDappPreferenceDeposits) -> Self {
        Self { deposits }
    }
}

impl HasSampleValues for AuthorizedDappPreferences {
    fn sample() -> Self {
        Self::new(AuthorizedDappPreferenceDeposits::sample())
    }

    fn sample_other() -> Self {
        Self::new(AuthorizedDappPreferenceDeposits::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDappPreferences;

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
    fn default() {
        let sut = SUT::default();
        assert_eq!(AuthorizedDappPreferenceDeposits::Visible, sut.deposits);
    }

    #[test]
    fn deposits() {
        let mut sut = SUT::sample();
        assert_eq!(AuthorizedDappPreferenceDeposits::Visible, sut.deposits);

        sut.deposits = AuthorizedDappPreferenceDeposits::Hidden;
        assert_eq!(AuthorizedDappPreferenceDeposits::Hidden, sut.deposits);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "deposits": "visible"
            }
            "#,
        );
    }
}
