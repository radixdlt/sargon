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
pub enum PostOnboardingCard {
    /// Get started on Radix with RadQuest, and earn XRD and collectible NFTs.
    StartRadquest { callback_url: Url },

    /// Congratulations! Continue your journey on Radquest!
    #[display("ContinueRadQuest {:?}", callback_url)]
    ContinueRadQuest { callback_url: Option<Url> },

    /// Now that you’ve got the Radix Wallet, you can continue on with using Gumball!
    #[display("Dapp {}, {:?}", name, callback_url)]
    Dapp {
        name: String,
        callback_url: Option<Url>,
    },

    /// To use Radix Wallet with desktop browsers, finish setup by visiting wallet.radixdlt.com.
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
        Self::StartRadquest {
            callback_url: Url::parse("https://gumball-club.radixdlt.com/")
                .unwrap(),
        }
    }

    pub fn sample_continue_radquest() -> Self {
        Self::ContinueRadQuest {
            callback_url: Some(
                Url::parse("https://gumball-club.radixdlt.com/").unwrap(),
            ),
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
