use sargon::prelude::*;

use core::fmt::Debug;
use serde::Deserialize;
use std::str::FromStr;

#[cfg(test)]
mod profile_snapshot_tests {
    use super::*;

    /// We cannot do a roundtrip test here because
    /// `only_plaintext_profile_snapshot_version_100.json` does contain `p2pLinks`
    /// and with the [CAP-36][doc] feature `p2pLinks` are no longer stored in `Profile`.
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3251863610/CAP-36+WebRTC+Clients+Protocol
    #[test]
    fn v100_100() {
        fixture_and_json::<Profile>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "only_plaintext_profile_snapshot_version_100.json"
        )))
        .expect("V100 Profile to deserialize");
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
        entity_index: u32,

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
            let curve = S::curve();
            let seed = self.mnemonic.to_seed("");
            self.tests
                .iter()
                .map(|v| {
                    let private_key = v.private_key::<S, P>()?;
                    let derived =
                        seed.derive_private_key_curve(curve, v.path.clone());
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
        path: BIP44LikePath,

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
                    let derived_private_key = seed.derive_private_key(&v.path);
                    assert_eq!(
                        derived_private_key.private_key,
                        PrivateKey::from(expected_private_key)
                    );
                    assert_eq!(
                        derived_private_key.public_key().public_key,
                        PublicKey::from(v.public_key)
                    );
                    assert_eq!(v.path.is_canonical(), v.is_strict_bip44);

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
        fn test(&self, seed: &BIP39Seed, path: &HDPath) {
            let maybe_curve: Option<SLIP10Curve> = match self.curve.as_str() {
                "ed25519" => Some(SLIP10Curve::Curve25519),
                "secp256k1" => Some(SLIP10Curve::Secp256k1),
                _ => {
                    assert_eq!(self.curve, "nist256p1");
                    /* P256 not yet supported */
                    None
                }
            };
            let Some(curve) = maybe_curve else { return };
            let derived = seed.derive_private_key_curve(curve, path.clone());
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
        fn test(&self, seed: &BIP39Seed) {
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
            let bytes = NonEmptyMax32Bytes::from_str(&self.entropy).unwrap();
            let entropy = BIP39Entropy::try_from(bytes).unwrap();
            assert_eq!(self.mnemonic, Mnemonic::from_entropy(entropy));
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

mod encrypted_profile_tests {
    use std::collections::HashSet;

    use serde::Serialize;

    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    struct IdentifiableMnemonic {
        #[serde(rename = "factorSourceID")]
        factor_source_id: FactorSourceID,
        #[serde(rename = "mnemonicWithPassphrase")]
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    struct EncryptedSnapshotWithPassword {
        password: String,
        snapshot: EncryptedProfileSnapshot,
    }
    impl EncryptedSnapshotWithPassword {
        fn decrypted(&self) -> Result<Profile> {
            self.snapshot.decrypt(self.password.clone())
        }
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Fixture {
        #[serde(rename = "_snapshotVersion")]
        snapshot_version: ProfileSnapshotVersion,
        mnemonics: Vec<IdentifiableMnemonic>,
        #[serde(rename = "encryptedSnapshots")]
        encrypted_snapshots: Vec<EncryptedSnapshotWithPassword>,
        plaintext: Profile,
    }

    impl Fixture {
        fn validate_all_entities_with_mnemonics(&self) -> Result<()> {
            self.plaintext
                .networks
                .clone()
                .into_iter()
                .try_for_each(|n| {
                    let test = |security_state: EntitySecurityState| {
                        let control = security_state.as_unsecured().unwrap();

                        let tx_signing_instance =
                            control.transaction_signing.clone();

                        let factor_source_id_from_hash =
                            tx_signing_instance.factor_source_id;

                        if factor_source_id_from_hash.kind
                            != FactorSourceKind::Device
                        {
                            return;
                        }

                        let factor_source_id: FactorSourceID =
                            factor_source_id_from_hash.into();

                        let factor_source = self
                            .plaintext
                            .factor_sources
                            .clone()
                            .into_iter()
                            .find(|x| x.factor_source_id() == factor_source_id);

                        assert!(factor_source.is_some());

                        let mnemonic = self
                            .mnemonics
                            .clone()
                            .into_iter()
                            .find(|x| x.factor_source_id == factor_source_id)
                            .unwrap();

                        let public_key = mnemonic
                            .mnemonic_with_passphrase
                            .to_seed()
                            .derive_private_key(
                                &tx_signing_instance.derivation_path(),
                            )
                            .public_key();

                        assert_eq!(public_key, tx_signing_instance.public_key);
                    };

                    n.accounts
                        .into_iter()
                        .map(|x| x.security_state)
                        .for_each(test);

                    n.personas
                        .into_iter()
                        .map(|x| x.security_state)
                        .for_each(test);

                    Ok::<(), CommonError>(())
                })?;
            Ok(())
        }

        fn validate(&self) -> Result<Vec<Profile>> {
            let decryptions: Vec<Profile> = self
                .encrypted_snapshots
                .clone()
                .into_iter()
                .map(|x| x.decrypted())
                .collect::<Result<Vec<Profile>>>()
                .unwrap();

            decryptions
                .clone()
                .into_iter()
                .for_each(|x| assert_eq!(x, self.plaintext));

            let ids = self
                .plaintext
                .factor_sources
                .clone()
                .into_iter()
                .filter(|x| x.factor_source_kind() == FactorSourceKind::Device)
                .map(|x| x.factor_source_id())
                .collect::<HashSet<FactorSourceID>>();

            assert_eq!(
                ids,
                self.mnemonics
                    .clone()
                    .into_iter()
                    .map(|x| x.factor_source_id)
                    .collect::<HashSet<FactorSourceID>>()
            );

            self.validate_all_entities_with_mnemonics()?;

            Ok(decryptions)
        }

        fn test(&self) {
            let decrypted_profiles = self.validate().unwrap();
            decrypted_profiles.clone().into_iter().for_each(|x| {
                assert_eq!(
                    x.header.snapshot_version,
                    self.plaintext.header.snapshot_version
                )
            });
            assert_json_roundtrip(self);
        }
    }

    #[test]
    fn test_vectors() {
        let fixture = fixture::<Fixture>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "multi_profile_snapshots_test_version_100_patch_after_app_version_120.json"
        )))
        .expect("Encrypted Profile tests");

        fixture.test();
    }
}

#[cfg(test)]
mod dapp_to_wallet_interaction_tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_vector() {
        let decoded_wallet_interactions =
            fixture::<Vec<DappToWalletInteraction>>(include_str!(concat!(
                env!("FIXTURES_VECTOR"),
                "wallet_interactions_dapp_to_wallet.json"
            )))
            .expect("wallet_interactions_dapp_to_wallet fixture");

        let metadata = DappToWalletInteractionMetadata::new(
            2,
            NetworkID::Stokenet,
            "https://dev-sandbox.rdx-works-main.extratools.works/",
            DappDefinitionAddress::from_str(
                "account_tdx_2_12xd46c22d6m696lv565t9afn088htudtq275px3qs925ywwty8axze",
            )
            .unwrap(),
        );

        let authorized_request_with_challenge_items = DappToWalletInteractionItems::AuthorizedRequest(
        DappToWalletInteractionAuthorizedRequestItems::new(
            DappToWalletInteractionAuthRequestItem::LoginWithChallenge(
                DappToWalletInteractionAuthLoginWithChallengeRequestItem::new(
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
                )
            ),
            DappToWalletInteractionResetRequestItem::new(
                true,
                true,
            ),
            DappToWalletInteractionAccountsRequestItem::new(
                RequestedQuantity::at_least(4),
                DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
            ),
            DappToWalletInteractionPersonaDataRequestItem::new(
                true,
                RequestedQuantity::exactly(1),
                RequestedQuantity::exactly(1),
            ),
            None,
            None,
            None,
        )
    );

        let authorized_request_with_challenge = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "d59590ea-d50b-4e8d-a5e1-da3a2574ae5c",
            )
            .unwrap(),
            authorized_request_with_challenge_items,
            metadata.clone(),
        );

        let authorized_request_without_challenge_items = DappToWalletInteractionItems::AuthorizedRequest(
            DappToWalletInteractionAuthorizedRequestItems::new(
                DappToWalletInteractionAuthRequestItem::LoginWithoutChallenge,
                DappToWalletInteractionResetRequestItem::new(
                    true,
                    true,
                ),
                DappToWalletInteractionAccountsRequestItem::new(
                    RequestedQuantity::exactly(4),
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
                ),
                DappToWalletInteractionPersonaDataRequestItem::new(
                    true,
                    RequestedQuantity::at_least(1),
                    RequestedQuantity::at_least(1),
                ),
                None,
                None,
                None,
            )
        );

        let authorized_request_without_challenge = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "d59590ea-d50b-4e8d-a5e1-da3a2574ae5c",
            )
            .unwrap(),
            authorized_request_without_challenge_items,
            metadata.clone(),
        );

        let updated_metadata = metadata.clone().with_updated_origin(
            "https://dev-sandbox.rdx-works-main.extratools.works",
        );
        let transaction = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "4051ff20-03b0-4a48-8205-0e8e8c673289",
            )
            .unwrap(),
            DappToWalletInteractionItems::Transaction(
                DappToWalletInteractionTransactionItems::new(
                    DappToWalletInteractionSendTransactionItem::sample_other(),
                ),
            ),
            updated_metadata,
        );

        let unauthorized_request_1_items =  DappToWalletInteractionItems::UnauthorizedRequest(
            DappToWalletInteractionUnauthorizedRequestItems::new(
                DappToWalletInteractionAccountsRequestItem::new(
                    RequestedQuantity::at_least(1),
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("84a5234f14a50dee062dc7a6a51f4bdab7cab5faadea05542af2040688d8fb6c").unwrap())
                ),
                DappToWalletInteractionPersonaDataRequestItem::new(
                    true,
                    RequestedQuantity::exactly(1),
                    RequestedQuantity::exactly(1),
                ),
            )
        );

        let unauthorized_request_1 = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "51a720a5-9f80-4d0f-8264-704d1645f0af",
            )
            .unwrap(),
            unauthorized_request_1_items,
            metadata.clone(),
        );

        let unauthorized_request_2_items =  DappToWalletInteractionItems::UnauthorizedRequest(
            DappToWalletInteractionUnauthorizedRequestItems::new(
                DappToWalletInteractionAccountsRequestItem::new(
                    RequestedQuantity::exactly(1),
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("84a5234f14a50dee062dc7a6a51f4bdab7cab5faadea05542af2040688d8fb6c").unwrap())
                ),
                DappToWalletInteractionPersonaDataRequestItem::new(
                    true,
                    RequestedQuantity::at_least(1),
                    RequestedQuantity::at_least(1),
                ),
            )
        );

        let unauthorized_request_2 = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "51a720a5-9f80-4d0f-8264-704d1645f0af",
            )
            .unwrap(),
            unauthorized_request_2_items,
            metadata.clone(),
        );

        let account_proof_request_items = DappToWalletInteractionItems::AuthorizedRequest(
            DappToWalletInteractionAuthorizedRequestItems::new(
                DappToWalletInteractionAuthRequestItem::UsePersona(
                    DappToWalletInteractionAuthUsePersonaRequestItem::new(
                        IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva")
                            .unwrap(),
                    )
                ),
                DappToWalletInteractionResetRequestItem::new(
                    false,
                    false,
                ),
                None,
                None,
                None,
                None,
                DappToWalletInteractionProofOfOwnershipRequestItem::new(
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("4c85e4a903ab97450ef83763f8d4ca55a43efe843e1d2ced78a4940e5c397c9c").unwrap()), 
                    vec![
                        AccountAddress::from_str("account_tdx_2_12ytkalad6hfxamsz4a7r8tevz7ahurfj58dlp4phl4nca5hs0hpu90").unwrap(),
                    ],
                    None,
                ),
            )
        );

        let account_proof_request = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "2916ad16-52a0-4564-a611-4971883c1322",
            )
            .unwrap(),
            account_proof_request_items,
            metadata.clone(),
        );

        let accounts_and_persona_proof_request_items = DappToWalletInteractionItems::AuthorizedRequest(
            DappToWalletInteractionAuthorizedRequestItems::new(
                DappToWalletInteractionAuthRequestItem::UsePersona(
                    DappToWalletInteractionAuthUsePersonaRequestItem::new(
                        IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva")
                            .unwrap(),
                    )
                ),
                DappToWalletInteractionResetRequestItem::new(
                    false,
                    false,
                ),
                None,
                None,
                None,
                None,
                DappToWalletInteractionProofOfOwnershipRequestItem::new(
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()), 
                    vec![
                        AccountAddress::from_str("account_tdx_2_12ytkalad6hfxamsz4a7r8tevz7ahurfj58dlp4phl4nca5hs0hpu90").unwrap(),
                        AccountAddress::from_str("account_tdx_2_129qeystv8tufmkmjrry2g6kadhhfh4f7rd0x3t9yagcvfhspt62paz").unwrap(),
                    ],
                    IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva").unwrap(),
                ),
            )
        );

        let accounts_and_persona_proof_request = DappToWalletInteraction::new(
            WalletInteractionId::from_str(
                "17d530f6-0cb6-4122-8540-64e46a2e0f84",
            )
            .unwrap(),
            accounts_and_persona_proof_request_items,
            metadata.clone(),
        );

        let interactions = vec![
            authorized_request_with_challenge,
            authorized_request_without_challenge,
            transaction,
            unauthorized_request_1,
            unauthorized_request_2,
            account_proof_request,
            accounts_and_persona_proof_request,
        ];

        for (fixture, expected) in
            decoded_wallet_interactions.iter().zip(interactions.iter())
        {
            pretty_assertions::assert_eq!(fixture, expected);
        }

        let raw_wallet_interactions =
            fixture::<Vec<Value>>(include_str!(concat!(
                env!("FIXTURES_VECTOR"),
                "wallet_interactions_dapp_to_wallet.json"
            )))
            .expect("wallet_interactions_dapp_to_wallet fixture");

        let encoded_interactions =
            serde_json::to_string(&interactions).unwrap();
        let serde_value: Vec<Value> =
            serde_json::from_str(&encoded_interactions).unwrap();

        for (fixture, expected) in
            raw_wallet_interactions.iter().zip(serde_value.iter())
        {
            pretty_assertions::assert_eq!(fixture, expected);
        }
    }
}

