use crate::prelude::*;

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
    uniffi::Enum,
)]

/// An enum describing the different cards that Wallet can display on home page.
/// Each card has an associated content and optional action.
pub enum HomeCard {
    /// Content: "Get started on Radix with RadQuest, and earn XRD and collectible NFTs."
    /// Action: Redirect user to RadQuest.
    StartRadQuest,

    /// Content: "Congratulations! Continue your journey on Radquest!"
    /// Action: If `should_redirect`, redirect user to RadQuest with the given `tracking_data`. Otherwise, none.
    #[display(
        "ContinueRadquest should_redirect: {}, tracking_data: {:?}",
        should_redirect,
        tracking_data
    )]
    ContinueRadQuest {
        should_redirect: bool,
        tracking_data: Option<String>,
    },

    /// Content: "Now that you’ve got the Radix Wallet, you can continue on with using {name}!"
    /// Action: If `callback_url` is available, redirect user to it. Otherwise, none.
    #[display("Dapp {}, {:?}", name, callback_url)]
    Dapp {
        name: String,
        callback_url: Option<Url>,
    },

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

impl HomeCard {
    pub fn sample_start_radquest() -> Self {
        Self::StartRadQuest
    }

    pub fn sample_continue_radquest() -> Self {
        Self::ContinueRadQuest {
            should_redirect: true,
            tracking_data: None,
        }
    }

    pub fn sample_dapp() -> Self {
        Self::Dapp {
            name: "OciSwap".into(),
            callback_url: None,
        }
    }

    pub fn sample_connector() -> Self {
        Self::Connector
    }
}

impl HasSampleValues for HomeCard {
    fn sample() -> Self {
        Self::sample_start_radquest()
    }

    fn sample_other() -> Self {
        Self::sample_connector()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeferredDeepLinkMethod;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
