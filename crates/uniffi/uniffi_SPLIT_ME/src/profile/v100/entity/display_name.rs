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
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct DisplayName {
    pub value: String,
}

impl DisplayName {
    pub fn into_internal(&self) -> InternalDisplayName {
        self.clone().into()
    }
}
impl From<InternalDisplayName> for DisplayName {
    fn from(display_name: InternalDisplayName) -> Self {
        Self {
            value: display_name.value(),
        }
    }
}
impl From<DisplayName> for InternalDisplayName {
    fn from(display_name: DisplayName) -> Self {
        if display_name.value.is_empty() {
            Self::empty()
        } else {
            Self::new(display_name.value.as_str()).unwrap()
        }
    }
}

json_string_convertible!(DisplayName);

#[uniffi::export]
pub fn new_display_name(name: String) -> Result<DisplayName> {
    InternalDisplayName::new(name.as_str()).into_result()
}

#[uniffi::export]
pub fn new_display_name_sample() -> DisplayName {
    InternalDisplayName::sample().into()
}

#[uniffi::export]
pub fn new_display_name_sample_other() -> DisplayName {
    InternalDisplayName::sample_other().into()
}