#[cfg(test)]
mod wallet_to_dapp_interaction_tests {
    use super::*;

    use serde_json::Value;

    #[test]
    fn test_vector() {
        let persona_data =
            WalletToDappInteractionPersonaDataRequestResponseItem::new(
                PersonaDataEntryName::new(
                    PersonaDataNameVariant::Western,
                    "Family",
                    "Given",
                    "Nick",
                )
                .unwrap(),
                vec![PersonaDataEntryEmailAddress::new("some@gmail.com")
                    .unwrap()],
                vec![PersonaDataEntryPhoneNumber::new("071234579").unwrap()],
            );

        let account_1 = WalletInteractionWalletAccount::new(
            AccountAddress::from_str("account_tdx_2_129qeystv8tufmkmjrry2g6kadhhfh4f7rd0x3t9yagcvfhspt62paz")
            .unwrap(),
            DisplayName::sample(),
            AppearanceID::new(0).unwrap(),
        );

        let account_2 = WalletInteractionWalletAccount::new(
            AccountAddress::from_str("account_tdx_2_128928hvf6pjr3rx2xvdw6ulf7pc8g88ya8ma3j8dtjmntckz09fr3n")
            .unwrap(),
            DisplayName::sample_other(),
            AppearanceID::new(1).unwrap(),
        );

        let authorized_request_response_items = WalletToDappInteractionResponseItems::AuthorizedRequest(
            WalletToDappInteractionAuthorizedRequestResponseItems::new(
                WalletToDappInteractionAuthRequestResponseItem::LoginWithChallenge(
                    WalletToDappInteractionAuthLoginWithChallengeRequestResponseItem::new(
                        DappWalletInteractionPersona::new(
                            IdentityAddress::from_str("identity_tdx_2_12twas58v4sthsmuky5653dup0drez3vcfwsfm6kp40qu9qyt8fgts6")
                            .unwrap(),
                            "Usdudh",
                        ),
                        DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("069ef236486d4cd5706b5e5b168e19f750ffd1b4876529a0a9de966d50a15ab7")
                        .unwrap()),
                        WalletToDappInteractionAuthProof::new(
                            PublicKey::from_str("ff8aee4c625738e35d837edb11e33b8abe0d6f40849ca1451edaba84d04d0699")
                            .unwrap(),
                            Signature::from_str("10177ac7d486691777133ffe59d46d55529d86cb1c4ce66aa82f432372f33e24d803d8498f42e26fe113c030fce68c526aeacff94334ba5a7f7ef84c2936eb05")
                            .unwrap()
                        ),
                    )
                ),
                WalletToDappInteractionAccountsRequestResponseItem::new(
                    vec![account_1.clone(), account_2.clone()],
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("069ef236486d4cd5706b5e5b168e19f750ffd1b4876529a0a9de966d50a15ab7")
                    .unwrap()),
                    vec![
                        WalletToDappInteractionAccountProof::new(
                            account_1.address,
                            WalletToDappInteractionAuthProof::new(
                                PublicKey::from_str("11b162e3343ce770b6e9ed8a29d125b5580d1272b0dc4e2bd0fcae33320d9566")
                                .unwrap(),
                                Signature::from_str("e18617b527d4d33607a8adb6a040c26ca97642ec89dd8a6fe7a41fa724473e4cc69b0729c1df57aba77455801f2eef6f28848a5d206e3739de29ca2288957502")
                                .unwrap(),
                            ),
                        ),
                        WalletToDappInteractionAccountProof::new(
                            account_2.address,
                            WalletToDappInteractionAuthProof::new(
                                PublicKey::from_str("5386353e4cc27e3d27d064d777d811e242a16ba7aefd425062ed46631739619d")
                                .unwrap(),
                                Signature::from_str("0143fd941d51f531c8265b0f6b24f4cfcdfd24b40aac47dee6fb3386ce0d400563c892e3894a33840d1c7af2dd43ecd0729fd209171003765d109a04d7485605")
                                .unwrap(),
                            ),
                        ),
                    ],
                ),
                persona_data.clone(),
                None,
                None,
                None
            )
        );

        let authorized_request_response =
            WalletToDappInteractionResponse::Success(
                WalletToDappInteractionSuccessResponse::new(
                    WalletInteractionId::from_str(
                        "06f00fbc-67ed-4a22-a122-1da719b25b6f",
                    )
                    .unwrap(),
                    authorized_request_response_items,
                ),
            );

        let unauthorized_request_response_items =
            WalletToDappInteractionResponseItems::UnauthorizedRequest(
                WalletToDappInteractionUnauthorizedRequestResponseItems::new(
                    WalletToDappInteractionAccountsRequestResponseItem::new(
                        vec![account_1.clone()],
                        None,
                        None,
                    ),
                    persona_data.clone(),
                ),
            );

        let unauthorized_request_response =
            WalletToDappInteractionResponse::Success(
                WalletToDappInteractionSuccessResponse::new(
                    WalletInteractionId::from_str(
                        "278608e0-e5ca-416e-8339-f2d2695651c4",
                    )
                    .unwrap(),
                    unauthorized_request_response_items,
                ),
            );

        let failure_response = WalletToDappInteractionResponse::Failure(
            WalletToDappInteractionFailureResponse::new(
                WalletInteractionId::from_str(
                    "278608e0-e5ca-416e-8339-f2d2695651c4",
                )
                .unwrap(),
                DappWalletInteractionErrorType::RejectedByUser,
                "User rejected the request".to_owned(),
            ),
        );

        let transaction_response = WalletToDappInteractionResponse::Success(
            WalletToDappInteractionSuccessResponse::new(
                WalletInteractionId::from_str("c42f8825-4bbb-4ce2-a646-776b529e2f51").unwrap(),
                WalletToDappInteractionResponseItems::Transaction(
                    WalletToDappInteractionTransactionResponseItems::new(
                    TransactionIntentHash::from_str("txid_tdx_2_1mwuvufnewv6qkxdaesx0gcwap7n79knhkn0crsc8dg9g9k7qknjs6vkd3n")
                    .unwrap(),
                ),
            )
        ));

        let account_proof_response_items = WalletToDappInteractionResponseItems::AuthorizedRequest(
            WalletToDappInteractionAuthorizedRequestResponseItems::new(
                WalletToDappInteractionAuthRequestResponseItem::UsePersona(
                    WalletToDappInteractionAuthUsePersonaRequestResponseItem::new(
                        DappWalletInteractionPersona::new(
                            IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva")
                                .unwrap(),
                            "alfz_psf",
                        )
                    )
                ),
                None,
                None,
                None,
                None,
                WalletToDappInteractionProofOfOwnershipRequestResponseItem::new(
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("4c85e4a903ab97450ef83763f8d4ca55a43efe843e1d2ced78a4940e5c397c9c").unwrap()),
                    vec![WalletToDappInteractionProofOfOwnership::Account(WalletToDappInteractionAccountProof::new(
                        AccountAddress::from_str("account_tdx_2_12ytkalad6hfxamsz4a7r8tevz7ahurfj58dlp4phl4nca5hs0hpu90").unwrap(),
                        WalletToDappInteractionAuthProof::new(
                            PublicKey::from_str("ff8aee4c625738e35d837edb11e33b8abe0d6f40849ca1451edaba84d04d0699")
                                .unwrap(),
                            Signature::from_str("10177ac7d486691777133ffe59d46d55529d86cb1c4ce66aa82f432372f33e24d803d8498f42e26fe113c030fce68c526aeacff94334ba5a7f7ef84c2936eb05")
                                .unwrap()
                        ),
                    ))]
                ),
            )
        );

        let account_proof_response = WalletToDappInteractionResponse::Success(
            WalletToDappInteractionSuccessResponse::new(
                WalletInteractionId::from_str(
                    "2916ad16-52a0-4564-a611-4971883c1322",
                )
                .unwrap(),
                account_proof_response_items,
            ),
        );

        let accounts_and_persona_proof_response_items =
        WalletToDappInteractionResponseItems::AuthorizedRequest(
            WalletToDappInteractionAuthorizedRequestResponseItems::new(
                WalletToDappInteractionAuthRequestResponseItem::UsePersona(
                    WalletToDappInteractionAuthUsePersonaRequestResponseItem::new(
                        DappWalletInteractionPersona::new(
                            IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva")
                                .unwrap(),
                            "pao13",
                        )
                    )
                ),
                None,
                None,
                None,
                None,
                WalletToDappInteractionProofOfOwnershipRequestResponseItem::new(
                    DappToWalletInteractionAuthChallengeNonce(Exactly32Bytes::from_hex("e280cfa39e1499f2862e59759cc2fc990cce28b70a7989324fe91c47814d0630").unwrap()),
                    vec![
                        WalletToDappInteractionProofOfOwnership::Account(WalletToDappInteractionAccountProof::new(
                        AccountAddress::from_str("account_tdx_2_12ytkalad6hfxamsz4a7r8tevz7ahurfj58dlp4phl4nca5hs0hpu90").unwrap(), 
                        WalletToDappInteractionAuthProof::new(
                            PublicKey::from_str("ff8aee4c625738e35d837edb11e33b8abe0d6f40849ca1451edaba84d04d0699")
                            .unwrap(),
                            Signature::from_str("10177ac7d486691777133ffe59d46d55529d86cb1c4ce66aa82f432372f33e24d803d8498f42e26fe113c030fce68c526aeacff94334ba5a7f7ef84c2936eb05")
                            .unwrap()
                        ),
                        )),
                        WalletToDappInteractionProofOfOwnership::Persona(WalletToDappInteractionPersonaProof::new(
                            IdentityAddress::from_str("identity_tdx_2_12fat0nh0gymw9j4rqka5344p3h3r86x4z0hkw2v78r03pt0kfv0qva").unwrap(),  
                            WalletToDappInteractionAuthProof::new(
                                PublicKey::from_str("ff8aee4c625738e35d837edb11e33b8abe0d6f40849ca1451edaba84d04d0699")
                                .unwrap(),
                                Signature::from_str("10177ac7d486691777133ffe59d46d55529d86cb1c4ce66aa82f432372f33e24d803d8498f42e26fe113c030fce68c526aeacff94334ba5a7f7ef84c2936eb05")
                                .unwrap()
                            ),
                        )),
                    ]
                ),
            ),
        );

        let accounts_and_persona_proof_response =
            WalletToDappInteractionResponse::Success(
                WalletToDappInteractionSuccessResponse::new(
                    WalletInteractionId::from_str(
                        "17d530f6-0cb6-4122-8540-64e46a2e0f84",
                    )
                    .unwrap(),
                    accounts_and_persona_proof_response_items,
                ),
            );

        let pre_authorization_response_items =
            WalletToDappInteractionResponseItems::PreAuthorization(
                WalletToDappInteractionPreAuthorizationResponseItems {
                    response: WalletToDappInteractionSubintentResponseItem::new(
                        SignedSubintent::sample_other(),
                        Instant::sample(),
                    ),
                },
            );

        let pre_authorization_response =
            WalletToDappInteractionResponse::Success(
                WalletToDappInteractionSuccessResponse::new(
                    WalletInteractionId::from_str(
                        "17d530f6-0cb6-4122-8540-64e46a2e0f84",
                    )
                    .unwrap(),
                    pre_authorization_response_items,
                ),
            );

        let responses = vec![
            authorized_request_response,
            unauthorized_request_response,
            failure_response,
            transaction_response,
            account_proof_response,
            accounts_and_persona_proof_response,
            pre_authorization_response,
        ];

        let encoded = serde_json::to_string(&responses).unwrap();
        let serde_value: Vec<Value> = serde_json::from_str(&encoded).unwrap();
        let fixture = fixture::<Vec<Value>>(include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "wallet_interactions_wallet_to_dapp.json"
        )))
        .expect("wallet_interactions_wallet_to_dapp fixture");

        for (serde_value, fixture) in serde_value.iter().zip(fixture.iter()) {
            pretty_assertions::assert_eq!(serde_value, fixture);
        }
    }
}
