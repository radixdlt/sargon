use time_utils::now;

use crate::prelude::*;

/// Flags which describe a certain state a FactorSource might be in, e.g. `Main` (BDFS).
pub type FactorSourceFlags = IdentifiedVecOf<FactorSourceFlag>;
impl Identifiable for FactorSourceFlag {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

/// Common properties shared between FactorSources of different kinds, describing
/// its state, when added, and supported cryptographic parameters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCommon {
    /// Cryptographic parameters a certain FactorSource supports, e.g. Elliptic Curves.
    ///
    /// Since Radix Wallet App version 1.3.0, it is possible to add crypto
    /// parameters to a FactorSource, e.g. when a user with a DeviceFactorSource
    /// with babylon crypto parameters, lets call it `B`, with mnemonic `M` adds
    /// `M` again but as an "Olympia" factor source, then the olympia crypto
    /// parameters are added to `B`.
    pub crypto_parameters: FactorSourceCryptoParameters,

    /// When this factor source for originally added by the user.
    pub added_on: Timestamp,

    /// Date of last usage of this factor source
    ///
    /// This is the only mutable property, it is mutable
    /// since we will update it every time this FactorSource
    /// is used.
    pub last_used_on: Timestamp,

    /// Flags which describe a certain state a FactorSource might be in, e.g. `Main` (BDFS).
    pub flags: FactorSourceFlags,
}

impl FactorSourceCommon {
    pub fn with_values<I>(
        crypto_parameters: FactorSourceCryptoParameters,
        added_on: Timestamp,
        last_used_on: Timestamp,
        flags: I,
    ) -> Self
    where
        I: IntoIterator<Item = FactorSourceFlag>,
    {
        Self {
            crypto_parameters,
            added_on,
            last_used_on,
            flags: FactorSourceFlags::from_iter(flags),
        }
    }

    pub fn new<I>(
        crypto_parameters: FactorSourceCryptoParameters,
        flags: I,
    ) -> Self
    where
        I: IntoIterator<Item = FactorSourceFlag>,
    {
        let date = now();
        Self::with_values(crypto_parameters, date, date, flags)
    }

    pub fn new_olympia() -> Self {
        Self::new(FactorSourceCryptoParameters::olympia(), Vec::new())
    }

    pub fn new_babylon() -> Self {
        Self::new(FactorSourceCryptoParameters::babylon(), Vec::new())
    }

    pub fn new_bdfs() -> Self {
        Self::new(FactorSourceCryptoParameters::babylon(), Vec::new())
    }

    pub fn supports_babylon(&self) -> bool {
        self.crypto_parameters.supports_babylon()
    }

    pub fn supports_olympia(&self) -> bool {
        self.crypto_parameters.supports_olympia()
    }
}

impl Default for FactorSourceCommon {
    fn default() -> Self {
        Self::new(FactorSourceCryptoParameters::default(), [])
    }
}

impl HasSampleValues for FactorSourceCommon {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_main_babylon()
    }

    fn sample_other() -> Self {
        Self::sample_olympia()
    }
}

impl FactorSourceCommon {
    /// A sample used to facilitate unit tests.
    pub fn sample_main_babylon() -> Self {
        let date = Timestamp::parse("2023-09-11T16:05:56.000Z").unwrap();
        FactorSourceCommon::with_values(
            FactorSourceCryptoParameters::babylon(),
            date,
            date,
            [FactorSourceFlag::Main],
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_olympia() -> Self {
        let date = Timestamp::parse("2023-09-11T16:05:56.000Z").unwrap();
        FactorSourceCommon::with_values(
            FactorSourceCryptoParameters::olympia(),
            date,
            date,
            [],
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(FactorSourceCommon::sample(), FactorSourceCommon::sample());
        assert_eq!(
            FactorSourceCommon::sample_other(),
            FactorSourceCommon::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceCommon::sample(),
            FactorSourceCommon::sample_other()
        );
    }

    #[test]
    fn default_support_babylon() {
        assert_eq!(
            FactorSourceCommon::default().crypto_parameters,
            FactorSourceCryptoParameters::babylon()
        );
        assert!(FactorSourceCommon::default().supports_babylon());
    }

    #[test]
    fn json_roundtrip() {
        let date = Timestamp::parse("2023-09-11T16:05:56.000Z").unwrap();
        let model = FactorSourceCommon::with_values(
            FactorSourceCryptoParameters::default(),
            date,
            date,
            Vec::new(),
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "addedOn": "2023-09-11T16:05:56.000Z",
                "cryptoParameters": {
                    "supportedCurves": ["curve25519"],
                    "supportedDerivationPathSchemes": ["cap26"]
                },
                "flags": [],
                "lastUsedOn": "2023-09-11T16:05:56.000Z"
            }
            "#,
        );
    }
}
