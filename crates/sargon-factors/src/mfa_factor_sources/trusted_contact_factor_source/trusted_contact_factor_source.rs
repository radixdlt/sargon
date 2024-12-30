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
        address: impl AsRef<str>, // Actually an AccountAddress
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
impl BaseBaseIsFactorSource for TrustedContactFactorSource {
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
        self.contact.name.value()
    }

    fn set_name(&mut self, updated: String) {
        self.contact.name.update(updated);
    }
}
impl TrustedContactFactorSource {
    fn new_sample(name: &str, email: &str, address: impl AsRef<str>) -> Self {
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
            "account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank",
        )
    }
    pub fn sample_grace() -> Self {
        Self::new_sample(
            "Grace Sample",
            "grace.sample@gmail.com",
            "account_rdx128c4f8dnuvd73d2r3fl95ryfuavw5zjf8zr57hjw0qjagz7s7grace",
        )
    }
    pub fn sample_judy() -> Self {
        Self::new_sample(
            "Judy Sample",
            "judy.sample@gmail.com",
            "account_rdx12y0389ew2xn7w02d059hhye6t0mjzqxqyavsetyg2j3p3xqyepjudy",
        )
    }
    pub fn sample_oscar() -> Self {
        Self::new_sample(
            "Oscar Sample",
            "oscar.sample@gmail.com",
            "account_rdx129uc6rf5vmkj2gau7fgxlsqdg8008nca8yd57sxx4v67dyw7u0scar",
        )
    }
    pub fn sample_trudy() -> Self {
        Self::new_sample(
            "Trudy Sample",
            "trudy.sample@gmail.com",
            "account_rdx1284z0gpg9vnhevn7sytdncszc7ukcrycntg7zjktqvggmwe6ctrudy",
        )
    }
    pub fn sample_radix() -> Self {
        Self::new_sample(
            "Radix InstaBridge",
            "hello@instabridge.com",
            "account_rdx12y7uww27s250g9d3d72ey9wdp5z78zpmq5la0r0wgw4fkf6y8eerdx",
        )
    }
}
impl HasSampleValues for TrustedContactFactorSource {
    fn sample() -> Self {
        let mut source = Self::new(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
            TrustedContactFactorSourceContact::sample(),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }

    fn sample_other() -> Self {
        let mut source = Self::new(
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264",
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

    #[test]
    fn name() {
        let mut sut = SUT::sample();
        assert_eq!(sut.name(), "Spending Account");
        sut.set_name("Savings Account".to_string());
        assert_eq!(sut.name(), "Savings Account");
    }
}
