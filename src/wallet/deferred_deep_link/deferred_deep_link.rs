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

        let callback_url: Option<Url>;
        if let Some(dapp_callback) = value.dapp_callback {
            callback_url = Url::parse(&dapp_callback).ok();
        } else {
            callback_url = None;
        }

        if value.radquest {
            if value.method == DeferredDeepLinkMethod::Mobile {
                result.push(PostOnboardingCard::ContinueRadQuest {
                    callback_url: callback_url.clone(),
                });
            } else {
                result.push(PostOnboardingCard::ContinueRadQuest {
                    callback_url: (None),
                });
            }
        } else if let Some(callback_url) = callback_url.clone() {
            result.push(PostOnboardingCard::StartRadquest {
                callback_url: (callback_url),
            });
        }

        result.push(PostOnboardingCard::Connector);

        if let Some(_referer) = value.dapp_referrer {
            if value.method == DeferredDeepLinkMethod::Desktop {
                result.push(PostOnboardingCard::Dapp {
                    name: "??".to_string(), // How should we set name from dappDefinitionAddress?
                    callback_url: callback_url.clone(),
                })
            }
        }

        Self::Onboarding {
            cards: PostOnboardingCards::from_iter(result),
        }
    }
}
