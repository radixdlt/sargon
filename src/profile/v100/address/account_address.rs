pub use crate::prelude::*;
// use crate::InnerAccountAddress;

/// Human readable address of an account. Always starts with `"account_"``, for example:
///
/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
///
/// Most commonly the user will see this address in its abbreviated
/// form which is:
///
/// `acco...please`
///
/// Addresses are checksummed, as per Bech32. **Only** *Account* addresses starts with
/// the prefix `account_`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    SerializeDisplay,
    DeserializeFromStr,
    uniffi::Record,
)]
#[display("{__inner}")]
pub struct AccountAddress {
    pub(crate) __inner: InnerAccountAddress,
}

#[uniffi::export]
pub fn new_account_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> AccountAddress {
    AccountAddress::new(public_key, network_id)
}

/// Formats the AccountAddress to its abbreviated form which is what the user
/// is most used to, since it is what we most commonly display in the Radix
/// ecosystem.
///
/// The abbreviated form returns:
///
/// `acco...please`
///
/// For the account address:
///
/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
///
#[uniffi::export]
pub fn account_address_to_short(address: &AccountAddress) -> String {
    address.short()
}

impl AccountAddress {
    pub fn new(public_key: PublicKey, network_id: NetworkID) -> Self {
        <Self as EntityAddress>::from_public_key(public_key, network_id)
    }

    /// Formats the AccountAddress to its abbreviated form which is what the user
    /// is most used to, since it is what we most commonly display in the Radix
    /// ecosystem.
    ///
    /// The abbreviated form returns:
    ///
    /// `acco...please`
    ///
    /// For the account address:
    ///
    /// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
    ///
    pub fn short(&self) -> String {
        let suffix = suffix_str(6, &self.address());
        format!("{}...{}", &self.address()[0..4], suffix)
    }
}

impl EntityAddress for AccountAddress {
    /// Identifies that AccountAddress uses the `EntityType::Account`, which are used
    /// to validate the HRP (`"account_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn entity_type() -> AbstractEntityType {
        AbstractEntityType::Account
    }
}

impl HasPlaceholder for AccountAddress {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}

impl AccountAddress {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        let address = AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
            "account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master",
        )
        .unwrap();
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
            )
            .unwrap();
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_other() -> Self {
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

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn try_from_bech32() {
        assert!(SUT::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
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
    fn format() {
        let a = SUT::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(
            format!("{}", a),
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
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
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(sut.network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn short() {
        let sut: SUT = SUT::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(sut.short(), "acco...please");
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
        let s = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3apleasx";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn invalid_entity_type() {
        let s = "identity_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"),
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
        let b32 = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease";
        let address = new_account_address(b32.to_owned()).unwrap();
        assert_eq!(account_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(account_address_bech32_address(&address), b32);
    }

    #[test]
    fn short() {
        let sut: SUT = SUT::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(account_address_to_short(&sut), "acco...please");
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
            SUT::new(public_key.clone(), NetworkID::Mainnet),
            new_account_address_from(public_key, NetworkID::Mainnet)
        );
        let from_bech32 = new_account_address(bech32.to_string()).unwrap();
        assert_eq!(SUT::try_from_bech32(bech32).unwrap(), from_bech32.clone());
        assert_eq!(from_bech32.address(), bech32)
    }
}
