use super::{entity_address::EntityAddress, entity_type::EntityType, network_id::NetworkID};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdentityAddress {
    pub address: String,
    pub network_id: NetworkID,
}

impl EntityAddress for IdentityAddress {
    fn entity_type() -> EntityType {
        EntityType::Identity
    }

    fn with_address_and_network_id(address: &str, network_id: NetworkID) -> Self {
        Self::validate(address);
        return Self {
            address: address.to_string(),
            network_id,
        };
    }
}

impl Display for IdentityAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use radix_engine_common::crypto::{Ed25519PublicKey, PublicKey};

    use crate::{
        error::Error,
        v100::networks::{
            entity_address::EntityAddress, identity_address::IdentityAddress, network_id::NetworkID,
        },
    };

    #[test]
    fn from_bech32() {
        assert!(IdentityAddress::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .is_ok());
    }

    #[test]
    fn from_public_key_bytes_and_network_id() {
        let public_key = Ed25519PublicKey::from_str(
            "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838",
        )
        .unwrap();
        assert_eq!(
            IdentityAddress::from_public_key(PublicKey::Ed25519(public_key), NetworkID::Mainnet)
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
}
