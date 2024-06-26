use url::ParseError;

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
        let mut result = Vec::new();

        let is_mobile = value.method == DeferredDeepLinkMethod::Mobile;
        if value.radquest {
            result.push(PostOnboardingCard::ContinueRadQuest {
                should_redirect: (is_mobile),
            })
        } else {
            result.push(PostOnboardingCard::StartRadquest);
        }

        let callback_url: Option<Url>;
        if let Some(dapp_callback) = value.dapp_callback {
            callback_url = Url::parse(&dapp_callback).ok();
        } else {
            callback_url = None;
        }

        if value.dapp_referrer.is_some() && is_mobile {
            if let Some(callback_url) = callback_url.clone() {
                result.push(PostOnboardingCard::Dapp {
                    name: ("TODO".to_string()),
                    callback_url: (Some(callback_url)),
                });
            }
        }

        result.push(PostOnboardingCard::Connector);

        if value.dapp_referrer.is_some() && !is_mobile {
            result.push(PostOnboardingCard::Dapp {
                name: ("TODO".to_string()),
                callback_url: (None),
            });
        }

        Self::Onboarding {
            cards: PostOnboardingCards::from_iter(result),
        }
    }
}
