use crate::prelude::*;

#[uniffi::export]
pub fn new_locker_address_sample_mainnet() -> LockerAddress {
    LockerAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_locker_address_sample_mainnet_other() -> LockerAddress {
    LockerAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet() -> LockerAddress {
    LockerAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet_other() -> LockerAddress {
    LockerAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LockerAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 =
            "locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz";
        let address = new_locker_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(locker_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(locker_address_bech32_address(&address), b32);
    }

    #[test]
    fn sample() {
        assert_eq!(new_locker_address_sample_mainnet(), SUT::sample_mainnet());

        assert_eq!(
            new_locker_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_locker_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_locker_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
