use crate::prelude::*;
use sargon::RadixNameService as InternalRadixNameService;

#[derive(uniffi::Object)]
pub struct RadixNameService {
    pub wrapped: Arc<InternalRadixNameService>,
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
pub fn rns_domain_validated(domain: RnsDomain) -> Result<RnsDomain> {
    domain.into_internal().validated().into_result()
}

#[uniffi::export]
impl RadixNameService {
    #[uniffi::method]
    pub async fn resolve_receiver_account_for_domain(
        &self,
        domain: RnsDomain,
    ) -> Result<RnsDomainConfiguredReceiver> {
        self.wrapped
            .resolve_receiver_account_for_domain(domain.into_internal())
            .await
            .into_result()
    }
}
