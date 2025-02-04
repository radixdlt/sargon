use crate::prelude::*;

decl_address!(
    /// An address to some On-Ledger (OnNetwork) component, e.g. a Dapp, being an instantiation
    /// of some Scrypto blueprint, e.g:
    /// `"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ComponentAddress:
    /// * GlobalGenericComponent
    /// * InternalGenericComponent
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalComponentAddress`][ret]
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L243-L246
    component => [
        ScryptoEntityType::GlobalGenericComponent,
        ScryptoEntityType::InternalGenericComponent
    ]
);

impl ComponentAddress {
    pub fn is_global(&self) -> bool {
        self.entity_type() == ScryptoEntityType::GlobalGenericComponent
    }

    pub fn is_internal(&self) -> bool {
        self.entity_type() == ScryptoEntityType::InternalGenericComponent
    }
}

impl ComponentAddress {
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_global()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_internal()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_global()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_internal()
    }
}

impl HasSampleValues for ComponentAddress {
    fn sample() -> Self {
        Self::sample_mainnet_global()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_internal()
    }
}

impl ComponentAddress {
    pub fn sample_mainnet_global() -> Self {
        "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"
            .parse()
            .expect("Sample")
    }

    pub fn sample_mainnet_internal() -> Self {
        "internal_component_rdx1lrhpef83s2c25zp9kzlk7qjak4en6llr7pw2zpuv5cswzufh9ff2ug".parse().expect("Sample")
    }

    pub fn sample_stokenet_global() -> Self {
        "component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl".parse().expect("Sample")
    }

    pub fn sample_stokenet_internal() -> Self {
        "internal_component_tdx_2_1lpjekpazrlrf2726kc29vur0nhpjk2p3jlswu3yyl72h9jghyq498r".parse().expect("Sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentAddress;

    #[test]
    fn equality() {
        assert_eq!(
            SUT::sample_mainnet_internal(),
            SUT::sample_mainnet_internal()
        );
        assert_eq!(SUT::sample_mainnet_global(), SUT::sample_mainnet_global());
        assert_eq!(
            SUT::sample_stokenet_internal(),
            SUT::sample_stokenet_internal()
        );
        assert_eq!(
            SUT::sample_stokenet_global(),
            SUT::sample_stokenet_global()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(
            SUT::sample_mainnet_internal(),
            SUT::sample_mainnet_global()
        );
        assert_ne!(
            SUT::sample_stokenet_internal(),
            SUT::sample_mainnet_internal()
        );
        assert_ne!(SUT::sample_stokenet_global(), SUT::sample_mainnet_global());
    }

    #[test]
    fn is_internal() {
        assert!(SUT::sample_stokenet_internal().is_internal());
        assert!(SUT::sample_mainnet_internal().is_internal());
        assert!(!SUT::sample_mainnet_global().is_internal());
        assert!(!SUT::sample_stokenet_global().is_internal());
    }

    #[test]
    fn is_global() {
        assert!(SUT::sample_mainnet_global().is_global());
        assert!(SUT::sample_stokenet_global().is_global());
        assert!(!SUT::sample_stokenet_internal().is_global());
        assert!(!SUT::sample_mainnet_internal().is_global());
    }

    #[test]
    fn display() {
        let s = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn debug() {
        let s = "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
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
