use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OnboardingDeepLinkValue {
    /// Indicates the device the user is coming from.
    pub method: DeferredDeepLinkMethod,

    /// Indicates the definition address of the dApp that referred the user to the Wallet
    pub dapp_referrer: Option<DappDefinitionAddress>,

    /// Indicates the particular dApp defining the user flow.
    pub special_dapp: Option<DeferredDeepLinkSpecialDapp>,
}

impl OnboardingDeepLinkValue {
    pub fn new(
        method: DeferredDeepLinkMethod,
        dapp_referrer: Option<DappDefinitionAddress>,
        special_dapp: Option<DeferredDeepLinkSpecialDapp>,
    ) -> Self {
        Self {
            method,
            dapp_referrer,
            special_dapp,
        }
    }
}

impl HasSampleValues for OnboardingDeepLinkValue {
    fn sample() -> Self {
        Self::new(
            DeferredDeepLinkMethod::Mobile,
            Some(DappDefinitionAddress::sample()),
            Some(DeferredDeepLinkSpecialDapp::RadQuest),
        )
    }

    fn sample_other() -> Self {
        Self::new(DeferredDeepLinkMethod::Desktop, None, None)
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
    fn json_roundtrip_referrer_and_special_dapp_missing() {
        let model = SUT::new(DeferredDeepLinkMethod::Desktop, None, None);
        let json = r#"
        {
            "method": "desktop"
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
            "dapp_referrer": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
            "special_dapp": "radquest"
        }
        "#;
        assert_eq_after_json_roundtrip(&model, json);
    }
}
