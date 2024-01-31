use crate::prelude::*;

/// The address of an Account, a bech32 encoding of a public key hash
/// that starts with the prefix `"account_"`, dependent on NetworkID, meaning the same
/// public key used for two AccountAddresses on two different networks will not have
/// the same address.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{address}")]
pub struct AccountAddress {
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
    pub address: String,

    /// The network this account address is tied to, i.e. which was used when a public key
    /// hash was used to bech32 encode it. This means that two public key hashes will result
    /// in two different account address on two different networks.
    pub network_id: NetworkID,
}

#[uniffi::export]
pub fn new_account_address(bech32: String) -> Result<AccountAddress> {
    AccountAddress::try_from_bech32(bech32.as_str())
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
        let suffix = suffix_str(6, &self.address);
        format!("{}...{}", &self.address[0..4], suffix)
    }
}

impl FromStr for AccountAddress {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccountAddress::try_from_bech32(s)
    }
}

impl EntityAddress for AccountAddress {
    /// Identifies that AccountAddress uses the `EntityType::Account`, which are used
    /// to validate the HRP (`"account_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn entity_type() -> AbstractEntityType {
        AbstractEntityType::Account
    }

    // Underscored to decrease visibility. You SHOULD NOT call this function directly,
    // instead use `try_from_bech32` which performs proper validation. Impl types SHOULD
    // `panic` if `address` does not start with `Self::entity_type().hrp()`
    fn __with_address_and_network_id(
        address: &str,
        network_id: NetworkID,
    ) -> Self {
        assert!(address.starts_with(&Self::entity_type().hrp()), "Invalid address, you SHOULD NOT call this function directly, you should use `try_from_bech32` instead.");
        Self {
            address: address.to_string(),
            network_id,
        }
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
        assert_eq!(address.network_id, NetworkID::Mainnet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
            "account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master",
        )
        .unwrap();
        assert_eq!(address.network_id, NetworkID::Mainnet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
            )
            .unwrap();
        assert_eq!(address.network_id, NetworkID::Stokenet);
        address
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_other() -> Self {
        let address = AccountAddress::try_from_bech32(
                "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr",
            )
            .unwrap();
        assert_eq!(address.network_id, NetworkID::Stokenet);
        address
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            AccountAddress::placeholder(),
            AccountAddress::placeholder()
        );
        assert_eq!(
            AccountAddress::placeholder_other(),
            AccountAddress::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            AccountAddress::placeholder(),
            AccountAddress::placeholder_other()
        );
    }

    #[test]
    fn try_from_bech32() {
        assert!(AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .is_ok());
    }

    #[test]
    fn from_bech32_invalid_entity_type() {
        assert_eq!(
            AccountAddress::try_from_bech32(
                "identity_tdx_21_12tljxea3s0mse52jmpvsphr0haqs86sung8d3qlhr763nxttj59650",
            ),
            Err(CommonError::MismatchingEntityTypeWhileDecodingAddress)
        );
    }

    #[test]
    fn format() {
        let a = AccountAddress::try_from_bech32(
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
            AccountAddress::from_public_key::<PublicKey>(public_key.into(), NetworkID::Mainnet)
                .address,
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
            AccountAddress::new(public_key.into(), NetworkID::Mainnet).address,
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        )
    }

    #[test]
    fn nebunet() {
        let address = AccountAddress::try_from_bech32(
            "account_tdx_b_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08m9raqq",
        )
        .unwrap();
        assert_eq!(address.network_id, NetworkID::Nebunet)
    }

    #[test]
    fn network_id() {
        let sut = AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(sut.network_id, NetworkID::Mainnet);
    }

    #[test]
    fn short() {
        let sut: AccountAddress = AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(sut.short(), "acco...please");
    }

    #[test]
    fn invalid() {
        assert_eq!(
            AccountAddress::try_from_bech32("x"),
            Err(CommonError::FailedToDecodeAddressFromBech32("x".to_owned()))
        )
    }

    #[test]
    fn invalid_checksum() {
        let s = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3apleasx";
        assert_eq!(
            AccountAddress::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32(s.to_owned()))
        )
    }

    #[test]
    fn invalid_entity_type() {
        let s = "identity_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease";
        assert_eq!(
            AccountAddress::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32(s.to_owned()))
        )
    }

    #[test]
    fn json_roundtrip() {
        let a: AccountAddress =
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
        assert_json_value_fails::<AccountAddress>(
            json!("identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp")
        );
        assert_json_value_fails::<AccountAddress>(
            json!("account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzzz")
        );
        assert_json_value_fails::<AccountAddress>(json!("super invalid"));
    }
}

#[cfg(test)]
mod uniffi_tests {
    use std::str::FromStr;

    use crate::{
        account_address_to_short, new_account_address,
        new_account_address_from, Ed25519PublicKey, EntityAddress, NetworkID,
        PublicKey,
    };

    use super::AccountAddress;

    #[test]
    fn short() {
        let sut: AccountAddress = AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .unwrap();
        assert_eq!(account_address_to_short(&sut), "acco...please");
    }

    #[test]
    fn new() {
        let public_key: PublicKey = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap()
        .into();

        let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm";
        assert_eq!(
            AccountAddress::new(public_key.clone(), NetworkID::Mainnet),
            new_account_address_from(public_key, NetworkID::Mainnet)
        );
        let from_bech32 = new_account_address(bech32.to_string()).unwrap();
        assert_eq!(
            AccountAddress::try_from_bech32(bech32).unwrap(),
            from_bech32.clone()
        );
        assert_eq!(from_bech32.address, bech32)
    }
}
