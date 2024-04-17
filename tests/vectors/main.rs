use sargon::prelude::*;

use core::fmt::Debug;
use serde::Deserialize;
use std::str::FromStr;

#[cfg(test)]
mod profile_snapshot_tests {
    use super::*;

    #[test]
    fn v100_100() {
        let (profile, json) =
            fixture_and_json::<Profile>(include_str!(concat!(
                env!("FIXTURES_VECTOR"),
                "only_plaintext_profile_snapshot_version_100.json"
            )))
            .expect("V100 Profile to deserialize");
        assert_json_value_eq_after_roundtrip(&profile, json)
    }
}

#[cfg(test)]
mod cap26_tests {

    use super::*;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct CAP26Vector {
        path: HDPath,

        #[serde(rename = "publicKey")]
        public_key_hex: String,

        #[serde(rename = "entityKind")]
        entity_kind: CAP26EntityKind,

        #[serde(rename = "privateKey")]
        private_key_hex: String,

        #[serde(rename = "entityIndex")]
        entity_index: HDPathValue,

        #[serde(rename = "keyKind")]
        key_kind: CAP26KeyKind,
    }

    #[derive(Deserialize, Debug)]
    struct CAP26Group {
        mnemonic: Mnemonic,
        tests: Vec<CAP26Vector>,
    }
    impl CAP26Group {
        fn test<S, P>(&self)
        where
            P: IsPublicKey<S::Signature>
                + FromStr<Err = CommonError>
                + std::fmt::Debug
                + PartialEq,
            S: IsPrivateKey<P> + FromStr<Err = CommonError> + std::fmt::Debug,
        {
            let seed = self.mnemonic.to_seed("");
            self.tests
                .iter()
                .map(|v| {
                    let private_key = v.private_key::<S, P>()?;
                    let derived: sargon::PrivateKey = match S::curve() {
                        sargon::SLIP10Curve::Curve25519 => MnemonicWithPassphrase::derive_ed25519_private_key(&seed, &v.path).into(),
                        sargon::SLIP10Curve::Secp256k1 => MnemonicWithPassphrase::derive_secp256k1_private_key(&seed, &v.path).into()
                    };
                    assert_eq!(derived.to_hex(), format!("{:?}", private_key));
                    Ok::<(), CommonError>(())
                })
                .collect::<Result<Vec<()>, CommonError>>()
                .expect("All test vectors to pass");
        }
    }

    impl CAP26Vector {
        fn private_key<S, P>(&self) -> Result<S, CommonError>
        where
            P: IsPublicKey<S::Signature>
                + FromStr<Err = CommonError>
                + std::fmt::Debug
                + PartialEq,
            S: IsPrivateKey<P> + FromStr<Err = CommonError>,
        {
            let s: S = self.private_key_hex.parse()?;
            let p: P = self.public_key_hex.parse()?;
            assert_eq!(s.public_key(), p);
            Ok(s)
        }
    }

    #[test]
    fn test_vectors() {
        let secp256k1 = fixture::<CAP26Group>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "cap26_secp256k1.json"
        )))
        .expect("CAP26 Secp256k1 vectors");
        let curve25519 = fixture::<CAP26Group>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "cap26_curve25519.json"
        )))
        .expect("CAP26 Curve25519 vectors");

        secp256k1.test::<Secp256k1PrivateKey, Secp256k1PublicKey>();
        curve25519.test::<Ed25519PrivateKey, Ed25519PublicKey>();
    }
}

#[cfg(test)]
mod bip44_tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Vector {
        path: HDPath,

        #[serde(rename = "publicKeyCompressed")]
        public_key: Secp256k1PublicKey,

        #[serde(rename = "privateKey")]
        private_key_hex: String,

        #[serde(rename = "isStrictBIP44")]
        is_strict_bip44: bool,
    }

    #[derive(Debug, Deserialize)]
    struct Group {
        tests: Vec<Vector>,
        mnemonic: Mnemonic,
    }
    impl Group {
        fn test(&self) {
            let seed = self.mnemonic.to_seed("");
            self.tests
                .iter()
                .map(|v| {
                    let expected_private_key: Secp256k1PrivateKey =
                        v.private_key_hex.parse()?;
                    let derived_private_key: Secp256k1PrivateKey =
                        MnemonicWithPassphrase::derive_secp256k1_private_key(
                            &seed, &v.path,
                        );
                    assert_eq!(derived_private_key, expected_private_key);
                    assert_eq!(
                        &derived_private_key.public_key(),
                        &v.public_key
                    );
                    if !v.is_strict_bip44 {
                        assert!(
                            TryInto::<BIP44LikePath>::try_into(&v.path).is_ok()
                        );
                    } else {
                        assert!(TryInto::<BIP44LikePath>::try_into(&v.path)
                            .is_err());
                    }
                    Ok::<(), CommonError>(())
                })
                .collect::<Result<Vec<()>, CommonError>>()
                .expect("All test vectors to pass");
        }
    }

    #[derive(Debug, Deserialize)]
    struct Fixture {
        #[serde(rename = "testGroups")]
        groups: Vec<Group>,
    }
    impl Fixture {
        fn test(&self) {
            self.groups.iter().for_each(|g| g.test())
        }
    }

    #[test]
    fn test_vectors() {
        let fixture = fixture::<Fixture>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "bip44_secp256k1.json"
        )))
        .expect("BIP44 fixture");

        fixture.test();
    }
}

