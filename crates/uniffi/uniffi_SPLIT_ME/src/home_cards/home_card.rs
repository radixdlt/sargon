use crate::prelude::*;

type InternalHomeCard = sargon::HomeCard;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
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

impl HomeCard {
    pub fn into_internal(&self) -> InternalHomeCard {
        self.clone().into()
    }
}

#[allow(deprecated)]
impl From<InternalHomeCard> for HomeCard {
    fn from(value: InternalHomeCard) -> Self {
        match value {
            InternalHomeCard::DiscoverRadixDapps => {
                panic!(
                    "DiscoverRadixDapps should never be used by hosts anymore."
                )
            }
            InternalHomeCard::StartRadQuest => Self::StartRadQuest,
            InternalHomeCard::ContinueRadQuest => Self::ContinueRadQuest,
            InternalHomeCard::Dapp { icon_url } => Self::Dapp { icon_url },
            InternalHomeCard::Connector => Self::Connector,
        }
    }
}

impl From<HomeCard> for InternalHomeCard {
    fn from(value: HomeCard) -> Self {
        match value {
            HomeCard::StartRadQuest => Self::StartRadQuest,
            HomeCard::ContinueRadQuest => Self::ContinueRadQuest,
            HomeCard::Dapp { icon_url } => Self::Dapp { icon_url },
            HomeCard::Connector => Self::Connector,
        }
    }
}
