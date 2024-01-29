use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{number}")]
#[debug("{number}")]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    number: String,
}

impl Identifiable for PhoneNumber {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.number.clone()
    }
}

impl PhoneNumber {
    pub fn new(number: impl AsRef<str>) -> Result<Self> {
        let number = number.as_ref().to_owned();
        if number.is_empty() {
            return Err(CommonError::PersonaDataInvalidPhoneNumberEmpty);
        }
        return Ok(Self { number });
    }
}

impl HasPlaceholder for PhoneNumber {
    fn placeholder() -> Self {
        PhoneNumber::new("+46123456789").expect("Valid placeholder.")
    }

    fn placeholder_other() -> Self {
        PhoneNumber::new("+449876554321").expect("Valid placeholder.")
    }
}
