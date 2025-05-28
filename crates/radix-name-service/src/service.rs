use crate::prelude::*;

pub struct RadixNameService {
    domains_collection_address: NonFungibleResourceAddress,
    records_collection_address: NonFungibleResourceAddress,

    gateway_client: GatewayClient,
}

impl RadixNameService {
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        domains_collection_address: NonFungibleResourceAddress,
        records_collection_address: NonFungibleResourceAddress,
        network_id: NetworkID,
    ) -> Self {
        let gateway_client = GatewayClient::new(networking_driver, network_id);
        Self {
            domains_collection_address,
            records_collection_address,
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
        // Get domain details
        // Validate domain authenticity
        // Fetch record details
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
            .fetch_non_fungible_data(self.domains_collection_address.clone(), domain_id)
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

#[cfg(test)]
mod fetch_tests {
    use prelude::fixture_gw_model;
    use super::*;
    use serde::Serialize;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixNameService;

    #[actix_rt::test]
    async fn test_fetch_domain_details() {
        let (_, json) = fixture_and_json::<StateNonFungibleDataResponse>(fixture_gw_model!(
            "state/request_non_fungible_data_domain"
        ))
        .unwrap();

        let body = json.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::with_spy(200, body, |req, v| {

            });

        let sut = SUT::new(
            Arc::new(mock_antenna), 
            NonFungibleResourceAddress::sample_mainnet(),
            NonFungibleResourceAddress::sample_mainnet_other(), 
             NetworkID::Mainnet
            );

        let domain = Domain::new("bakirci.xrd".to_owned());
        let result = sut.fetch_domain_details(domain.clone()).await.unwrap();

        let expected_domain_details = DomainDetails::new(
            domain,
            AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap()
        );
        assert_eq!(result, expected_domain_details);
    }
}