#[cfg(test)]
mod slip10_tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Deserialize, Clone)]
    struct KeyVector {
        curve: String,

        #[serde(rename = "chainCode")]
        chain_code: String,

        #[serde(rename = "privateKey")]
        private_key: String,

        #[serde(rename = "publicKey")]
        public_key: String,

        fingerprint: String,
        xpub: String,
        xprv: String,
    }
    impl KeyVector {
        fn test(&self, seed: &[u8; 64], path: &HDPath) {
            let maybe_derived: Option<sargon::PrivateKey> =
                match self.curve.as_str() {
                    "ed25519" => Some(
                        MnemonicWithPassphrase::derive_ed25519_private_key(
                            seed, path,
                        )
                        .into(),
                    ),
                    "secp256k1" => Some(
                        MnemonicWithPassphrase::derive_secp256k1_private_key(
                            seed, path,
                        )
                        .into(),
                    ),
                    _ => {
                        assert_eq!(self.curve, "nist256p1");
                        /* P256 not yet supported */
                        None
                    }
                };
            let Some(derived) = maybe_derived else { return };
            assert_eq!(derived.to_hex(), self.private_key);
            assert!(self.public_key.ends_with(&derived.public_key().to_hex()));
        }
    }

    #[derive(Deserialize, Clone)]
    struct TestCase {
        path: HDPath,

        #[serde(rename = "childKeys")]
        child_keys: Vec<KeyVector>,
    }
    impl TestCase {
        fn test(&self, seed: &[u8; 64]) {
            self.child_keys
                .iter()
                .for_each(|k| k.test(seed, &self.path));
        }
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Clone)]
    struct Group {
        seed: String,

        #[serde(rename = "mnemonicPhrase")]
        mnemonic: Mnemonic,

        entropy: String,

        passphrase: BIP39Passphrase,

        #[serde(rename = "masterKeys")]
        master_keys: Vec<KeyVector>,

        #[serde(rename = "testCases")]
        test_cases: Vec<TestCase>,
    }
    impl Group {
        fn test(&self) {
            let seed = self.mnemonic.to_seed(&self.passphrase.0);
            let entropy = ::hex::decode(&self.entropy).unwrap();
            assert_eq!(self.mnemonic, Mnemonic::from_entropy(&entropy));
            assert_eq!(::hex::encode(seed), self.seed);
            self.test_cases.iter().for_each(|c| c.test(&seed));
        }
    }

    #[derive(Deserialize, Clone)]
    struct Fixture {
        #[serde(rename = "testGroups")]
        test_groups: Vec<Group>,
    }
    impl Fixture {
        fn test(&self) {
            self.test_groups.iter().for_each(|g| g.test())
        }
    }

    #[test]
    fn test_vectors() {
        let ten = fixture::<Fixture>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "slip10_tests_#10.json"
        )))
        .expect("SLIP10 #10 fixture");
        let thousand = fixture::<Fixture>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "slip10_tests_#1000.json"
        )))
        .expect("SLIP10 #1000 fixture");

        ten.test();
        thousand.test();
    }
}

#[cfg(test)]
mod wallet_interaction_tests {
    use url::Url;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct Fixture {
        tests: Vec<DappToWalletInteraction>,
    }

