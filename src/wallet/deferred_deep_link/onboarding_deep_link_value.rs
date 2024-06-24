use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OnboardingDeepLinkValue {
    pub dapp_referrer: AccountAddress,
    pub dapp_callback: String,
    pub method: DeferredDeepLinkMethod,
    pub radquest: bool,
}

impl OnboardingDeepLinkValue {
    pub fn new(
        dapp_referrer: AccountAddress,
        dapp_callback: impl Into<String>,
        method: DeferredDeepLinkMethod,
        radquest: bool,
    ) -> Self {
        Self {
            dapp_referrer,
            dapp_callback: dapp_callback.into(),
            method,
            radquest,
        }
    }
}

impl HasSampleValues for OnboardingDeepLinkValue {
    fn sample() -> Self {
        Self::new(
            AccountAddress::sample(),
            "https://example.com",
            DeferredDeepLinkMethod::Mobile,
            true,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_other(),
            "https://example.com",
            DeferredDeepLinkMethod::Desktop,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OnboardingDeepLinkValue;

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
