pub use crate::prelude::*;

#[uniffi::export]
pub fn new_account_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> AccountAddress {
    AccountAddress::new(public_key, network_id)
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet() -> AccountAddress {
    AccountAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet_other() -> AccountAddress {
    AccountAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet() -> AccountAddress {
    AccountAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet_other() -> AccountAddress {
    AccountAddress::sample_stokenet_other()
}

/// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
/// for all accounts created by the Babylon Radix Wallets.
/// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
/// imported from the Olympia Wallet.
#[uniffi::export]
pub fn account_address_is_legacy(address: &AccountAddress) -> bool {
    address.is_legacy_address()
}

impl AccountAddress {
    pub fn new(
        public_key: impl Into<PublicKey>,
        network_id: NetworkID,
    ) -> Self {
        <Self as EntityAddress>::from_public_key(public_key.into(), network_id)
    }

    /// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
    /// for all accounts created by the Babylon Radix Wallets.
    /// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
    /// imported from the Olympia Wallet.
    pub fn is_legacy_address(&self) -> bool {
        self.entity_type() == ScryptoEntityType::GlobalVirtualSecp256k1Account
    }
}

impl EntityAddress for AccountAddress {
    /// Identifies that AccountAddress uses the `EntityType::Account`, which are used
    /// to validate the HRP (`"account_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn abstract_entity_type() -> AbstractEntityType {
        AbstractEntityType::Account
    }
}

impl HasSampleValues for AccountAddress {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl AccountAddress {
    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        let address = AccountAddress::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
        )
        .unwrap();
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264",
        )
        .unwrap();
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
            )
            .unwrap();
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr",
            )
            .unwrap();
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn is_legacy_address() {
        assert!(SUT::from_str("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease").unwrap().is_legacy_address());
        assert!(!SUT::sample_stokenet().is_legacy_address());
    }

    #[test]
    fn zabanet() {
        let public_key: Ed25519PublicKey =
            "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
                .parse()
                .unwrap();
        let network_id = NetworkID::Zabanet;
        let address = AccountAddress::new(
            PublicKey::Ed25519 { value: public_key },
            network_id,
        );

        assert_eq!(address.address(), "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw");

        use radix_engine_toolkit::models::canonical_address_types::{
            CanonicalAccountAddress, CanonicalAddress,
        };
        let s = "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw";
        let from_str = CanonicalAccountAddress::from_str(s).unwrap();
        assert_eq!(from_str.network_id(), 0xe); // Zabanet (0x0e / 0d14)
        assert_eq!(from_str.to_string(), s);
        let from_json: CanonicalAccountAddress =
            serde_json::from_value(json!(s)).unwrap();
        assert_eq!(from_json.to_string(), s);
    }

    #[test]
    fn try_from_bech32() {
        assert!(SUT::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
        )
        .is_ok());
    }

    #[test]
    fn from_bech32_invalid_entity_type() {
        let s = "identity_tdx_21_12tljxea3s0mse52jmpvsphr0haqs86sung8d3qlhr763nxttj59650";
        assert_eq!(
            SUT::try_from_bech32(s,),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        );
    }

    #[test]
    fn display() {
        let a = SUT::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
        )
        .unwrap();
        assert_eq!(
            format!("{}", a),
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
        );
    }

    #[test]
    fn debug() {
        let a = SUT::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
        )
        .unwrap();
        assert_eq!(
            format!("{:?}", a),
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
        );
    }

    #[test]
    fn from_public_key_bytes_and_network_id() {
        let public_key = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap();

        assert_eq!(
            SUT::from_public_key::<PublicKey>(public_key.into(), NetworkID::Mainnet)
                .address(),
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        )
    }

    #[test]
    fn from_ed25519_public_key_bytes() {
        let public_key = Ed25519PublicKey::from_str(
            "48d24f09b43d50f3acd58cf8509a57c8f306d94b945bd9b7e6ebcf6691eed3b6",
        )
        .unwrap();

        assert_eq!(
            SUT::from_public_key::<PublicKey>(
                public_key.into(),
                NetworkID::Mainnet
            ),
            SUT::sample()
        )
    }

    #[test]
    fn new() {
        let public_key = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap();

        assert_eq!(
            SUT::new(public_key.into(), NetworkID::Mainnet).address(),
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        )
    }

    #[test]
    fn nebunet() {
        let address = SUT::try_from_bech32(
            "account_tdx_b_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08m9raqq",
        )
        .unwrap();
        assert_eq!(address.network_id(), NetworkID::Nebunet)
    }

    #[test]
    fn network_id() {
        let sut = SUT::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
        )
        .unwrap();
        assert_eq!(sut.network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn formatted_full() {
        assert_eq!(SUT::sample().formatted(AddressFormat::Full), "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr");
    }

    #[test]
    fn formatted_raw() {
        assert_eq!(SUT::sample().formatted(AddressFormat::Raw), "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr");
    }

    #[test]
    fn formatted_default() {
        assert_eq!(
            SUT::sample().formatted(AddressFormat::Default),
            "acco...nvjdwr"
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            SUT::try_from_bech32("x"),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: "x".to_owned()
            })
        )
    }

    #[test]
    fn invalid_checksum() {
        let s = "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdxx";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn invalid_entity_type() {
        let s = "identity_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn invalid_got_olympia_address() {
        let s =
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn into_scrypto_global_address() {
        assert_eq!(
            radix_engine::types::GlobalAddress::from(SUT::sample())
                .into_node_id()
                .as_bytes()[0],
            radix_engine_common::types::EntityType::GlobalVirtualEd25519Account
                as u8
        );
    }

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        type RetAddr = <SUT as FromRetAddress>::RetAddress;
        let sut = SUT::sample();
        let bech32 = sut.to_string();
        let ret = RetAddr::try_from_bech32(&bech32).unwrap();

        let ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
        assert_eq!(ffi_side, bech32);
        let from_ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();
        assert_eq!(ret, from_ffi_side);
    }

    #[test]
    fn manual_perform_uniffi_conversion_fail() {
        type RetAddr = <SUT as FromRetAddress>::RetAddress;
        assert!(<RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
            "invalid".to_string()
        )
        .is_err());
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzzz")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264";
        let address = new_account_address(b32.to_owned()).unwrap();
        assert_eq!(account_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(account_address_bech32_address(&address), b32);
    }

    #[test]
    fn address_format_default() {
        let sut: SUT = SUT::try_from_bech32(
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264",
        )
        .unwrap();
        assert_eq!(
            account_address_formatted(&sut, AddressFormat::Default),
            "acco...aer264"
        );
    }

    #[test]
    fn new_from_key() {
        let public_key: PublicKey = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap()
        .into();

        let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm";
        assert_eq!(
            SUT::new(public_key, NetworkID::Mainnet),
            new_account_address_from(public_key, NetworkID::Mainnet)
        );
        let from_bech32 = new_account_address(bech32.to_string()).unwrap();
        assert_eq!(SUT::try_from_bech32(bech32).unwrap(), from_bech32.clone());
        assert_eq!(from_bech32.address(), bech32)
    }

    #[test]
    fn is_legacy_address() {
        assert!(account_address_is_legacy(&SUT::from_str("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease").unwrap()));
        assert!(!account_address_is_legacy(&SUT::sample_stokenet()));
    }

    #[test]
    fn sample() {
        assert_eq!(new_account_address_sample_mainnet(), SUT::sample_mainnet());

        assert_eq!(
            new_account_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_account_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_account_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
