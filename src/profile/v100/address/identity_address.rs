use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalIdentityAddress as RetIdentityAddress;

/// Human readable address of an identity, which are used by Personas. Always starts with
/// the prefix `"identity_"`, for example:
///
/// `identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j`
///
/// Addresses are checksummed, as per Bech32. **Only** *Identity* addresses starts with
/// the prefix `"identity_"`.
///
/// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of IdentityAddresses:
/// * GlobalIdentity,
/// * GlobalVirtualSecp256k1Identity,
/// * GlobalVirtualEd25519Identity
///
/// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalIdentityAddress`][ret], and
/// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
///
/// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
/// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L229-L234
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
#[display("{secret_magic}")]
pub struct IdentityAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetIdentityAddress,
}

#[uniffi::export]
pub fn new_identity_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> IdentityAddress {
    IdentityAddress::new(public_key, network_id)
}

impl EntityAddress for IdentityAddress {
    /// Identifies that IdentityAddresses uses the `EntityType::Identity`, which are used
    /// to validate the HRP (`"identity_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn entity_type() -> AbstractEntityType {
        AbstractEntityType::Identity
    }
}

impl IdentityAddress {
    pub fn new(public_key: PublicKey, network_id: NetworkID) -> Self {
        <Self as EntityAddress>::from_public_key(public_key, network_id)
    }
}

impl IdentityAddress {
    pub fn placeholder_mainnet() -> Self {
        let address: IdentityAddress = "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn placeholder_mainnet_other() -> Self {
        let address: IdentityAddress = "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn placeholder_stokenet() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }
    pub fn placeholder_stokenet_other() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08"
            .parse()
            .expect("Should have a valid placeholder value");
        assert_eq!(address.network_id(), NetworkID::Stokenet);
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use radix_engine_common::crypto::{
        Ed25519PublicKey as ScryptoEd25519PublicKey,
        PublicKey as ScryptoPublicKey,
    };

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentityAddress;

    #[test]
    fn from_bech32() {
        assert!(SUT::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .is_ok());
    }

    #[test]
    fn from_str() {
        assert!(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
                .parse::<SUT>()
                .is_ok()
        );
    }

    #[test]
    fn display() {
        let a = SUT::try_from_bech32(
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
        let public_key = Ed25519PublicKey::from_str(
            "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838",
        )
        .unwrap();
        assert_eq!(
            SUT::from_public_key(
                PublicKey::Ed25519 { value: public_key },
                NetworkID::Mainnet
            )
            .address(),
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
        )
    }

    #[test]
    fn network_id() {
        let sut = SUT::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .unwrap();
        assert_eq!(sut.network_id(), NetworkID::Mainnet);
    }

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
        let s = "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8x";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn invalid_entity_type() {
        let s = "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease";
        assert_eq!(
            SUT::try_from_bech32(s),
            Err(CommonError::FailedToDecodeAddressFromBech32 {
                bad_value: s.to_owned()
            })
        )
    }

    #[test]
    fn json_roundtrip_success() {
        let a: SUT =
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
        assert_json_value_fails::<SUT>(
            json!("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
        );
        assert_json_value_fails::<SUT>(
            json!("identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzkuxx")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x";
        let address = new_identity_address(b32.to_owned()).unwrap();
        assert_eq!(identity_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(identity_address_bech32_address(&address), b32);
    }

    #[test]
    fn new_from_public_key_bytes_and_network_id() {
        let public_key = Ed25519PublicKey::from_str(
            "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838",
        )
        .unwrap();
        assert_eq!(
            new_identity_address_from(PublicKey::Ed25519 { value: public_key }, NetworkID::Mainnet)
            .address(),
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
        )
    }
}
