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

/// An enum describing the different cards that Wallet can display after onboarding.
/// Each card has an associated content and optional action.
pub enum PostOnboardingCard {
    /// Content: "Get started on Radix with RadQuest, and earn XRD and collectible NFTs."
    /// Action: Redirect user to RadQuest.
    StartRadquest,

    /// Content: "Congratulations! Continue your journey on Radquest!"
    /// Action: If `should_redirect`, redirect user to RadQuest. Otherwise, none.
    ContinueRadQuest { should_redirect: bool },

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

impl Identifiable for PostOnboardingCard {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl PostOnboardingCard {
    pub fn sample_start_radquest() -> Self {
        Self::StartRadquest
    }

    pub fn sample_continue_radquest() -> Self {
        Self::ContinueRadQuest {
            should_redirect: true,
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

impl HasSampleValues for PostOnboardingCard {
    fn sample() -> Self {
        Self::sample_start_radquest()
    }

    fn sample_other() -> Self {
        Self::sample_connector()
    }
}
