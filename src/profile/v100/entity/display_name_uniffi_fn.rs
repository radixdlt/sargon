use crate::prelude::*;

#[uniffi::export]
pub fn new_display_name(name: String) -> Result<DisplayName> {
    DisplayName::new(name.as_str())
}

#[uniffi::export]
pub fn new_display_name_sample() -> DisplayName {
    DisplayName::sample()
}

#[uniffi::export]
pub fn new_display_name_sample_other() -> DisplayName {
    DisplayName::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DisplayName;

    #[test]
    fn new() {
        assert_eq!(
            new_display_name("Main".to_string()).unwrap(),
            SUT::new("Main").unwrap(),
        );
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_display_name_sample(),
                new_display_name_sample_other(),
                // duplicates should be removed
                new_display_name_sample(),
                new_display_name_sample_other(),
            ])
            .len(),
            2
        );
    }
}
