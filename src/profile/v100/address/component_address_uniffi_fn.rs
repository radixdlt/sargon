use crate::prelude::*;

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

#[cfg(test)]
mod uniffi_tests {

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
