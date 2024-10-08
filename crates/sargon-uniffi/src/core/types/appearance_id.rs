use crate::prelude::*;
use sargon::AppearanceID as InternalAppearanceID;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    
    Hash,
    uniffi::Record,
)]
pub struct AppearanceID {
    pub value: u8,
}

impl From<InternalAppearanceID> for AppearanceID {
    fn from(value: InternalAppearanceID) -> Self {
        Self { value: value.0 }
    }
}

#[uniffi::export]
pub fn new_appearance_id(validating: u8) -> Result<AppearanceID> {
    map_result_from_internal(InternalAppearanceID::new(validating))
}

#[uniffi::export]
pub fn new_appearance_id_from_number_of_accounts_on_network(
    count: u64,
) -> AppearanceID {
    InternalAppearanceID::from_number_of_accounts_on_network(count as usize).into()
}

#[uniffi::export]
pub fn new_appearance_id_sample() -> AppearanceID {
    InternalAppearanceID::sample()
}

#[uniffi::export]
pub fn new_appearance_id_sample_other() -> AppearanceID {
    InternalAppearanceID::sample_other().into()
}

#[uniffi::export]
pub fn appearance_ids_all() -> Vec<AppearanceID> {
    InternalAppearanceID::all().into_iter().map(|x| x.into()).collect()
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
