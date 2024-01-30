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
#[display("{email}")]
#[debug("{email}")]
#[serde(transparent)]
pub struct EmailAddress {
    pub email: String,
}

impl Identifiable for EmailAddress {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.email.clone()
    }
}

impl EmailAddress {
    pub fn new(email: impl AsRef<str>) -> Result<Self> {
        let email = email.as_ref().to_owned();
        if email.is_empty() {
            return Err(CommonError::PersonaDataInvalidEmailAddressEmpty);
        }
        Ok(Self { email })
    }
}

impl HasPlaceholder for EmailAddress {
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
        assert_eq!(EmailAddress::placeholder(), EmailAddress::placeholder());
        assert_eq!(
            EmailAddress::placeholder_other(),
            EmailAddress::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            EmailAddress::placeholder(),
            EmailAddress::placeholder_other()
        );
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(
            EmailAddress::new(""),
            Err(CommonError::PersonaDataInvalidEmailAddressEmpty)
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = EmailAddress::placeholder();
        assert_json_value_eq_after_roundtrip(&model, json!("alan@turing.hero"));
    }

    #[test]
    fn id_is_email() {
        assert_eq!(
            EmailAddress::placeholder().id(),
            EmailAddress::placeholder().email
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            EmailAddress::new("alan@turing.hero".to_string()).unwrap(),
            EmailAddress::placeholder()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            EmailAddress::new("alan@turing.hero").unwrap(),
            EmailAddress::placeholder()
        );
    }
}
