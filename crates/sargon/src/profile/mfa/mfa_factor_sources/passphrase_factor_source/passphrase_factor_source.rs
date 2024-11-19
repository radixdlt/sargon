use crate::prelude::*;

/// NOT IMPLEMENTED NOR USED YET
///
/// A passphrase based FactorSource is essentially a Input Key Material based Mnemonic,
/// user needs to input the passphrase - key material - every time they use this factor source
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
pub struct PassphraseFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,
}

impl PassphraseFactorSource {
    /// Instantiates a new `PassphraseFactorSource`
    pub fn new(id: FactorSourceIDFromHash) -> Self {
        Self {
            id,
            common: FactorSourceCommon::new_bdfs(false),
        }
    }
}

impl From<PassphraseFactorSource> for FactorSource {
    fn from(value: PassphraseFactorSource) -> Self {
        FactorSource::Passphrase { value }
    }
}

fn new_passphrase_with_mwp(
    mwp: MnemonicWithPassphrase,
) -> PassphraseFactorSource {
    let id = FactorSourceIDFromHash::new_for_passphrase(&mwp);
    let mut source = PassphraseFactorSource::new(id);
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
}

impl HasSampleValues for PassphraseFactorSource {
    fn sample() -> Self {
        new_passphrase_with_mwp(MnemonicWithPassphrase::sample_passphrase())
    }

    fn sample_other() -> Self {
        new_passphrase_with_mwp(
            MnemonicWithPassphrase::sample_passphrase_other(),
        )
    }
}

impl TryFrom<FactorSource> for PassphraseFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        match value {
            FactorSource::Passphrase { value } => Ok(value),
            _ => {
                Err(Self::Error::ExpectedPassphraseFactorSourceGotSomethingElse)
            }
        }
    }
}

impl HasFactorSourceKind for PassphraseFactorSource {
    fn factor_source_kind() -> FactorSourceKind {
        FactorSourceKind::Passphrase
    }
}
impl BaseIsFactorSource for PassphraseFactorSource {
    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PassphraseFactorSource;

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
                    "kind": "passphrase",
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
        assert_eq!(SUT::factor_source_kind(), FactorSourceKind::Passphrase);
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::ExpectedPassphraseFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(SUT::sample().factor_source_id(), SUT::sample().id.into());
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(
            SUT::sample().get_factor_source_kind(),
            SUT::sample().id.kind
        );
    }
}
