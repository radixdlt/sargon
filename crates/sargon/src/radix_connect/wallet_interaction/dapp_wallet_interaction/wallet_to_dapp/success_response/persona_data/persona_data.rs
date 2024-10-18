use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionPersonaDataRequestResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonaDataEntryName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<PersonaDataEntryEmailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PersonaDataEntryPhoneNumber>>,
}

impl WalletToDappInteractionPersonaDataRequestResponseItem {
    pub fn new(
        name: impl Into<Option<PersonaDataEntryName>>,
        email_addresses: impl Into<Option<Vec<PersonaDataEntryEmailAddress>>>,
        phone_numbers: impl Into<Option<Vec<PersonaDataEntryPhoneNumber>>>,
    ) -> Self {
        Self {
            name: name.into(),
            email_addresses: email_addresses.into(),
            phone_numbers: phone_numbers.into(),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionPersonaDataRequestResponseItem {
    fn sample() -> Self {
        Self::new(
            PersonaDataEntryName::sample(),
            vec![PersonaDataEntryEmailAddress::sample()],
            vec![PersonaDataEntryPhoneNumber::sample()],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            PersonaDataEntryName::sample_other(),
            vec![PersonaDataEntryEmailAddress::sample_other()],
            vec![PersonaDataEntryPhoneNumber::sample_other()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionPersonaDataRequestResponseItem;

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
