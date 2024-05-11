use crate::prelude::*;

#[uniffi::export]
pub fn new_appearance_id(validating: u8) -> Result<AppearanceID> {
    AppearanceID::new(validating)
}

#[uniffi::export]
pub fn new_appearance_id_from_number_of_accounts_on_network(
    count: u64,
) -> AppearanceID {
    AppearanceID::from_number_of_accounts_on_network(count as usize)
}

#[uniffi::export]
pub fn new_appearance_id_sample() -> AppearanceID {
    AppearanceID::sample()
}

#[uniffi::export]
pub fn new_appearance_id_sample_other() -> AppearanceID {
    AppearanceID::sample_other()
}

#[uniffi::export]
pub fn appearance_ids_all() -> Vec<AppearanceID> {
    AppearanceID::all()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AppearanceID;

    #[test]
    fn new() {
        assert_eq!(new_appearance_id(5).unwrap(), SUT::new(5).unwrap());
    }

    #[test]
    fn sample_values() {
        assert_ne!(
            new_appearance_id_sample(),
            new_appearance_id_sample_other()
        );
    }

    #[test]
    fn test_new_appearance_id_from_number_of_accounts_on_network() {
        assert_eq!(
            new_appearance_id_from_number_of_accounts_on_network(23),
            SUT::sample_other()
        )
    }
}
