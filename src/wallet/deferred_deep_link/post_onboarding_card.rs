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
    RadQuest { already_visited: bool },
    Dapp { callback_url: String },
    Connector,
}

impl Identifiable for PostOnboardingCard {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl PostOnboardingCard {
    pub fn sample_radquest_visited() -> Self {
        Self::RadQuest {
            already_visited: true,
        }
    }

    pub fn sample_radquest_not_visited() -> Self {
        Self::RadQuest {
            already_visited: false,
        }
    }

    pub fn sample_dapp() -> Self {
        Self::Dapp {
            callback_url: "https://example.com".into(),
        }
    }

    pub fn sample_connector() -> Self {
        Self::Connector
    }
}

impl HasSampleValues for PostOnboardingCard {
    fn sample() -> Self {
        Self::sample_radquest_visited()
    }

    fn sample_other() -> Self {
        Self::sample_dapp()
    }
}
