use crate::prelude::*;

#[uniffi::export]
pub fn new_identity_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> IdentityAddress {
    IdentityAddress::new(public_key, network_id)
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet() -> IdentityAddress {
    IdentityAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet_other() -> IdentityAddress {
    IdentityAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet() -> IdentityAddress {
    IdentityAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet_other() -> IdentityAddress {
    IdentityAddress::sample_stokenet_other()
}

impl EntityAddress for IdentityAddress {
    /// Identifies that IdentityAddresses uses the `EntityType::Identity`, which are used
    /// to validate the HRP (`"identity_"`) and is also used when forming HD derivation
    /// paths as per CAP26.
    fn abstract_entity_type() -> AbstractEntityType {
        AbstractEntityType::Identity
    }
}

impl IdentityAddress {
    pub fn new(public_key: PublicKey, network_id: NetworkID) -> Self {
        <Self as EntityAddress>::from_public_key(public_key, network_id)
    }
}

impl IdentityAddress {
    pub fn sample_mainnet() -> Self {
        let address: IdentityAddress = "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn sample_mainnet_other() -> Self {
        let address: IdentityAddress = "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn sample_stokenet() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }
    pub fn sample_stokenet_other() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }
}

impl HasSampleValues for IdentityAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn debug() {
        let a = SUT::try_from_bech32(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j",
        )
        .unwrap();
        assert_eq!(
            format!("{:?}", a),
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
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_stokenet());
        assert_ne!(SUT::sample_mainnet_other(), SUT::sample_stokenet_other());
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
    fn manual_perform_uniffi_conversion() {
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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentityAddress;

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

    #[test]
    fn sample() {
        assert_eq!(
            new_identity_address_sample_mainnet(),
            SUT::sample_mainnet()
        );

        assert_eq!(
            new_identity_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_identity_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_identity_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
