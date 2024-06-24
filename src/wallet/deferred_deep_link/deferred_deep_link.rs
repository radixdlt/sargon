use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum DeferredDeepLink {
    Onboarding { cards: PostOnboardingCards },
}

impl From<OnboardingDeepLinkValue> for DeferredDeepLink {
    fn from(value: OnboardingDeepLinkValue) -> Self {
        Self::Onboarding {
            cards: PostOnboardingCards::from_iter([
                PostOnboardingCard::RadQuest {
                    already_visited: value.radquest,
                },
                PostOnboardingCard::Connector,
            ]),
        }
    }
}
