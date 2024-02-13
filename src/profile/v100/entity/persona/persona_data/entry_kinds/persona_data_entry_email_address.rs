use crate::prelude::*;

/// A persons email address they have chosen to associated with a Persona, e.g.
/// `satoshi@btc.org`.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
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
#[display("{email}")]
#[debug("{email}")]
#[serde(transparent)]
pub struct PersonaDataEntryEmailAddress {
    pub email: String,
}

impl Identifiable for PersonaDataEntryEmailAddress {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.email.clone()
    }
}

impl FromStr for PersonaDataEntryEmailAddress {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl PersonaDataEntryEmailAddress {
    pub fn new(email: impl AsRef<str>) -> Result<Self> {
        let email = email.as_ref().to_owned();
        if email.is_empty() {
            return Err(CommonError::PersonaDataInvalidEmailAddressEmpty);
        }
        Ok(Self { email })
    }
}

impl HasPlaceholder for PersonaDataEntryEmailAddress {
    fn placeholder() -> Self {
        Self::new("alan@turing.hero").expect("Valid placeholder.")
    }

    fn placeholder_other() -> Self {
        Self::new("satoshi@nakamoto.btc").expect("Valid placeholder.")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            PersonaDataEntryEmailAddress::placeholder(),
            PersonaDataEntryEmailAddress::placeholder()
        );
        assert_eq!(
            PersonaDataEntryEmailAddress::placeholder_other(),
            PersonaDataEntryEmailAddress::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PersonaDataEntryEmailAddress::placeholder(),
            PersonaDataEntryEmailAddress::placeholder_other()
        );
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(
            PersonaDataEntryEmailAddress::new(""),
            Err(CommonError::PersonaDataInvalidEmailAddressEmpty)
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = PersonaDataEntryEmailAddress::placeholder();
        assert_json_value_eq_after_roundtrip(&model, json!("alan@turing.hero"));
    }

    #[test]
    fn id_is_email() {
        assert_eq!(
            PersonaDataEntryEmailAddress::placeholder().id(),
            PersonaDataEntryEmailAddress::placeholder().email
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            PersonaDataEntryEmailAddress::new("alan@turing.hero".to_string())
                .unwrap(),
            PersonaDataEntryEmailAddress::placeholder()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            PersonaDataEntryEmailAddress::new("alan@turing.hero").unwrap(),
            PersonaDataEntryEmailAddress::placeholder()
        );
    }

    #[test]
    fn new_with_fromstr() {
        let email: PersonaDataEntryEmailAddress =
            "alan@turing.hero".parse().unwrap();
        assert_eq!(email, PersonaDataEntryEmailAddress::placeholder());
    }
}
