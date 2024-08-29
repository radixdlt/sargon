use crate::prelude::*;

/// The specific Asset exception rule, which overrides the general
///  `deposit_rule` of a `ThirdPartyDeposits` settings.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("rule: {}, address: {}", self.exception_rule, self.address)]
pub struct AssetException {
    /// Address of an asset to either deny or allow, as an exception overriding the `ThirdPartyDeposits`'s general `deposit_rule`.
    pub address: ResourceAddress,

    /// Either deny or allow the `address`.
    pub exception_rule: DepositAddressExceptionRule,
}

impl HasSampleValues for AssetException {
    fn sample() -> Self {
        Self::new(
            ResourceAddress::sample(),
            DepositAddressExceptionRule::Allow,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceAddress::sample_other(),
            DepositAddressExceptionRule::Deny,
        )
    }
}

impl Identifiable for AssetException {
    type ID = ResourceAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

impl AssetException {
    /// Instantiates a new `AssetException` with the specified `ResourceAddress` and rule.
    pub fn new(
        address: ResourceAddress,
        exception_rule: DepositAddressExceptionRule,
    ) -> Self {
        Self {
            address,
            exception_rule,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetException;

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
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "address" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
                "exceptionRule" : "allow"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_sample_other() {
        let sut = SUT::sample_other();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "address" : "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j",
                "exceptionRule" : "deny"
            }
            "#,
        )
    }

    #[test]
    fn inequality_allow_ne_deny() {
        let a = SUT::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let b = SUT::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Deny,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_allow_different_addresses() {
        let a = SUT::new(
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let b = SUT::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        assert_ne!(a, b);
    }
}
