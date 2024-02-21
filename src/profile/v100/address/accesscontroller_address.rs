use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalAccessControllerAddress as RetAccessControllerAddress;

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
/// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccessControllerAddress`][ret], and
/// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
///
/// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
/// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L247-L248
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
pub struct AccessControllerAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetAccessControllerAddress,
}

#[uniffi::export]
pub fn new_accesscontroller_address_placeholder() -> AccessControllerAddress {
    AccessControllerAddress::placeholder()
}

#[uniffi::export]
pub fn new_accesscontroller_address_placeholder_other(
) -> AccessControllerAddress {
    AccessControllerAddress::placeholder_other()
}

impl HasPlaceholder for AccessControllerAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl AccessControllerAddress {
    pub fn placeholder_mainnet() -> Self {
        "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a".parse().expect("Placeholder")
    }

    pub fn placeholder_mainnet_other() -> Self {
        "accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet() -> Self {
        "accesscontroller_tdx_2_1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9f53juq".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_other() -> Self {
        "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny".parse().expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_common::address::AddressBech32DecodeError;
    use radix_engine_toolkit::models::canonical_address_types::{
        CanonicalAccessControllerAddress, CanonicalAddress,
        CanonicalAddressError,
    };

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder_mainnet(), SUT::placeholder_mainnet());
        assert_eq!(
            SUT::placeholder_mainnet_other(),
            SUT::placeholder_mainnet_other()
        );
        assert_eq!(SUT::placeholder_stokenet(), SUT::placeholder_stokenet());
        assert_eq!(
            SUT::placeholder_stokenet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            SUT::placeholder_mainnet(),
            SUT::placeholder_mainnet_other()
        );
        assert_ne!(SUT::placeholder_mainnet(), SUT::placeholder_stokenet());
        assert_ne!(
            SUT::placeholder_mainnet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn display() {
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
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

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let address = new_accesscontroller_address(b32.to_owned()).unwrap();
        assert_eq!(
            accesscontroller_address_network_id(&address),
            NetworkID::Mainnet
        );
        assert_eq!(accesscontroller_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_accesscontroller_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            new_accesscontroller_address_placeholder(),
            SUT::placeholder()
        );

        assert_eq!(
            new_accesscontroller_address_placeholder_other(),
            SUT::placeholder_other()
        );
    }
}