    #[test]
    fn test_vector() {
        let fixture = fixture::<Fixture>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "wallet_interactions.json"
        )))
        .expect("BIP44 fixture");

    let metadata = DappToWalletInteractionMetadata {
        network_id: NetworkID::Stokenet,
        dapp_definition_address: DappDefinitionAddress::from_str("account_tdx_2_12xd46c22d6m696lv565t9afn088htudtq275px3qs925ywwty8axze").unwrap(),
        origin: Url::from_str("https://dev-sandbox.rdx-works-main.extratools.works").unwrap(),
        version: 2.into(),
    };

    let authorized_request_with_challenge = DappToWalletInteraction {
        items: DappToWalletInteractionItems::AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems {
            ongoing_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem {
                is_requesting_name: Some(true),
                number_of_requested_email_addresses: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 1,
                }),
                number_of_requested_phone_numbers: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 1,
                }),
            }),
            reset: Some(DappToWalletInteractionResetRequestItem {
                accounts: true,
                persona_data: true,
            }),
            auth: DappToWalletInteractionAuthRequestItem::LoginWithChallenge(DappToWalletInteractionAuthLoginWithChallengeRequestItem {
                challenge: Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap(),
            }),
            ongoing_accounts: Some(DappToWalletInteractionAccountsRequestItem {
                challenge: Some(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
                number_of_accounts: RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 4,
                },
            }),
            one_time_accounts: None,
            one_time_persona_data: None,
        }),
        interaction_id: "d59590ea-d50b-4e8d-a5e1-da3a2574ae5c".into(),
        metadata: metadata.clone(),
    };

    let authorized_request_without_challenge = DappToWalletInteraction {
        items: DappToWalletInteractionItems::AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems {
            ongoing_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem {
                is_requesting_name: Some(true),
                number_of_requested_email_addresses: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 1,
                }),
                number_of_requested_phone_numbers: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 1,
                }),
            }),
            reset: Some(DappToWalletInteractionResetRequestItem {
                accounts: true,
                persona_data: true,
            }),
            auth: DappToWalletInteractionAuthRequestItem::LoginWithoutChallenge,
            ongoing_accounts: Some(DappToWalletInteractionAccountsRequestItem {
                challenge: Some(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
                number_of_accounts: RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 4,
                },
            }),
            one_time_accounts: None,
            one_time_persona_data: None,
        }),
        interaction_id: "d59590ea-d50b-4e8d-a5e1-da3a2574ae5c".into(),
        metadata: metadata.clone(),
    };

    let transaction = DappToWalletInteraction {
        items: DappToWalletInteractionItems::Transaction(DappToWalletInteractionTransactionItems {
            send: DappToWalletInteractionSendTransactionItem {
                version: 1.into(),
                message: Some("test message".into()),
                blobs: Some(vec!["0061736d0100000001c8011c60037f7f7f0060027f7f0060027f7f017f60017f0060037f7f7f017f60017f017f60047f7f7f7f0060017f017e60037f7f7f017e60057f7f7f7f7f0060057f7f7f7f7f017f60027f7e017f60037f7f7e0".into()]),
                transaction_manifest: "CALL_FUNCTION Address(\"package_tdx_2_1pkgxxxxxxxxxplxxxxxxxxxxxxx020379220524xxxxxxxxxe4r780\") \n    \"OneResourcePool\"\n    \"instantiate\"\n    Enum<OwnerRole::Fixed>(Enum<AccessRule::AllowAll>())\n    Enum<AccessRule::AllowAll>() \n    Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\")\n    None;".into(),
            },
        }),
        interaction_id: "4051ff20-03b0-4a48-8205-0e8e8c673289".into(),
        metadata: metadata.clone(),
    };

    let unauthorized_request_1 = DappToWalletInteraction {
        items: DappToWalletInteractionItems::UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems {
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem {
                challenge: Some(Exactly32Bytes::from_hex("84a5234f14a50dee062dc7a6a51f4bdab7cab5faadea05542af2040688d8fb6c").unwrap()),
                number_of_accounts: RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 1,
                },
            }),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem {
                is_requesting_name: Some(true),
                number_of_requested_email_addresses: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 1,
                }),
                number_of_requested_phone_numbers: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 1,
                }),
            }),
        }),
        interaction_id: "51a720a5-9f80-4d0f-8264-704d1645f0af".into(),
        metadata: metadata.clone(),
    };

    let unauthorized_request_2 = DappToWalletInteraction {
        items: DappToWalletInteractionItems::UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems {
            one_time_accounts: Some(DappToWalletInteractionAccountsRequestItem {
                challenge: Some(Exactly32Bytes::from_hex("84a5234f14a50dee062dc7a6a51f4bdab7cab5faadea05542af2040688d8fb6c").unwrap()),
                number_of_accounts: RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::Exactly,
                    quantity: 1,
                },
            }),
            one_time_persona_data: Some(DappToWalletInteractionPersonaDataRequestItem {
                is_requesting_name: Some(true),
                number_of_requested_email_addresses: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 1,
                }),
                number_of_requested_phone_numbers: Some(RequestedQuantity {
                    quantifier: RequestedNumberQuantifier::AtLeast,
                    quantity: 1,
                }),
            }),
        }),
        interaction_id: "51a720a5-9f80-4d0f-8264-704d1645f0af".into(),
        metadata: metadata.clone(),
    };

        let expected_after_decoding = vec![
            authorized_request_with_challenge,
            authorized_request_without_challenge,
            transaction,
            unauthorized_request_1,
            unauthorized_request_2,
        ];

        for (fixture, expected) in fixture.tests.iter().zip(expected_after_decoding.iter()) {
            pretty_assertions::assert_eq!(fixture, expected);
        }
    }
}
