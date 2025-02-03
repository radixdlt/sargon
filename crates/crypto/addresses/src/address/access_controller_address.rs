use crate::prelude::*;

decl_address!(
    /// Address to an AccessController that controls an Account or Identity (Persona),
    /// it said entity has been "securified", e.g.:
    /// `"accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"`
    ///
    /// When a user applies a SecurityStructureConfiguration for the first time on a
    /// non-securified entity (and signs and submit the resulting TX) said entity is
    /// "assigned" an AccessControllerAddress by the network.
    ///
    /// An `AccessControllerAddress` has the [Scrypto's `EntityType`][entt] `GlobalAccessController`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccessControllerAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L247-L248
    accessController => [ScryptoEntityType::GlobalAccessController]
);

impl HasSampleValues for AccessControllerAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl AccessControllerAddress {
    pub fn sample_mainnet() -> Self {
        "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a".parse().expect("Sample")
    }

    pub fn sample_mainnet_other() -> Self {
        "accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak".parse().expect("Sample")
    }

    pub fn sample_stokenet() -> Self {
        "accesscontroller_tdx_2_1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9f53juq".parse().expect("Sample")
    }

    pub fn sample_stokenet_other() -> Self {
        "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny".parse().expect("Sample")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample_mainnet(), SUT::sample_mainnet());
        assert_eq!(SUT::sample_mainnet_other(), SUT::sample_mainnet_other());
        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_mainnet_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_stokenet());
        assert_ne!(SUT::sample_mainnet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn display() {
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2vxx")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}
