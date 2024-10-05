use crate::prelude::*;
use std::cmp::Ordering;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    EnumAsInner,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]

/// An enum describing the different cards that Wallet can display on home page.
/// Each card has an associated content and optional action.
pub enum HomeCard {
    /// Content: "Start RadQuest, learn about Radix, earn XRD and collectibles."
    /// Action: Redirect user to RadQuest.
    StartRadQuest,

    /// Content: "Continue your Radix journey in your browser. Tap to dismiss."
    /// Action: None.
    ContinueRadQuest,

    /// Content: "You can now connect with your Radix Wallet. Tap to dismiss."
    /// Action: None.
    #[display("Dapp {:?}", icon_url)]
    Dapp { icon_url: Option<Url> },

    /// Content: "To use Radix Wallet with desktop browsers, finish setup by visiting wallet.radixdlt.com"
    /// Action: None
    Connector,
}

impl Identifiable for HomeCard {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl HasSampleValues for HomeCard {
    fn sample() -> Self {
        Self::StartRadQuest
    }

    fn sample_other() -> Self {
        Self::ContinueRadQuest
    }
}

impl Ord for HomeCard {
    fn cmp(&self, other: &Self) -> Ordering {
        fn order_value(card: &HomeCard) -> usize {
            use HomeCard::*;

            match card {
                StartRadQuest => 0,
                ContinueRadQuest => 1,
                Dapp { .. } => 2,
                Connector => 3,
            }
        }

        order_value(self).cmp(&order_value(other))
    }
}

impl PartialOrd for HomeCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HomeCard;

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
    fn identifiable() {
        assert_eq!(SUT::sample().id(), SUT::sample());
    }

    #[test]
    fn compare() {
        assert!(SUT::sample() < SUT::sample_other());
    }
}
