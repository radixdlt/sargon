use crate::decl_identified_array_of;
use crate::decl_never_empty_impl;
use crate::{decl_never_empty_identified_array_of, prelude::*};

decl_never_empty_identified_array_of!(
    /// A collection of [`FactorSource`]s generated by a wallet or manually added by user.
    /// MUST never be empty.
    FactorSources,
    FactorSource
);

impl FactorSources {
    /// Panics if `device_factor_source` is not using Babylon crypto parameters
    /// AND marked "main".
    pub fn with_bdfs(device_factor_source: DeviceFactorSource) -> Self {
        assert!(device_factor_source.is_main_bdfs());
        Self::just(device_factor_source.into())
    }
}

impl HasSampleValues for FactorSources {
    fn sample() -> Self {
        Self::from_iter([
            FactorSource::sample_device(),
            FactorSource::sample_ledger(),
        ])
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::from_iter([
            FactorSource::sample_device_olympia(),
            FactorSource::sample_device_babylon(),
        ])
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSources;

    #[test]
    fn identifiable_id_uses_factor_source_id() {
        assert_eq!(
            FactorSource::sample_device().id(),
            FactorSource::sample_device().factor_source_id()
        )
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            SUT::from_iter(
                [FactorSource::sample(), FactorSource::sample()].into_iter()
            )
            .unwrap()
            .len(),
            1
        )
    }

    #[test]
    fn remove_returns_err_if_empty() {
        let fs = FactorSource::sample();
        let sut = SUT::from_iter([fs.clone()]).unwrap();
        assert_eq!(sut.len(), 1);
        assert_eq!(
            new_factor_sources_removed_by_id(&fs.id(), &sut),
            Err(CommonError::FactorSourcesMustNotBeEmpty)
        );
    }

    #[test]
    fn json_deserialize_of_empty_factor_sources_is_err() {
        assert!(serde_json::from_value::<SUT>(serde_json::Value::Array(
            Vec::new()
        ))
        .is_err());
    }

    #[test]
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "discriminator": "device",
                    "device": {
                        "id": {
                            "kind": "device",
                            "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        },
                        "common": {
                            "flags": ["main"],
                            "addedOn": "2023-09-11T16:05:56.000Z",
                            "cryptoParameters": {
                                "supportedCurves": ["curve25519"],
                                "supportedDerivationPathSchemes": ["cap26"]
                            },
                            "lastUsedOn": "2023-09-11T16:05:56.000Z"
                        },
                        "hint": {
                            "name": "Unknown Name",
                            "model": "iPhone",
                            "mnemonicWordCount": 24
                        }
                    }
                },
                {
                    "discriminator": "ledgerHQHardwareWallet",
                    "ledgerHQHardwareWallet": {
                        "id": {
                            "kind": "ledgerHQHardwareWallet",
                            "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        },
                        "common": {
                            "addedOn": "2023-09-11T16:05:56.000Z",
                            "cryptoParameters": {
                                "supportedCurves": ["curve25519"],
                                "supportedDerivationPathSchemes": ["cap26"]
                            },
                            "flags": ["main"],
                            "lastUsedOn": "2023-09-11T16:05:56.000Z"
                        },
                        "hint": {
                            "name": "Orange, scratched",
                            "model": "nanoS+"
                        }
                    }
                }
            ]
            "#,
        )
    }
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSources;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_factor_sources_sample());
        assert_eq!(SUT::sample_other(), new_factor_sources_sample_other());
    }
}
