use core::fmt::Debug;
use sargon::prelude::*;
use serde::Deserialize;
use std::str::FromStr;

#[cfg(test)]
mod profile_snapshot_tests {
    use prelude::fixture_profiles;

    use super::*;

    /// We cannot do a roundtrip test here because
    /// `only_plaintext_profile_snapshot_version_100.json` does contain `p2pLinks`
    /// and with the [CAP-36][doc] feature `p2pLinks` are no longer stored in `Profile`.
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3251863610/CAP-36+WebRTC+Clients+Protocol
    #[test]
    fn v100_100() {
        fixture_and_json::<Profile>(fixture_profiles!(
            "only_plaintext_profile_snapshot_version_100"
        ))
        .expect("V100 Profile to deserialize");
    }
}

#[cfg(test)]
mod cap26_tests {

    use prelude::fixture_vector;

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
        let secp256k1 =
            fixture::<CAP26Group>(fixture_vector!("cap26_secp256k1"))
                .expect("CAP26 Secp256k1 vectors");
        let curve25519 =
            fixture::<CAP26Group>(fixture_vector!("cap26_curve25519"))
                .expect("CAP26 Curve25519 vectors");

        secp256k1.test::<Secp256k1PrivateKey, Secp256k1PublicKey>();
        curve25519.test::<Ed25519PrivateKey, Ed25519PublicKey>();
    }

    // ========= SECURIFIED KEY SPACE =========
    // Higher part of the 2^31-1 key space
    // These values have been cross references
    // using this Python script:
    // https://gist.github.com/Sajjon/060c5747c6ffead12f78645b623a8164
    // Which is based on the SLIP10 reference implementation:
    // https://github.com/satoshilabs/slips/blob/master/slip-0010/testvectors.py
    // ========================================
    fn test(
        mnemonic: Mnemonic,
        passphrase: impl AsRef<str>,
        network_id: NetworkID,
        index_unhardened: u32,
        path_canonical_notation: impl AsRef<str>,
        path_securified_notation: Option<&'static str>,
        private_key_hex: impl AsRef<str>,
        public_key_hex: impl AsRef<str>,
        factor_source_id_str: impl AsRef<str>,
        address: impl AsRef<str>,
    ) {
        let index_hardened = index_unhardened | 0x8000_0000;
        let path_canonical_notation = path_canonical_notation.as_ref();
        let account_path = AccountPath::new(
            network_id,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_global_key_space(index_hardened).unwrap(),
        );

        // Test display
        let derivation_path = DerivationPath::from(account_path.clone());
        pretty_assertions::assert_eq!(
            derivation_path.to_bip32_string(),
            path_canonical_notation
        );
        if let Some(path_securified_notation) = path_securified_notation {
            pretty_assertions::assert_eq!(
                derivation_path.to_cap43_string(),
                path_securified_notation
            );
            pretty_assertions::assert_eq!(
                path_securified_notation.parse::<AccountPath>().unwrap(),
                account_path
            ); // test FromStr
        } else {
            pretty_assertions::assert_eq!(
                path_canonical_notation.parse::<AccountPath>().unwrap(),
                account_path
            ); // test FromStr
        }

        let private_key = mnemonic
            .to_seed(passphrase.as_ref())
            .derive_private_key(&account_path);
        let public_key = private_key.public_key();
        pretty_assertions::assert_eq!(
            private_key.to_hex(),
            private_key_hex.as_ref()
        );
        pretty_assertions::assert_eq!(
            public_key.to_hex(),
            public_key_hex.as_ref()
        );
        let account_address = AccountAddress::new_from_public_key(
            public_key.public_key,
            network_id,
        );
        let factor_source_id = FactorSourceIDFromHash::new_for_device(
            &MnemonicWithPassphrase::new(mnemonic),
        );
        pretty_assertions::assert_eq!(
            factor_source_id.to_string(),
            format!("device:{}", factor_source_id_str.as_ref())
        );
        pretty_assertions::assert_eq!(
            account_address.to_string(),
            address.as_ref()
        );
        pretty_assertions::assert_eq!(account_address.network_id(), network_id);
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_0() {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            None,
            "5b82120ec4763f8bacff71c8e529894fea1e735a5698ff400364a913f7b20c00",
            "f3d0210f6c2cecbdc977b7aae19d468a6c363e73a055bc877248f8318f0122e8",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12xek3geay25lktmk5zyplc7z7mg5xe8ldh48ta4mkcd9q0v0q6l8y6",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_1() {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            None,
            "da5e924c716a05b616940dd7828e3020de4dc09c371ab03966e00e95c68cb439",
            "df49129a10aa88c76837611a4ecda794ac5f650e4401037e1ff275e52bc784c5",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128mh2ae9dsrwa0t8l37ayjrxxf0p84e6qm227ytxtcu447f5uw5m8w",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_3_hardened(
    ) {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 3) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741821H",
            None,
            "d9e0394b67affb91b5acdc3ecf6786a6628892ffd605291c853568cbed498afa",
            "a484112bcd119488f13191a6ec57ff27606ea041537662730e60580cdb679616",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12yxslky3ye2rdtcv439s7l8hw2pm7sp6g3e537dsuk6558z66yteu5",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_2_hardened(
    ) {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 2) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741822H",
            None,
            "5218159039d5c639ae4e0b6b351b821e3687aa44768230c4f06a13ae0c78715c",
            "2155707a3cebd7788dc83113174d30e2c29abae34f399c27a6caa8c6f5ae543e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx1299mrqwvhy6cka9vsvjddqhttm9qckk08w32kp6nrnzrwaclqelp4x",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_1_hardened(
    ) {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 1) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741823H",
            None,
            "5c5adfebe650684e3cc20e4dba49e1447d7ac12f63ae1bd8723554d0a95aaf38",
            "f4f43daaedc3603b3dc6b92a2014630a96ca2a20cc14d2dcaa71f49c30789689",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx129zmjv05ljhm3tc3f5nayvfgym69fu6zlajt6xp2jj900c5qt76m6v",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_hardened()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30),
            "m/44H/1022H/1H/525H/1460H/1073741824H",
            Some("m/44H/1022H/1H/525H/1460H/0S"),
            "b0b9180f7c96778cffba7af2ef1ddf4705fca21b965e8a722ccf2ec403c35950",
            "e0293d4979bc303ea4fe361a62baf9c060c7d90267972b05c61eead9ef3eed3e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128znphf3gxek50qyxjcuels6xtulum3g46vhr43ryavj7zr53xxded",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_plus_1_hardened(
    ) {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30) + 1,
            "m/44H/1022H/1H/525H/1460H/1073741825H",
            Some("m/44H/1022H/1H/525H/1460H/1S"), 
            "c1880587c727f2f01dfdf61d19b44283d311b31c12e8898b774b73e8067d25b1",
            "c6aaee6fa60d73a17989ce2a2a5db5a88cd696aef61d2f298262fae189dff04e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12y8gd9dyz9mhg3jv5p9md5gvuzc34m0p90te0hx7aqgsvuy5g2p09s",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_plus_2_hardened(
    ) {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30) + 2,
            "m/44H/1022H/1H/525H/1460H/1073741826H", 
            Some("m/44H/1022H/1H/525H/1460H/2S"),
            "837bc77bb29e4702be39c69fbade7d350bc23f6daddf68a64474984e899a97a3",
            "6a92b3338dc74a50e8b3fff896a7e0f43c42742544af52de20353675d8bc7907",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12xluhgaw3vcyskpsmswu279jlysmrdjuk23erjcx8s83kcgx3r4zvn",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_5()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(31u32) - 5,
            "m/44H/1022H/1H/525H/1460H/2147483643H",
            Some("m/44H/1022H/1H/525H/1460H/1073741819S"),
            "8371cdce66f0733cf1f8a07235825267e8e650f9bf194dfe82992c8ae77faa84",
            "9bce7e1a1d724b2013add0697e4133e2affc93b806793ee6709dfdc242738e19",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12x0z0sm5qpp9gmuah7nnpkkkk2zn2r8tvpd9w64097949mcs7jm960",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_4()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(31u32) - 4,
            "m/44H/1022H/1H/525H/1460H/2147483644H",
            Some("m/44H/1022H/1H/525H/1460H/1073741820S"), 
            "361126bd7947254c49b83c23bbb557219cfa2ac5e5a4551501f18236ffa4eb17",
            "481b737f5baaf52520612e70858ffa72a3624d5a050da5748844ac14036c8b17",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12y27yrwuqmec5saaykp82098nykpeqentzt3syt4dfdyuq0ckkc07u",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_3()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(31u32) - 3,
            "m/44H/1022H/1H/525H/1460H/2147483645H", 
            Some("m/44H/1022H/1H/525H/1460H/1073741821S"), 
            "f63fe429c5723448dfb8d1f3eda88a659473b4c38960a09bb20efe546fac95ee",
            "b2819057da648f36eadb59f60b732d4ae7fb22a207acf214e0271d3c587afd54",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12x9mszdtxacj5trw78g2ndvc54wtxg9mxx982w2p8vnv7jes7nvc40",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_2()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(31u32) - 2,
            "m/44H/1022H/1H/525H/1460H/2147483646H", 
            Some("m/44H/1022H/1H/525H/1460H/1073741822S"), 
            "5a8b6327942ca8fc5b30fb5b0c1fa53e97362d514ff4f2c281060b9d51f7fc88",
            "932123e6c46af8ebde7a96bee4563e09bbf41b28eae9d6ba1c667a2f490a1fcf",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12ysf5nesz5h3wk8aypyn83e9752mal8q545epwykq6nr8k8aavyu7d",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_1()
    {
        test(
            Mnemonic::sample_device_other(),
            "",
            NetworkID::Mainnet,
            2u32.pow(31u32) - 1,
            "m/44H/1022H/1H/525H/1460H/2147483647H", 
            Some("m/44H/1022H/1H/525H/1460H/1073741823S"),
            "7eae6f235206329561b09fc2235d35e017c3f28b54fd3b4f6525e601257c4ce7",
            "87a2f84f826da0c62052fbe7b385ab78883c02d1fa5472c55a06aa529a0701e9",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128258pxhges8rmva0a2egr0tzqd8x8clsl5d90a8qv3zqggc4jr2ss",
        );
    }
}

