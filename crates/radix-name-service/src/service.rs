use crate::prelude::*;

pub struct RadixNameService {
    domains_collection: NonFungibleResourceAddress,
    records_collection: NonFungibleResourceAddress,

    gateway_client: GatewayClient,
}

impl RadixNameService {
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        domains_collection: NonFungibleResourceAddress,
        records_collection: NonFungibleResourceAddress,
        network_id: NetworkID,
    ) -> Self {
        let gateway_client = GatewayClient::new(networking_driver, network_id);
        Self {
            domains_collection,
            records_collection,
            gateway_client,
        }
    }
}

impl RadixNameService {
    async fn get_receiver_account_for_domain(
        &self,
        domain: Domain,
    ) {
        todo!()
    }
}

impl RadixNameService {
    async fn get_domain_details(
        &self,
        domain: Domain,
    ) -> Result<DomainDetails> {
        let fetched_domain_details =
            self.fetch_domain_details(domain.clone()).await?;
        if domain != fetched_domain_details.domain {
            return Err(CommonError::Unknown);
        }

        self.check_domain_authenticity(fetched_domain_details.clone())
            .await?;

        Ok(fetched_domain_details)
    }

    async fn resolve_record(
        &self,
        domain: Domain,
        docket: Docket
    ) {
        // 
        todo!()
    }

    async fn check_domain_authenticity(
        &self,
        domain_details: DomainDetails,
    ) -> Result<()> {
        Ok(())
    }


}

/// Fetch
impl RadixNameService {
    async fn fetch_domain_details(
        &self,
        domain: Domain,
    ) -> Result<DomainDetails> {
        let domain_id = domain.to_non_fungible_id()?;

        let data = self
            .gateway_client
            .fetch_non_fungible_data(self.domains_collection.clone(), domain_id)
            .await?;
        let sbor_data = data.data.ok_or(CommonError::Unknown)?;

        TryFrom::try_from(sbor_data)
    }

    async fn fetch_record_details(
        &self
    ) {
        todo!()
    }

    async fn fetch_account_domains(
        &self,
        account: AccountAddress
    ) {
        todo!()
    }
}