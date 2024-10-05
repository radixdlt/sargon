use crate::prelude::*;
use sargon::DisplayName as InternalDisplayName;

/// A max 30 chars long string used for display purposes, e.g.
/// the name of an Account or Persona.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
/// #[allow(clippy::upper_case_acronyms)]
/// type SUT = DisplayName;
///
/// assert_eq!(SUT::MAX_LEN, 30);
/// assert_eq!("Satoshi".parse::<SUT>().unwrap().to_string(), "Satoshi");
/// ```
///
/// Names with longer than 30 chars are trimmed.
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
/// #[allow(clippy::upper_case_acronyms)]
/// type SUT = DisplayName;
/// assert_eq!("A very big name that is over than 30 characters long".parse::<SUT>().unwrap().to_string(), "A very big name that is over t");
/// ```
///
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{value}")]
pub struct DisplayName {
    pub value: String,
}

impl From<InternalDisplayName> for DisplayName {
    fn from(value: InternalDisplayName) -> Self {
        Self {
            value: value.value,
        }
    }
}

impl Into<InternalDisplayName> for DisplayName {
    fn into(self) -> InternalDisplayName {
        InternalDisplayName {
            value: self.value,
        }
    }
}

json_string_convertible!(DisplayName);

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
