use derive_getters::Getters;
use serde::{de, Deserializer, Serialize, Serializer};
use std::fmt::Display;

use crate::{v100::AbstractEntityType, CommonError, NetworkID};

use super::entity_address::EntityAddress;

/// The address of an identity, used by Personas, a bech32 encoding of a public key hash
/// that starts with the prefix `"identity_"`, dependent on NetworkID, meaning the same
/// public key used for two IdentityAddresses on two different networks will not have
/// the same address.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Getters)]
pub struct IdentityAddress {
    /// Human readable address of an identity, which are used by Personas. Always starts with
    /// the prefix `"identity_"`, for example:
    ///
    /// `identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Identity* addresses starts with
    /// the prefix `"identity_"`.
    address: String,

    /// The network this identity address is tied to, i.e. which was used when a public key
    /// hash was used to bech32 encode it. This means that two public key hashes will result
    /// in two different identity address on two different networks.
    network_id: NetworkID,
}

impl EntityAddress for IdentityAddress {
    /// Identifies that IdentityAddresses uses the `EntityType::Identity`, which are used
    /// to validate the HRP (`"identity_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn entity_type() -> AbstractEntityType {
        AbstractEntityType::Identity
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

impl Serialize for IdentityAddress {
    /// Serializes this `IdentityAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.address)
    }
}

impl<'de> serde::Deserialize<'de> for IdentityAddress {
    /// Tries to deserializes a JSON string as a bech32 address into an `IdentityAddress`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<IdentityAddress, D::Error> {
        let s = String::deserialize(d)?;
        IdentityAddress::try_from_bech32(&s).map_err(de::Error::custom)
    }
}

impl TryInto<IdentityAddress> for &str {
    type Error = CommonError;

    /// Tries to deserializes a bech32 address into an `IdentityAddress`.
    fn try_into(self) -> Result<IdentityAddress, Self::Error> {
        IdentityAddress::try_from_bech32(self)
    }
}

impl Display for IdentityAddress {
    /// The full bech32 address.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };
    use radix_engine_common::crypto::{
        Ed25519PublicKey as EngineEd25519PublicKey, PublicKey as EnginePublicKey,
    };
    use serde_json::json;

    use super::*;
    use crate::CommonError as Error;

    #[test]
    fn from_bech32() {
        assert!(IdentityAddress::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .is_ok());
    }

    #[test]
    fn format() {
        let a = IdentityAddress::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .unwrap();
        assert_eq!(
            format!("{}", a),
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
        );
    }

    #[test]
    fn from_public_key_bytes_and_network_id() {
        let public_key = EngineEd25519PublicKey::from_str(
            "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838",
        )
        .unwrap();
        assert_eq!(
            IdentityAddress::from_public_key(
                EnginePublicKey::Ed25519(public_key),
                NetworkID::Mainnet
            )
            .address,
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
        )
    }

    #[test]
    fn network_id() {
        let sut = IdentityAddress::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .unwrap();
        assert_eq!(sut.network_id, NetworkID::Mainnet);
    }

    #[test]
    fn equality() {
        let i: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();
        assert_eq!(
            i,
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap()
        )
    }

    #[test]
    fn not_equal() {
        let i: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();
        let j: IdentityAddress =
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
                .try_into()
                .unwrap();
        assert_ne!(i, j)
    }

    #[test]
    fn invalid() {
        assert_eq!(
            IdentityAddress::try_from_bech32("x"),
            Err(Error::FailedToDecodeAddressFromBech32)
        )
    }

    #[test]
    fn invalid_checksum() {
        assert_eq!(
            IdentityAddress::try_from_bech32(
                "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8x"
            ),
            Err(Error::FailedToDecodeAddressFromBech32)
        )
    }

    #[test]
    fn invalid_entity_type() {
        assert_eq!(
            IdentityAddress::try_from_bech32(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            Err(Error::MismatchingEntityTypeWhileDecodingAddress)
        )
    }

    #[test]
    fn json_roundtrip() {
        let a: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"),
        );
    }
}
