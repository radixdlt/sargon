use super::{entity_address::EntityAddress, entity_type::EntityType, network_id::NetworkID};
use crate::utils::string_utils::suffix_string;
use serde::{de, Deserializer, Serialize, Serializer};
use std::fmt::Display;

/// The address of an Account, a bech32 encoding of a public key hash
/// that starts with the prefix `"account_"`, dependent on NetworkID, meaning the same
/// public key used for two AccountAddresses on two different networks will not have
/// the same address.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Serialize for AccountAddress {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.address)
    }
}

impl<'de> serde::Deserialize<'de> for AccountAddress {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<AccountAddress, D::Error> {
        let s = String::deserialize(d)?;
        AccountAddress::try_from_bech32(&s).map_err(de::Error::custom)
    }
}

impl AccountAddress {
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
        let suffix = suffix_string(6, &self.address);
        format!("{}...{}", &self.address[0..4], suffix)
    }
}

impl EntityAddress for AccountAddress {
    /// Identifies that AccountAddress uses the `EntityType::Account`, which are used
    /// to validate the HRP (`"account_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn entity_type() -> EntityType {
        EntityType::Account
    }

    // Underscored to decrease visibility. You SHOULD NOT call this function directly,
    // instead use `try_from_bech32` which performs proper validation. Impl types SHOULD
    // `panic` if `address` does not start with `Self::entity_type().hrp()`
    fn __with_address_and_network_id(address: &str, network_id: NetworkID) -> Self {
        assert!(address.starts_with(&Self::entity_type().hrp()), "Invalid address, you SHOULD NOT call this function directly, you should use `try_from_bech32` instead.");
        return Self {
            address: address.to_string(),
            network_id,
        };
    }
}

impl TryInto<AccountAddress> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<AccountAddress, Self::Error> {
        AccountAddress::try_from_bech32(self)
    }
}

impl Display for AccountAddress {
    /// The full bech32 address.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use radix_engine_common::crypto::{Ed25519PublicKey, PublicKey};
    use wallet_kit_test_utils::json::{
        assert_eq_after_json_roundtrip, assert_json_roundtrip, assert_ne_after_json_roundtrip,
    };

    use crate::{
        error::Error,
        v100::networks::{
            account_address::AccountAddress, entity_address::EntityAddress, network_id::NetworkID,
        },
    };

    #[test]
    fn from_bech32() {
        assert!(AccountAddress::try_from_bech32(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        )
        .is_ok());
    }

    #[test]
    fn from_public_key_bytes_and_network_id() {
        let public_key = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap();
        assert_eq!(
            AccountAddress::from_public_key(PublicKey::Ed25519(public_key), NetworkID::Mainnet)
                .address,
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        )
    }

    #[test]
    fn not_equal() {
        let a: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();
        let b: AccountAddress =
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
                .try_into()
                .unwrap();
        assert_ne!(a, b)
    }

    #[test]
    fn equality() {
        let a: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();

        assert_eq!(
            a,
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap()
        )
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
            Err(Error::FailedToDecodeAddressFromBech32)
        )
    }

    #[test]
    fn invalid_checksum() {
        assert_eq!(
            AccountAddress::try_from_bech32(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3apleasx"
            ),
            Err(Error::FailedToDecodeAddressFromBech32)
        )
    }

    #[test]
    fn invalid_entity_type() {
        assert_eq!(
            AccountAddress::try_from_bech32(
                "identity_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            Err(Error::FailedToDecodeAddressFromBech32)
        )
    }

    #[test]
    fn json_roundtrip() {
        let a: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();

        assert_eq_after_json_roundtrip(
            &a,
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
        );
        assert_json_roundtrip(&a);
        assert_ne_after_json_roundtrip(
            &a,
            "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm",
        );
    }
}
