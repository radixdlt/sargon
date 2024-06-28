use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`FactorSource`]s generated by a wallet or manually added by user.
    /// MUST never be empty.
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
    }

    fn sample_other() -> Self {
        Self::from_iter([
            FactorSource::sample_device_olympia(),
            FactorSource::sample_device_babylon(),
        ])
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use uniffi::{
        check_remaining,
        deps::bytes::{Buf, BufMut},
        metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
    };

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
    fn factor_sources_sample_is_not_empty() {
        assert_eq!(SUT::sample().len(), 2);
        assert_eq!(SUT::sample().get_all().len(), 2);
        assert_eq!(SUT::sample().ids().len(), 2);
        assert!(!SUT::sample().is_empty());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn manual_uniffi_conversion_fails_if_factor_sources_is_empty() {
        // This is some advanced techniques...
        let mut bad_value_from_ffi_vec = Vec::new();
        bad_value_from_ffi_vec.put_i32(0); // empty, not allowed
        let bad_value_from_ffi = RustBuffer::from_vec(bad_value_from_ffi_vec);
        let res =
            <IdentifiedVecOf<FactorSource> as Lift<crate::UniFfiTag>>::try_lift(
                bad_value_from_ffi,
            );
        assert!(res.is_err());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            SUT::from_iter(
                [FactorSource::sample(), FactorSource::sample()].into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn json_serialize_of_empty_factor_sources_is_err() {
        assert!(serde_json::to_value(SUT::new()).is_err());
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
                            "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
                            "name": "iPhone",
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
                            "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
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
