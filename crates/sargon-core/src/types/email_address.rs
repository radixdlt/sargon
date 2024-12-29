use crate::prelude::*;

/// An email address.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{email}")]
#[debug("{email}")]
#[serde(transparent)]
pub struct EmailAddress {
    pub email: arraystring::MaxString,
}

impl Identifiable for EmailAddress {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.email.clone().as_str().to_owned()
    }
}

impl FromStr for EmailAddress {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl EmailAddress {
    pub fn new(email: impl AsRef<str>) -> Result<Self> {
        let email = email.as_ref().to_owned();
        if email.is_empty() {
            return Err(CommonError::EmailAddressEmpty);
        }
        let maxstring = arraystring::MaxString::try_from_str(email).unwrap();
        Ok(Self { email: maxstring })
    }
}

impl HasSampleValues for EmailAddress {
    fn sample() -> Self {
        Self::new("alan@turing.hero").expect("Valid sample.")
    }

    fn sample_other() -> Self {
        Self::new("satoshi@nakamoto.btc").expect("Valid sample.")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn equality() {
        assert_eq!(EmailAddress::sample(), EmailAddress::sample());
        assert_eq!(EmailAddress::sample_other(), EmailAddress::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(EmailAddress::sample(), EmailAddress::sample_other());
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(EmailAddress::new(""), Err(CommonError::EmailAddressEmpty));
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = EmailAddress::sample();
        assert_json_value_eq_after_roundtrip(&model, json!("alan@turing.hero"));
    }

    #[test]
    fn id_is_email() {
        assert_eq!(
            EmailAddress::sample().id(),
            EmailAddress::sample().email.as_str().to_owned()
        );
    }

    #[test]
    fn new_from_string() {
        assert_eq!(
            EmailAddress::new("alan@turing.hero").unwrap(),
            EmailAddress::sample()
        );
    }

    #[test]
    fn new_from_str() {
        assert_eq!(
            EmailAddress::new("alan@turing.hero").unwrap(),
            EmailAddress::sample()
        );
    }

    #[test]
    fn new_with_fromstr() {
        let email: EmailAddress = "alan@turing.hero".parse().unwrap();
        assert_eq!(email, EmailAddress::sample());
    }
}
