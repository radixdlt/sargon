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

impl HasSampleValues for PersonaDataEntryEmailAddress {
    fn sample() -> Self {
        Self::new("alan@turing.hero").expect("Valid sample.")
    }

    fn sample_other() -> Self {
        Self::new("satoshi@nakamoto.btc").expect("Valid sample.")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            PersonaDataEntryEmailAddress::sample(),
            PersonaDataEntryEmailAddress::sample()
        );
        assert_eq!(
            PersonaDataEntryEmailAddress::sample_other(),
            PersonaDataEntryEmailAddress::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PersonaDataEntryEmailAddress::sample(),
            PersonaDataEntryEmailAddress::sample_other()
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
    fn json_roundtrip_sample() {
        let model = PersonaDataEntryEmailAddress::sample();
        assert_json_value_eq_after_roundtrip(&model, json!("alan@turing.hero"));
    }

    #[test]
    fn id_is_email() {
        assert_eq!(
            PersonaDataEntryEmailAddress::sample().id(),
            PersonaDataEntryEmailAddress::sample().email
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            PersonaDataEntryEmailAddress::new("alan@turing.hero").unwrap(),
            PersonaDataEntryEmailAddress::sample()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            PersonaDataEntryEmailAddress::new("alan@turing.hero").unwrap(),
            PersonaDataEntryEmailAddress::sample()
        );
    }

    #[test]
    fn new_with_fromstr() {
        let email: PersonaDataEntryEmailAddress =
            "alan@turing.hero".parse().unwrap();
        assert_eq!(email, PersonaDataEntryEmailAddress::sample());
    }
}
