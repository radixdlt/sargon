use crate::prelude::*;

#[uniffi::export]
pub fn new_factor_sources_sample() -> FactorSources {
    FactorSources::sample()
}
#[uniffi::export]
pub fn new_factor_sources_sample_other() -> FactorSources {
    FactorSources::sample_other()
}

impl FactorSources {
    /// Panics if `device_factor_source` is not using Babylon crypto parameters
    /// AND marked "main".
    pub fn with_bdfs(device_factor_source: DeviceFactorSource) -> Self {
        assert!(device_factor_source.is_main_bdfs());
        Self::with_factorsource(device_factor_source.into())
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
    use crate::prelude::*;

    #[test]
    fn identifiable_id_uses_factor_source_id() {
        assert_eq!(
            FactorSource::sample_device().id(),
            FactorSource::sample_device().factor_source_id()
        )
    }

    #[test]
    fn inequality() {
        assert_ne!(FactorSources::sample(), FactorSources::sample_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            FactorSources::from_iter(
                [FactorSource::sample(), FactorSource::sample()].into_iter()
            )
            .unwrap()
            .len(),
            1
        )
    }

    #[test]
    fn json_roundtrip_sample() {
        let sut = FactorSources::sample();
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
    use crate::{
        new_factor_sources_sample, new_factor_sources_sample_other,
        HasSampleValues,
    };

    use super::FactorSources;

    #[test]
    fn equality_samples() {
        assert_eq!(FactorSources::sample(), new_factor_sources_sample());
        assert_eq!(
            FactorSources::sample_other(),
            new_factor_sources_sample_other()
        );
    }
}
