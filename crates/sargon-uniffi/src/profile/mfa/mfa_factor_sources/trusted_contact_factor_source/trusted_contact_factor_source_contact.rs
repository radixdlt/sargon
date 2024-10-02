use crate::prelude::*;

/// Hints about the trusted contact.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{name} {email_address}")]
pub struct TrustedContactFactorSourceContact {
    /// The email address of the contact that the user trusts
    pub email_address: EmailAddress,
    /// The name of the contact that the user trusts
    pub name: DisplayName,
}

impl TrustedContactFactorSourceContact {
    pub fn new(
        email_address: impl Into<EmailAddress>,
        name: impl Into<DisplayName>,
    ) -> Self {
        Self {
            email_address: email_address.into(),
            name: name.into(),
        }
    }
}

impl HasSampleValues for TrustedContactFactorSourceContact {
    fn sample() -> Self {
        Self::new(EmailAddress::sample(), DisplayName::sample())
    }
    fn sample_other() -> Self {
        Self::new(EmailAddress::sample_other(), DisplayName::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TrustedContactFactorSourceContact;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
