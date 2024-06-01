use crate::prelude::*;
use crate::prelude::*;

/// An Arculus card, a hierarchal deterministic wallet capable of CAP26 derivation
/// which users interact with by placing it near their host device, which
/// communicates with the card over NFC.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{hint} : {id}")]
pub struct ArculusCardFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Arculus Card.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a ArculusCardFactorSource to help user disambiguate
    /// between it and another one.
    pub hint: ArculusCardHint,
}

impl From<ArculusCardFactorSource> for FactorSource {
    fn from(value: ArculusCardFactorSource) -> Self {
        FactorSource::ArculusCard { value }
    }
}

fn new_arculus_with_mwp(
    mwp: MnemonicWithPassphrase,
    hint: ArculusCardHint,
    common: FactorSourceCommon,
) -> ArculusCardFactorSource {
    let id = FactorSourceIDFromHash::new_for_arculus(&mwp);
    let mut source = ArculusCardFactorSource::new(id, common, hint);
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
}

impl ArculusCardFactorSource {
    /// Instantiates a new `ArculusCardFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        hint: ArculusCardHint,
    ) -> Self {
        Self { id, common, hint }
    }
}

impl HasSampleValues for ArculusCardFactorSource {
    fn sample() -> Self {
        new_arculus_with_mwp(
            MnemonicWithPassphrase::sample_arculus(),
            ArculusCardHint::sample(),
            FactorSourceCommon::new_bdfs(false),
        )
    }

    fn sample_other() -> Self {
        new_arculus_with_mwp(
            MnemonicWithPassphrase::sample_arculus_other(),
            ArculusCardHint::sample_other(),
            FactorSourceCommon::new_bdfs(false),
        )
    }
}

impl TryFrom<FactorSource> for ArculusCardFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        match value {
            FactorSource::ArculusCard { value } => Ok(value),
            _ => Err(
                Self::Error::ExpectedArculusCardFactorSourceGotSomethingElse,
            ),
        }
    }
}
impl IsFactorSource for ArculusCardFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::ArculusCard
    }
}
impl BaseIsFactorSource for ArculusCardFactorSource {
    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            ArculusCardFactorSource::sample(),
            ArculusCardFactorSource::sample()
        );
        assert_eq!(
            ArculusCardFactorSource::sample_other(),
            ArculusCardFactorSource::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            ArculusCardFactorSource::sample(),
            ArculusCardFactorSource::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = ArculusCardFactorSource::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"            
            {
                "id": {
                    "kind": "arculusCard",
                    "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
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
                    "name": "Silver",
                    "model": "arculusColdStorageWallet"
                }
            }
            "#,
        );
    }

    #[test]
    fn from_factor_source() {
        let sut = ArculusCardFactorSource::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(ArculusCardFactorSource::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            ArculusCardFactorSource::try_from(factor_source),
            Err(CommonError::ExpectedArculusCardFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(
            ArculusCardFactorSource::sample().factor_source_id(),
            ArculusCardFactorSource::sample().id.into()
        );
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(
            ArculusCardFactorSource::sample().factor_source_kind(),
            ArculusCardFactorSource::sample().id.kind
        );
    }
}