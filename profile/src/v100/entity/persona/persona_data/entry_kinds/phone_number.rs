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
#[serde(transparent)]
pub struct PhoneNumber {
    pub number: String,
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
        Ok(Self { number })
    }
}

impl HasPlaceholder for PhoneNumber {
    fn placeholder() -> Self {
        PhoneNumber::new("+46123456789").expect("Valid placeholder.")
    }

    fn placeholder_other() -> Self {
        PhoneNumber::new("+44987654321").expect("Valid placeholder.")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(PhoneNumber::placeholder(), PhoneNumber::placeholder());
        assert_eq!(
            PhoneNumber::placeholder_other(),
            PhoneNumber::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PhoneNumber::placeholder(),
            PhoneNumber::placeholder_other()
        );
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(
            PhoneNumber::new(""),
            Err(CommonError::PersonaDataInvalidPhoneNumberEmpty)
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = PhoneNumber::placeholder();
        assert_json_value_eq_after_roundtrip(&model, json!("+46123456789"));
    }

    #[test]
    fn id_is_number() {
        assert_eq!(
            PhoneNumber::placeholder().id(),
            PhoneNumber::placeholder().number
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            PhoneNumber::new("+46123456789".to_string()).unwrap(),
            PhoneNumber::placeholder()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            PhoneNumber::new("+46123456789").unwrap(),
            PhoneNumber::placeholder()
        );
    }
}
