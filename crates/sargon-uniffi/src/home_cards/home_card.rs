use crate::prelude::*;
use sargon::HomeCard as InternalHomeCard;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
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
    Dapp { icon_url: Option<Url> },

    /// Content: "To use Radix Wallet with desktop browsers, finish setup by visiting wallet.radixdlt.com"
    /// Action: None
    Connector,
}

impl From<InternalHomeCard> for HomeCard {
    fn from(value: InternalHomeCard) -> Self {
        match value {
            InternalHomeCard::StartRadQuest => HomeCard::StartRadQuest,
            InternalHomeCard::ContinueRadQuest => HomeCard::ContinueRadQuest,
            InternalHomeCard::Dapp { icon_url } => HomeCard::Dapp { icon_url },
            InternalHomeCard::Connector => HomeCard::Connector,
        }
    }
}

impl Into<InternalHomeCard> for HomeCard {
    fn into(self) -> InternalHomeCard {
        match self {
            HomeCard::StartRadQuest => InternalHomeCard::StartRadQuest,
            HomeCard::ContinueRadQuest => InternalHomeCard::ContinueRadQuest,
            HomeCard::Dapp { icon_url } => InternalHomeCard::Dapp { icon_url },
            HomeCard::Connector => InternalHomeCard::Connector,
        }
    }
}