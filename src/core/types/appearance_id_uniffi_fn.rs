use crate::prelude::*;

#[uniffi::export]
pub fn new_appearance_id(validating: u8) -> Result<AppearanceID> {
    AppearanceID::new(validating)
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
    vec![
        AppearanceID::gradient0(),
        AppearanceID::gradient1(),
        AppearanceID::gradient2(),
        AppearanceID::gradient3(),
        AppearanceID::gradient4(),
        AppearanceID::gradient5(),
        AppearanceID::gradient6(),
        AppearanceID::gradient7(),
        AppearanceID::gradient8(),
        AppearanceID::gradient9(),
        AppearanceID::gradient10(),
        AppearanceID::gradient11(),
    ]
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new() {
        assert_eq!(new_appearance_id(5).unwrap(), AppearanceID::gradient5());
    }

    #[test]
    fn sample_values() {
        assert_ne!(
            new_appearance_id_sample(),
            new_appearance_id_sample_other()
        );
    }
}
