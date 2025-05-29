use crate::prelude::*;

use indexmap::map::raw_entry_v1::RawEntryMut;
use sargon::AccountAddress as InternalAccountAddress;
use sargon::Domain as InternalDomain;
use sargon::DomainDetails as InternalDomainDetails;
use sargon::ResolvedReceiver as InternalResolvedReceiver;
use sargon::RadixNameService as InternalRadixNameService;

#[derive(uniffi::Object)]
pub struct RadixNameService {
    pub wrapped: Arc<InternalRadixNameService>,
}

uniffi::custom_newtype!(Domain, String);
#[derive(Debug, Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct Domain(pub String);

#[derive(PartialEq, Eq, Hash, Clone, Debug, InternalConversion, uniffi::Record)]
pub struct DomainDetails {
    pub domain: Domain,
    pub owner: AccountAddress,
    pub gradient_color_start: String,
    pub gradient_color_end: String,
}

#[uniffi::export]
impl RadixNameService {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let wrapped = InternalRadixNameService::new_xrd_domains(
            Arc::new(NetworkingDriverAdapter {
                wrapped: networking_driver,
            }),
            network_id.into(),
        )
        .into_result()?;

        return Ok(Self {
            wrapped: Arc::new(wrapped),
        });
    }
}

#[uniffi::export]
pub fn rns_domain_validated(domain: Domain) -> Result<Domain> {
    domain.into_internal()
        .validated()
        .into_result()
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ResolvedReceiver {
    pub domain: DomainDetails,
    pub account: AccountAddress,
}

#[uniffi::export]
impl RadixNameService {
    #[uniffi::method]
    pub async fn resolve_receiver_account_for_domain(
        &self,
        domain: Domain,
    ) -> Result<ResolvedReceiver> {
        self.wrapped
            .resolve_receiver_account_for_domain(
                domain.into_internal(),
            )
            .await
            .into_result()
    }
}
