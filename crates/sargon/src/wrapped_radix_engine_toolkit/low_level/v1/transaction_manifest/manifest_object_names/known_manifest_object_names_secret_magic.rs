use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct KnownManifestObjectNamesSecretMagic {
    pub(crate) bucket_names: Vec<KnownManifestObjectNamesKeyValue>,
    pub(crate) proof_names: Vec<KnownManifestObjectNamesKeyValue>,
    pub(crate) address_reservation_names: Vec<KnownManifestObjectNamesKeyValue>,
    pub(crate) address_names: Vec<KnownManifestObjectNamesKeyValue>,
    pub(crate) intent_names: Vec<KnownManifestObjectNamesKeyValue>,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct KnownManifestObjectNamesKeyValue {
    pub key: u32,
    pub value: String,
}

impl From<ScryptoKnownManifestObjectNames>
    for KnownManifestObjectNamesSecretMagic
{
    fn from(value: ScryptoKnownManifestObjectNames) -> Self {
        let bucket_names = value
            .bucket_names
            .into_iter()
            .map(|(k, v)| KnownManifestObjectNamesKeyValue {
                key: k.0,
                value: v,
            })
            .collect();

        let proof_names = value
            .proof_names
            .into_iter()
            .map(|(k, v)| KnownManifestObjectNamesKeyValue {
                key: k.0,
                value: v,
            })
            .collect();

        let address_reservation_names = value
            .address_reservation_names
            .into_iter()
            .map(|(k, v)| KnownManifestObjectNamesKeyValue {
                key: k.0,
                value: v,
            })
            .collect();

        let address_names = value
            .address_names
            .into_iter()
            .map(|(k, v)| KnownManifestObjectNamesKeyValue {
                key: k.0,
                value: v,
            })
            .collect();

        let intent_names = value
            .intent_names
            .into_iter()
            .map(|(k, v)| KnownManifestObjectNamesKeyValue {
                key: k.0,
                value: v,
            })
            .collect();

        KnownManifestObjectNamesSecretMagic {
            bucket_names,
            proof_names,
            address_reservation_names,
            address_names,
            intent_names,
        }
    }
}

impl From<KnownManifestObjectNamesSecretMagic>
    for ScryptoKnownManifestObjectNames
{
    fn from(value: KnownManifestObjectNamesSecretMagic) -> Self {
        let bucket_names = value
            .bucket_names
            .into_iter()
            .map(|kv| (ScryptoManifestBucket(kv.key), kv.value))
            .collect();

        let proof_names = value
            .proof_names
            .into_iter()
            .map(|kv| (ScryptoManifestProof(kv.key), kv.value))
            .collect();

        let address_reservation_names = value
            .address_reservation_names
            .into_iter()
            .map(|kv| (ScryptoManifestAddressReservation(kv.key), kv.value))
            .collect();

        let address_names = value
            .address_names
            .into_iter()
            .map(|kv| (ScryptoManifestNamedAddress(kv.key), kv.value))
            .collect();

        let intent_names = value
            .intent_names
            .into_iter()
            .map(|kv| (ScryptoManifestIntent(kv.key), kv.value))
            .collect();

        ScryptoKnownManifestObjectNames {
            bucket_names,
            proof_names,
            address_reservation_names,
            address_names,
            intent_names,
        }
    }
}

impl HasSampleValues for KnownManifestObjectNamesSecretMagic {
    fn sample() -> Self {
        Self {
            bucket_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 1,
                    value: "Bucket 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 2,
                    value: "Bucket 2".to_string(),
                },
            ],
            proof_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 1,
                    value: "Proof 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 2,
                    value: "Proof 2".to_string(),
                },
            ],
            address_reservation_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 1,
                    value: "Address Reservation 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 2,
                    value: "Address Reservation 2".to_string(),
                },
            ],
            address_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 1,
                    value: "Address 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 2,
                    value: "Address 2".to_string(),
                },
            ],
            intent_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 1,
                    value: "Intent 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 2,
                    value: "Intent 2".to_string(),
                },
            ],
        }
    }

    fn sample_other() -> Self {
        Self {
            bucket_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 3,
                    value: "Other Bucket 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 4,
                    value: "Other Bucket 2".to_string(),
                },
            ],
            proof_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 3,
                    value: "Other Proof 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 4,
                    value: "Other Proof 2".to_string(),
                },
            ],
            address_reservation_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 3,
                    value: "Other Address Reservation 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 4,
                    value: "Other Address Reservation 2".to_string(),
                },
            ],
            address_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 3,
                    value: "Other Address 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 4,
                    value: "Other Address 2".to_string(),
                },
            ],
            intent_names: vec![
                KnownManifestObjectNamesKeyValue {
                    key: 3,
                    value: "Other Intent 1".to_string(),
                },
                KnownManifestObjectNamesKeyValue {
                    key: 4,
                    value: "Other Intent 2".to_string(),
                },
            ],
        }
    }
}