#[cfg(test)]
mod bip44_tests {
    use prelude::fixture_vector;

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
        let fixture = fixture::<Fixture>(fixture_vector!("bip44_secp256k1"))
            .expect("BIP44 fixture");

        fixture.test();
    }
}

#[cfg(test)]
mod slip10_tests {
    use prelude::fixture_vector;

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
        let ten = fixture::<Fixture>(fixture_vector!("slip10_tests_#10"))
            .expect("SLIP10 #10 fixture");
        let thousand =
            fixture::<Fixture>(fixture_vector!("slip10_tests_#1000"))
                .expect("SLIP10 #1000 fixture");

        ten.test();
        thousand.test();
    }
}

mod encrypted_profile_tests {
    use std::collections::HashSet;

    use prelude::fixture_profiles;
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
                        .map(|x| x.security_state.clone())
                        .for_each(test);

                    n.personas
                        .into_iter()
                        .map(|x| x.security_state.clone())
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
        let fixture = fixture::<Fixture>(fixture_profiles!(
            "multi_profile_snapshots_test_version_100_patch_after_app_version_120"
        ))
        .expect("Encrypted Profile tests");

        fixture.test();
    }
}

#[cfg(test)]
mod dapp_to_wallet_interaction_tests {
    use super::*;
    use prelude::fixture_interaction;
    use serde_json::Value;

    #[test]
    fn test_vector() {
        let decoded_wallet_interactions =
            fixture::<Vec<DappToWalletInteraction>>(fixture_interaction!(
                "wallet_interactions_dapp_to_wallet"
            ))
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

        let raw_wallet_interactions = fixture::<Vec<Value>>(
            fixture_interaction!("wallet_interactions_dapp_to_wallet"),
        )
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

    use prelude::fixture_interaction;
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
        let fixture = fixture::<Vec<Value>>(fixture_interaction!(
            "wallet_interactions_wallet_to_dapp"
        ))
        .expect("wallet_interactions_wallet_to_dapp fixture");

        for (serde_value, fixture) in serde_value.iter().zip(fixture.iter()) {
            pretty_assertions::assert_eq!(serde_value, fixture);
        }
    }
}
