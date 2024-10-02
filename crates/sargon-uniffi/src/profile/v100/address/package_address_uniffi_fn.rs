use crate::prelude::*;

#[uniffi::export]
pub fn new_package_address_sample_mainnet() -> PackageAddress {
    PackageAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_package_address_sample_mainnet_other() -> PackageAddress {
    PackageAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet() -> PackageAddress {
    PackageAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet_other() -> PackageAddress {
    PackageAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PackageAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63";
        let address = new_package_address(b32.to_owned()).unwrap();
        assert_eq!(package_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(package_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_package_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
                // duplicates should be removed
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
