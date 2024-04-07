use crate::prelude::*;

impl ComponentAddress {
    pub fn is_global(&self) -> bool {
        self.secret_magic.entity_type()
            == ScryptoEntityType::GlobalGenericComponent
    }

    pub fn is_internal(&self) -> bool {
        self.secret_magic.entity_type()
            == ScryptoEntityType::InternalGenericComponent
    }
}

impl ComponentAddress {
    pub(crate) fn sample_mainnet() -> Self {
        Self::sample_mainnet_global()
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_internal()
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::sample_stokenet_global()
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_internal()
    }
}

/// Returns `true` if the ComponentAddress is `global` (i.e. not `internal`)
#[uniffi::export]
pub fn component_address_is_global(address: &ComponentAddress) -> bool {
    address.is_global()
}

/// Returns `true` if the ComponentAddress is `internal` (i.e. not `global`)
#[uniffi::export]
pub fn component_address_is_internal(address: &ComponentAddress) -> bool {
    address.is_internal()
}

/// Sample to a mainnet ComponentAddress (global)
#[uniffi::export]
pub fn new_component_address_sample_mainnet_global() -> ComponentAddress {
    ComponentAddress::sample_mainnet()
}

/// Sample to a mainnet ComponentAddress (internal)
#[uniffi::export]
pub fn new_component_address_sample_mainnet_internal() -> ComponentAddress {
    ComponentAddress::sample_mainnet_other()
}

/// Sample to a stokenet ComponentAddress (global)
#[uniffi::export]
pub fn new_component_address_sample_stokenet_global() -> ComponentAddress {
    ComponentAddress::sample_stokenet()
}

/// Sample to a stokenet ComponentAddress (internal)
#[uniffi::export]
pub fn new_component_address_sample_stokenet_internal() -> ComponentAddress {
    ComponentAddress::sample_stokenet_other()
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
    use crate::prelude::*;

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
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_component_address_sample_mainnet_global(),
                new_component_address_sample_mainnet_internal(),
                new_component_address_sample_stokenet_global(),
                new_component_address_sample_stokenet_internal(),
                // duplicates should be removed
                new_component_address_sample_mainnet_global(),
                new_component_address_sample_mainnet_internal(),
                new_component_address_sample_stokenet_global(),
                new_component_address_sample_stokenet_internal(),
            ])
            .len(),
            4
        );
    }

    #[test]
    fn test_component_address_is_global() {
        assert!(component_address_is_global(&SUT::sample_mainnet_global()));
        assert!(component_address_is_global(&SUT::sample_stokenet_global()));

        assert!(!component_address_is_global(
            &SUT::sample_stokenet_internal()
        ));
        assert!(!component_address_is_global(&SUT::sample_mainnet_internal()));
    }

    #[test]
    fn test_component_address_is_internal() {
        assert!(component_address_is_internal(
            &SUT::sample_stokenet_internal()
        ));
        assert!(component_address_is_internal(
            &SUT::sample_mainnet_internal()
        ));

        assert!(!component_address_is_internal(&SUT::sample_mainnet_global()));
        assert!(!component_address_is_internal(
            &SUT::sample_stokenet_global()
        ));
    }
}
