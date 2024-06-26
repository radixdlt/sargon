use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OnboardingDeepLinkValue {
    pub method: DeferredDeepLinkMethod,
    pub radquest: bool,
    pub dapp_referrer: Option<DappDefinitionAddress>,
    pub dapp_callback: Option<String>,
}

impl OnboardingDeepLinkValue {
    pub fn new(
        method: DeferredDeepLinkMethod,
        radquest: bool,
        dapp_referrer: Option<DappDefinitionAddress>,
        dapp_callback: impl Into<Option<String>>,
    ) -> Self {
        Self {
            method,
            radquest,
            dapp_referrer,
            dapp_callback: dapp_callback.into(),
        }
    }
}

impl HasSampleValues for OnboardingDeepLinkValue {
    fn sample() -> Self {
        Self::new(
            DeferredDeepLinkMethod::Mobile,
            true,
            Some(DappDefinitionAddress::sample()),
            "https://example.com".to_owned(),
        )
    }

    fn sample_other() -> Self {
        Self::new(DeferredDeepLinkMethod::Desktop, false, None, None)
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

    #[test]
    fn json_roundtrip_referrer_and_callback_missing() {
        let model =
            SUT::new(DeferredDeepLinkMethod::Desktop, false, None, None);
        let json = r#"
        {
            "method": "desktop",
            "radquest": false
        }
        "#;
        assert_eq_after_json_roundtrip(&model, json);
    }

    #[test]
    fn json_roundtrip_complete() {
        let model = SUT::sample();
        let json = r#"
        {
            "method": "mobile",
            "radquest": true,
            "dapp_referrer": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
            "dapp_callback": "https://example.com"
        }
        "#;
        assert_eq_after_json_roundtrip(&model, json);
    }
}
