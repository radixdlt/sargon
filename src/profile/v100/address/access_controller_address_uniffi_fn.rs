use crate::prelude::*;

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet() -> AccessControllerAddress
{
    AccessControllerAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet_other(
) -> AccessControllerAddress {
    AccessControllerAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet() -> AccessControllerAddress
{
    AccessControllerAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet_other(
) -> AccessControllerAddress {
    AccessControllerAddress::sample_stokenet_other()
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
        let address = new_access_controller_address(b32.to_owned()).unwrap();
        assert_eq!(
            access_controller_address_network_id(&address),
            NetworkID::Mainnet
        );
        assert_eq!(access_controller_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_access_controller_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_access_controller_address_sample_mainnet(),
                new_access_controller_address_sample_mainnet_other(),
                new_access_controller_address_sample_stokenet(),
                new_access_controller_address_sample_stokenet_other(),
                // duplicates should be removed
                new_access_controller_address_sample_mainnet(),
                new_access_controller_address_sample_mainnet_other(),
                new_access_controller_address_sample_stokenet(),
                new_access_controller_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
