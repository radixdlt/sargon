use crate::prelude::*;

#[uniffi::export]
pub fn new_network_id_from_discriminant(discriminant: u8) -> Result<NetworkID> {
    NetworkID::try_from(discriminant)
}

#[uniffi::export]
pub fn network_id_to_string(id: NetworkID) -> String {
    id.logical_name()
}

#[uniffi::export]
pub fn network_id_discriminant(id: NetworkID) -> u8 {
    id.discriminant()
}

#[uniffi::export]
pub fn network_ids_all() -> Vec<NetworkID> {
    NetworkID::all()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkID;

    #[test]
    fn test_network_id_to_string() {
        assert_eq!(network_id_to_string(SUT::Mainnet), "mainnet");
    }

    #[test]
    fn test_try_from_discriminant() {
        assert_eq!(new_network_id_from_discriminant(1).unwrap(), SUT::Mainnet);
    }

    #[test]
    fn test_network_id_discriminant() {
        assert_eq!(network_id_discriminant(SUT::Mainnet), 1);
        assert_eq!(network_id_discriminant(SUT::Stokenet), 2);
    }

    #[test]
    fn all() {
        assert_eq!(network_ids_all().len(), 12);
    }
}
