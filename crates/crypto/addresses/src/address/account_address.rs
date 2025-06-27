pub use crate::prelude::*;

decl_address!(
    /// Human readable address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...nvjdwr`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Account* addresses starts with
    /// the prefix `account_`.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of AccountAddresses:
    /// * GlobalAccount
    /// * GlobalVirtualSecp256k1Account
    /// * GlobalVirtualEd25519Account
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccountAddress`][ret]
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L224-L228
    account => [
        ScryptoEntityType::GlobalAccount,
        ScryptoEntityType::GlobalPreallocatedSecp256k1Account,
        ScryptoEntityType::GlobalPreallocatedEd25519Account
    ]
);

pub type DappDefinitionAddress = AccountAddress;

impl AccountAddress {
    pub fn new_from_public_key(
        public_key: impl Into<PublicKey>,
        network_id: NetworkID,
    ) -> Self {
        <Self as IsEntityAddress>::from_public_key(
            public_key.into(),
            network_id,
        )
    }

    /// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
    /// for all accounts created by the Babylon Radix Wallets.
    /// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
    /// imported from the legacy Olympia desktop application.
    pub fn is_legacy_address(&self) -> bool {
        self.entity_type()
            == ScryptoEntityType::GlobalPreallocatedSecp256k1Account
    }
}
impl HasEntityKind for AccountAddress {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}
impl IsBaseEntityAddress for AccountAddress {}
impl IsEntityAddress for AccountAddress {}

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

impl From<AccountAddress> for ScryptoComponentAddress {
    fn from(value: AccountAddress) -> ScryptoComponentAddress {
        ScryptoComponentAddress::new_or_panic(value.node_id().0)
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
                "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
            )
            .unwrap();
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp",
            )
            .unwrap();
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }

    pub fn sample_grace() -> Self {
        Self::from_str("account_rdx128c4f8dnuvd73d2r3fl95ryfuavw5zjf8zr57hjw0qjagz7s7grace").unwrap()
    }

    pub fn sample_frank() -> Self {
        Self::from_str("account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank").unwrap()
    }

    pub fn sample_judy() -> Self {
        // Or alternatively: `"account_rdx12xc095hk3g8klf8gvz5q6qy9pl9e7v5m5eju0xctzvesnyhnfdjudy"`
        // Or alternatively: `"account_rdx12x82w84l6n55m78h8eldedeca52tp8tpuu3342g5lw4f9v85znjudy"`
        Self::from_str("account_rdx12y0389ew2xn7w02d059hhye6t0mjzqxqyavsetyg2j3p3xqyepjudy").unwrap()
    }

    pub fn sample_oscar() -> Self {
        Self::from_str("account_rdx129uc6rf5vmkj2gau7fgxlsqdg8008nca8yd57sxx4v67dyw7u0scar").unwrap()
    }

    pub fn sample_trudy() -> Self {
        Self::from_str("account_rdx1284z0gpg9vnhevn7sytdncszc7ukcrycntg7zjktqvggmwe6ctrudy").unwrap()
    }
    pub fn sample_radix() -> Self {
        // Or alternatively: `"account_rdx12yql52whel8xjttrw84tau270asj92ssu9pgqzgrftd4m8ptm8zrdx"`
        // Or alternatively: `"account_rdx129sctf9rusf0ceq6ap2ca8r030f2mf8z0a2fx90jg5yxtladqrprdx"`
        // Or alternatively: `"account_rdx1287mp2d20wfmc8tyluryehz3j53jn0f9jmkdxp9808vrjxetu9prdx"`
        // Or alternatively: `"account_rdx1282v25pw9y59ny74rv7aht6q0qgqs4g85q20zmustkyu6mxphp5rdx"`
        // Or alternatively: `"account_rdx128avae2px32e8t7vcax0axvt0afa5lrw4rwjlksev48wamgs472rdx"`
        // Or alternatively: `"account_rdx128g9urn56vyu2axptn9r4rctggn9f0phlu3mzd6mz6lsdkxkd7hrdx"`
        // Or alternatively: `"account_rdx129mvy35wx370ey2sxqces5ddq5sqj4q7xxhwl3ejpvu8gd8ta3erdx"`
        Self::from_str("account_rdx12y7uww27s250g9d3d72ey9wdp5z78zpmq5la0r0wgw4fkf6y8eerdx").unwrap()
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
        let address =
            AccountAddress::new_from_public_key(public_key, network_id);

        assert_eq!(address.address(), "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw");

        let s = "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw";
        let from_str = AccountAddress::from_str(s).unwrap();
        assert_eq!(from_str.network_id(), NetworkID::Zabanet); // Zabanet (0x0e / 0d14)
        assert_eq!(from_str.to_string(), s);
        let from_json: AccountAddress =
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
            Err(CommonError::AddressInvalidEntityType { address_kind: "Account".to_string(), entity_type: ScryptoEntityType::GlobalPreallocatedEd25519Identity as u8, node_id_as_hex: "52ff2367b183f70cd152d85900dc6fbf4103ea1c9a0ed883f71fb519996b".to_string() })
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
            SUT::new_from_public_key(public_key, NetworkID::Mainnet).address(),
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
    fn invalid_got_olympia_address() {
        let s =
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToFindNetworkIdFromBech32mString { bech32m_encoded_address: "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842".to_string() })
        )
    }

    #[test]
    fn into_scrypto_global_address() {
        assert_eq!(
            ScryptoGlobalAddress::from(SUT::sample())
                .into_node_id()
                .as_bytes()[0],
            ScryptoEntityType::GlobalPreallocatedEd25519Account as u8
        );
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
