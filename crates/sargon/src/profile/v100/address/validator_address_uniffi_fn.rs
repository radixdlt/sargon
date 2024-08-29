use crate::prelude::*;

#[uniffi::export]
pub fn new_validator_address_sample_mainnet() -> ValidatorAddress {
    ValidatorAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_validator_address_sample_mainnet_other() -> ValidatorAddress {
    ValidatorAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet() -> ValidatorAddress {
    ValidatorAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet_other() -> ValidatorAddress {
    ValidatorAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ValidatorAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0";
        let address = new_validator_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(validator_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(validator_address_bech32_address(&address), b32);
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_validator_address_sample_mainnet(),
            SUT::sample_mainnet()
        );

        assert_eq!(
            new_validator_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_validator_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_validator_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
