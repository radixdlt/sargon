use crate::prelude::*;

/// NOT IMPLEMENTED NOR USED YET
///
/// A password based FactorSource is essentially a Input Key Material based Mnemonic,
/// user needs to input the password - key material - every time they use this factor source
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{id}")]
pub struct PasswordFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a PasswordFactorSource to help user
    /// disambiguate between it and another one.
    pub hint: PasswordFactorSourceHint,
}

impl PasswordFactorSource {
    /// Instantiates a new `PasswordFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        hint: PasswordFactorSourceHint,
    ) -> Self {
        Self {
            id,
            common: FactorSourceCommon::new_bdfs(),
            hint,
        }
    }
}

impl From<PasswordFactorSource> for FactorSource {
    fn from(value: PasswordFactorSource) -> Self {
        FactorSource::Password { value }
    }
}

fn new_password_with_mwp(
    mwp: MnemonicWithPassphrase,
    hint: PasswordFactorSourceHint,
) -> PasswordFactorSource {
    let id = FactorSourceIDFromHash::new_for_password(&mwp);
    let mut source = PasswordFactorSource::new(id, hint);
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
}

impl HasSampleValues for PasswordFactorSource {
    fn sample() -> Self {
        new_password_with_mwp(
            MnemonicWithPassphrase::sample_password(),
            PasswordFactorSourceHint::sample(),
        )
    }

    fn sample_other() -> Self {
        new_password_with_mwp(
            MnemonicWithPassphrase::sample_password_other(),
            PasswordFactorSourceHint::sample_other(),
        )
    }
}

impl TryFrom<FactorSource> for PasswordFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        match value {
            FactorSource::Password { value } => Ok(value),
            _ => Err(Self::Error::ExpectedPasswordFactorSourceGotSomethingElse),
        }
    }
}

impl IsFactorSource for PasswordFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Password
    }
}

impl BaseBaseIsFactorSource for PasswordFactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }

    fn name(&self) -> String {
        self.hint.label.clone()
    }

    fn set_name(&mut self, updated: String) {
        self.hint.label = updated;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PasswordFactorSource;

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
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": {
                    "kind": "password",
                    "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                },
                "common": {
                    "addedOn": "2023-09-11T16:05:56.000Z",
                    "cryptoParameters": {
                        "supportedCurves": ["curve25519"],
                        "supportedDerivationPathSchemes": ["cap26"]
                    },
                    "flags": [],
                    "lastUsedOn": "2023-09-11T16:05:56.000Z"
                },
                "hint": {
                    "label": "Password 1"
                }
            }
            "#,
        );
    }

    #[test]
    fn from_factor_source() {
        let sut = SUT::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(SUT::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn kind() {
        assert_eq!(SUT::kind(), FactorSourceKind::Password);
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::ExpectedPasswordFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(SUT::sample().factor_source_id(), SUT::sample().id.into());
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(SUT::sample().factor_source_kind(), SUT::sample().id.kind);
    }

    #[test]
    fn name() {
        let mut sut = SUT::sample();
        assert_eq!(sut.name(), "Password 1");
        sut.set_name("Password 2".to_string());
        assert_eq!(sut.name(), "Password 2");
    }
}
