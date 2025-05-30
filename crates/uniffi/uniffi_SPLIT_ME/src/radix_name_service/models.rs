use crate::prelude::*;

use sargon::RnsDomain as InternalRnsDomain;
use sargon::RnsDomainConfiguredReceiver as InternalRnsDomainConfiguredReceiver;
use sargon::RnsDomainDetails as InternalRnsDomainDetails;

uniffi::custom_newtype!(RnsDomain, String);
#[derive(Debug, Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct RnsDomain(pub String);

#[derive(
    PartialEq, Eq, Hash, Clone, Debug, InternalConversion, uniffi::Record,
)]
pub struct RnsDomainDetails {
    pub domain: RnsDomain,
    pub owner: AccountAddress,
    pub gradient_color_start: String,
    pub gradient_color_end: String,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct RnsDomainConfiguredReceiver {
    pub domain: RnsDomainDetails,
    pub receiver: AccountAddress,
}