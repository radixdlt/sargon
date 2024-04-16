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

struct TestDerivation {
    curve: SLIP10Curve,
    hd_path: HDPath,
    derivation_path: DerivationPath,
}
#[cfg(not(tarpaulin_include))]
impl Derivation for TestDerivation {
    fn curve(&self) -> SLIP10Curve {
        self.curve
    }
    fn hd_path(&self) -> &HDPath {
        &self.hd_path
    }
    fn derivation_path(&self) -> DerivationPath {
        self.derivation_path.clone()
    }
}

#[cfg(test)]
mod cap26_tests {

    use super::*;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct CAP26Vector {
        path: CAP26Path,

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

                    // A liiittle bit hacky, but this allows us to test CAP26 paths with Secp256k1,
                    // which we have test vectors for - but which we actually do not allow in
                    // the wallets. So we force say "No, dont use Ed25519 curve for these CAP26 paths, actu
                    // use Secp256k1 instead!"
                    let derivation = TestDerivation {
                        curve: S::curve(), // will be `Secp256k1` for `cap26_secp256k1.json` vectors!
                        hd_path: v.path.hd_path().clone(),
                        derivation_path: DerivationPath::CAP26 {
                            value: v.path.clone(),
                        },
                    };

                    let derived = seed.derive_private_key(&derivation);
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

            let derivation = TestDerivation {
                curve,
                hd_path: path.clone(),
                derivation_path: // no used by the test, unable to express non Radix BIP44 paths which this test uses...
                DerivationPath::CAP26 {
                    value: CAP26Path::GetID {
                        value: GetIDPath::default(),
                    },
                }
            };
            let derived = seed.derive_private_key(&derivation);
            assert_eq!(derived.private_key.to_hex(), self.private_key);
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

#[cfg(test)]
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

                    Ok(())
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
