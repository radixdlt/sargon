use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalComponentAddress as RetComponentAddress;

/// An address to some On-Ledger (OnNetwork) component, e.g. a Dapp, being an instantiation
/// of some Scrypto blueprint, e.g:
/// `"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"`
///
/// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ComponentAddress:
/// * GlobalGenericComponent
/// * InternalGenericComponent
///
/// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalComponentAddress`][ret], and
/// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
///
/// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
/// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L243-L246
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
pub struct ComponentAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetComponentAddress,
}

/// Placeholder to a mainnet ComponentAddress (global)
#[uniffi::export]
pub fn new_component_address_placeholder() -> ComponentAddress {
    ComponentAddress::placeholder_mainnet_global()
}

/// Placeholder to a mainnet ComponentAddress (internal)
#[uniffi::export]
pub fn new_component_address_placeholder_other() -> ComponentAddress {
    ComponentAddress::placeholder_mainnet_internal()
}

impl HasPlaceholder for ComponentAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_global()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_internal()
    }
}

impl ComponentAddress {
    pub fn placeholder_mainnet_global() -> Self {
        "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
            .parse()
            .expect("Placeholder")
    }

    pub fn placeholder_mainnet_internal() -> Self {
        "internal_component_rdx1lrhpef83s2c25zp9kzlk7qjak4en6llr7pw2zpuv5cswzufh9ff2ug".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_global() -> Self {
        "component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_internal() -> Self {
        "internal_component_tdx_2_1lpjekpazrlrf2726kc29vur0nhpjk2p3jlswu3yyl72h9jghyq498r".parse().expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentAddress;

    #[test]
    fn equality() {
        assert_eq!(
            SUT::placeholder_mainnet_internal(),
            SUT::placeholder_mainnet_internal()
        );
        assert_eq!(
            SUT::placeholder_mainnet_global(),
            SUT::placeholder_mainnet_global()
        );
        assert_eq!(
            SUT::placeholder_stokenet_internal(),
            SUT::placeholder_stokenet_internal()
        );
        assert_eq!(
            SUT::placeholder_stokenet_global(),
            SUT::placeholder_stokenet_global()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            SUT::placeholder_mainnet_internal(),
            SUT::placeholder_mainnet_global()
        );
        assert_ne!(
            SUT::placeholder_stokenet_internal(),
            SUT::placeholder_mainnet_internal()
        );
        assert_ne!(
            SUT::placeholder_stokenet_global(),
            SUT::placeholder_mainnet_global()
        );
    }

    #[test]
    fn display() {
        let s = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("internal_component_rdx1lrhpef83s2c25zp9kzlk7qjak4en6llr7pw2zpuv5cswzufh9ff2ug"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucex")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_resource_address, EntityAddress};

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "internal_component_rdx1lrhpef83s2c25zp9kzlk7qjak4en6llr7pw2zpuv5cswzufh9ff2ug";
        let address = new_component_address(b32.to_owned()).unwrap();
        assert_eq!(component_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(component_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_component_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            SUT::placeholder_mainnet_global(),
            new_component_address_placeholder()
        );
        assert_eq!(
            SUT::placeholder_mainnet_internal(),
            new_component_address_placeholder_other()
        );
    }
}
