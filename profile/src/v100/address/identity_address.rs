use crate::prelude::*;

/// The address of an identity, used by Personas, a bech32 encoding of a public key hash
/// that starts with the prefix `"identity_"`, dependent on NetworkID, meaning the same
/// public key used for two IdentityAddresses on two different networks will not have
/// the same address.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{address}")]
pub struct IdentityAddress {
    /// Human readable address of an identity, which are used by Personas. Always starts with
    /// the prefix `"identity_"`, for example:
    ///
    /// `identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Identity* addresses starts with
    /// the prefix `"identity_"`.
    pub address: String,

    /// The network this identity address is tied to, i.e. which was used when a public key
    /// hash was used to bech32 encode it. This means that two public key hashes will result
    /// in two different identity address on two different networks.
    pub network_id: NetworkID,
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

impl IdentityAddress {
    pub fn placeholder_mainnet() -> Self {
        let address: IdentityAddress = "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id, NetworkID::Mainnet);
        address
    }
    pub fn placeholder_mainnet_other() -> Self {
        let address: IdentityAddress = "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id, NetworkID::Mainnet);
        address
    }
    pub fn placeholder_stokenet() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id, NetworkID::Stokenet);
        address
    }
    pub fn placeholder_stokenet_other() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id, NetworkID::Stokenet);
        address
    }
}

impl HasPlaceholder for IdentityAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}

impl FromStr for IdentityAddress {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IdentityAddress::try_from_bech32(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use radix_engine_common::crypto::{
        Ed25519PublicKey as EngineEd25519PublicKey,
        PublicKey as EnginePublicKey,
    };

    #[test]
    fn from_bech32() {
        assert!(IdentityAddress::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .is_ok());
    }

    #[test]
    fn from_str() {
        assert!(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
                .parse::<IdentityAddress>()
                .is_ok()
        );
    }

    #[test]
    fn display() {
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
        assert_eq!(
            IdentityAddress::placeholder(),
            IdentityAddress::placeholder()
        );
        assert_eq!(
            IdentityAddress::placeholder_other(),
            IdentityAddress::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            IdentityAddress::placeholder(),
            IdentityAddress::placeholder_other()
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            IdentityAddress::try_from_bech32("x"),
            Err(CommonError::FailedToDecodeAddressFromBech32("x".to_owned()))
        )
    }

    #[test]
    fn invalid_checksum() {
        let s = "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8x";
        assert_eq!(
            IdentityAddress::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32(s.to_owned()))
        )
    }

    #[test]
    fn invalid_entity_type() {
        assert_eq!(
            IdentityAddress::try_from_bech32(
                "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            ),
            Err(CommonError::MismatchingEntityTypeWhileDecodingAddress)
        )
    }

    #[test]
    fn json_roundtrip_success() {
        let a: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .parse()
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

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<IdentityAddress>(
            json!("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
        );
        assert_json_value_fails::<IdentityAddress>(
            json!("identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzkuxx")
        );
        assert_json_value_fails::<IdentityAddress>(json!("super invalid"));
    }
}
