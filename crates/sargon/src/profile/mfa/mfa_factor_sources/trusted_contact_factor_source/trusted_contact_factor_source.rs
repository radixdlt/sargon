use crate::prelude::*;

/// A factor source representing a person, company, organization or otherwise
/// entity that the user trusts to help her with recovery, if ever needed.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{contact} {id}")]
pub struct TrustedContactFactorSource {
    /// Unique and stable identifier of this factor source.
    pub id: FactorSourceIDFromAddress,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// The contact information about the contact that is 'trusted'.
    pub contact: TrustedContactFactorSourceContact,
}

impl TrustedContactFactorSource {
    pub fn with_details(
        id: FactorSourceIDFromAddress,
        common: FactorSourceCommon,
        contact: TrustedContactFactorSourceContact,
    ) -> Self {
        Self {
            id,
            common,
            contact,
        }
    }

    pub fn new(
        address: AccountAddress,
        contact: TrustedContactFactorSourceContact,
    ) -> Self {
        let id = FactorSourceIDFromAddress::new_for_trusted_contact(address);
        Self::with_details(id, FactorSourceCommon::new_babylon(), contact)
    }
}

impl TryFrom<FactorSource> for TrustedContactFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        value.clone().into_trusted_contact().map_err(|_| {
            CommonError::InvalidFactorSourceKind {
                bad_value: value.factor_source_kind().to_string(),
            }
        })
    }
}

impl From<TrustedContactFactorSource> for FactorSource {
    fn from(value: TrustedContactFactorSource) -> Self {
        FactorSource::TrustedContact { value }
    }
}

impl IsFactorSource for TrustedContactFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::TrustedContact
    }
}
impl BaseIsFactorSource for TrustedContactFactorSource {
    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }

    fn name(&self) -> String {
        self.contact.name.value.clone()
    }
}
impl TrustedContactFactorSource {
    fn new_sample(name: &str, email: &str, address: AccountAddress) -> Self {
        let mut source = Self::new(
            address,
            TrustedContactFactorSourceContact::new(
                EmailAddress::new(email).unwrap(),
                DisplayName::new(name).unwrap(),
            ),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }

    pub fn sample_frank() -> Self {
        Self::new_sample(
            "Frank Sample",
            "frank.sample@gmail.com",
            AccountAddress::sample_frank(),
        )
    }
    pub fn sample_grace() -> Self {
        Self::new_sample(
            "Grace Sample",
            "grace.sample@gmail.com",
            AccountAddress::sample_grace(),
        )
    }
    pub fn sample_judy() -> Self {
        Self::new_sample(
            "Judy Sample",
            "judy.sample@gmail.com",
            AccountAddress::sample_judy(),
        )
    }
    pub fn sample_oscar() -> Self {
        Self::new_sample(
            "Oscar Sample",
            "oscar.sample@gmail.com",
            AccountAddress::sample_oscar(),
        )
    }
    pub fn sample_trudy() -> Self {
        Self::new_sample(
            "Trudy Sample",
            "trudy.sample@gmail.com",
            AccountAddress::sample_trudy(),
        )
    }
    pub fn sample_radix() -> Self {
        Self::new_sample(
            "Radix InstaBridge",
            "hello@instabridge.com",
            AccountAddress::sample_radix(),
        )
    }
}
impl HasSampleValues for TrustedContactFactorSource {
    fn sample() -> Self {
        let mut source = Self::new(
            AccountAddress::sample_mainnet(),
            TrustedContactFactorSourceContact::sample(),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }

    fn sample_other() -> Self {
        let mut source = Self::new(
            AccountAddress::sample_mainnet_other(),
            TrustedContactFactorSourceContact::sample_other(),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TrustedContactFactorSource;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn kind() {
        assert_eq!(SUT::kind(), FactorSourceKind::TrustedContact);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_factor_source() {
        let sut = SUT::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(SUT::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: "device".to_owned()
            })
        );
    }
}
