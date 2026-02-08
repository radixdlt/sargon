use crate::prelude::*;

decl_address!(
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
    /// ```
    /// use addresses::IdentityAddress;
    /// use network::prelude::{IsNetworkAware, NetworkID};
    ///
    /// assert_eq!(
    ///     "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j".parse::<IdentityAddress>().unwrap().network_id(),
    ///     NetworkID::Mainnet
    /// );
    /// ```
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalIdentityAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L229-L234
    identity => [
        ScryptoEntityType::GlobalIdentity,
        ScryptoEntityType::GlobalPreallocatedSecp256k1Identity,
        ScryptoEntityType::GlobalPreallocatedEd25519Identity
    ]
);

impl HasEntityKind for IdentityAddress {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}
impl IsBaseEntityAddress for IdentityAddress {}
impl IsEntityAddress for IdentityAddress {}

impl IdentityAddress {
    pub fn new_from_public_key(
        public_key: PublicKey,
        network_id: NetworkID,
    ) -> Self {
        <Self as IsEntityAddress>::from_public_key(public_key, network_id)
    }
}

impl IdentityAddress {
    pub fn sample_mainnet() -> Self {
        let address: IdentityAddress = "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn sample_mainnet_other() -> Self {
        let address: IdentityAddress = "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Mainnet);
        address
    }
    pub fn sample_stokenet() -> Self {
        let address: IdentityAddress = "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8"
            .parse()
            .expect("Should have a valid sample value");
        assert_eq!(address.network_id(), NetworkID::Stokenet);
        address
    }
    pub fn sample_stokenet_other() -> Self {
        let address: IdentityAddress = "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq"
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
                PublicKey::Ed25519(public_key),
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
            json!("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
        );
        assert_json_value_fails::<SUT>(
            json!("identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzkuxx")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }
}
