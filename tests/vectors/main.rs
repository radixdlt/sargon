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
                    let derived = seed.derive_private_key(&v.path);
                    assert_eq!(
                        derived.to_hex(),
                        format!("{:?}", private_key)
                    );
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
                    let derived_private_key =
                        seed.derive_private_key(&v.path);
                    assert_eq!(derived_private_key.private_key, PrivateKey::from(expected_private_key));
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
            struct TestDerivation {
                let curve: 
            }
            impl Derivation for TestDerivation {
                /*
                    fn derivation_path(&self) -> DerivationPath;
    fn hd_path(&self) -> &HDPath;
    fn scheme(&self) -> DerivationPathScheme;
                */
            }
            let maybe_derived: Option<sargon::PrivateKey> =
                match self.curve.as_str() {
                    "ed25519" => Some(
                        seed::derive_private_key(
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
            let entropy = ::hex::decode(&self.entropy).unwrap();
            assert_eq!(self.mnemonic, Mnemonic::from_entropy(&entropy));
            assert_eq!(::hex::encode(*seed), self.seed);
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
