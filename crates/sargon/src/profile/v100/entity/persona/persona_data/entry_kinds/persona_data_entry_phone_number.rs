use crate::prelude::*;

/// A persons telephone number they have chosen to associated with a Persona, e.g.
/// `+46 987 654 321` (don't try calling this number, it does not exist).
///
/// Current implementation does not validate the phone number other than it
/// cannot be empty, since telephone number validation is tricky.
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
#[serde(transparent)]
pub struct PersonaDataEntryPhoneNumber {
    pub number: String,
}

impl Identifiable for PersonaDataEntryPhoneNumber {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.number.clone()
    }
}

impl FromStr for PersonaDataEntryPhoneNumber {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl PersonaDataEntryPhoneNumber {
    pub fn new(number: impl AsRef<str>) -> Result<Self> {
        let number = number.as_ref().to_owned();
        if number.is_empty() {
            return Err(CommonError::PersonaDataInvalidPhoneNumberEmpty);
        }
        Ok(Self { number })
    }
}

impl HasSampleValues for PersonaDataEntryPhoneNumber {
    fn sample() -> Self {
        PersonaDataEntryPhoneNumber::new("+46123456789").expect("Valid sample.")
    }

    fn sample_other() -> Self {
        PersonaDataEntryPhoneNumber::new("+44987654321").expect("Valid sample.")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            PersonaDataEntryPhoneNumber::sample(),
            PersonaDataEntryPhoneNumber::sample()
        );
        assert_eq!(
            PersonaDataEntryPhoneNumber::sample_other(),
            PersonaDataEntryPhoneNumber::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PersonaDataEntryPhoneNumber::sample(),
            PersonaDataEntryPhoneNumber::sample_other()
        );
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(
            PersonaDataEntryPhoneNumber::new(""),
            Err(CommonError::PersonaDataInvalidPhoneNumberEmpty)
        );
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = PersonaDataEntryPhoneNumber::sample();
        assert_json_value_eq_after_roundtrip(&model, json!("+46123456789"));
    }

    #[test]
    fn id_is_number() {
        assert_eq!(
            PersonaDataEntryPhoneNumber::sample().id(),
            PersonaDataEntryPhoneNumber::sample().number
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            PersonaDataEntryPhoneNumber::new("+46123456789").unwrap(),
            PersonaDataEntryPhoneNumber::sample()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            PersonaDataEntryPhoneNumber::new("+46123456789").unwrap(),
            PersonaDataEntryPhoneNumber::sample()
        );
    }

    #[test]
    fn new_with_fromstr() {
        let phone: PersonaDataEntryPhoneNumber =
            "+46123456789".parse().unwrap();
        assert_eq!(phone, PersonaDataEntryPhoneNumber::sample());
    }
}
