use crate::prelude::*;

pub struct RadixNameService {
    config: RadixNameServiceConfig,
    gateway_client: GatewayClient,
}

#[derive(Clone)]
struct RadixNameServiceConfig {
    domains_collection_address: NonFungibleResourceAddress,
    records_collection_address: NonFungibleResourceAddress,
}

impl RadixNameServiceConfig {
    fn new(
        domains_collection_address: NonFungibleResourceAddress,
    records_collection_address: NonFungibleResourceAddress,
    ) -> Self {
        Self {
            domains_collection_address,
            records_collection_address
        }
    }

    fn xrd_domains_mainnet() -> Self {
        Self::new(
            NonFungibleResourceAddress::from_str("resource_rdx1n2dd0w53zpdlqdz65vpymygj8a60vqnggyuxfpfdldjmy2224x020q").unwrap(),
            NonFungibleResourceAddress::from_str("resource_rdx1nf7lt68zan0fvlfqqrtnxasxjmv877ncnr2kpdl69t076sw4whjc27").unwrap(),
        )
    }

    fn xrd_domains_stokenet() -> Self {
        Self::new(
            NonFungibleResourceAddress::from_str("resource_tdx_2_1n2leg5zgd0cw3766mdae43jg8dvp2h4x08rjjcrf3qrta8lhfjt7wq").unwrap(),
            NonFungibleResourceAddress::from_str("resource_tdx_2_1ng2r922evyvtzhdfdh4r2nqznw4zwkfesed296aclc5xqfr857t8mz").unwrap(),
        )
    }
}

impl RadixNameService {
    fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        config: RadixNameServiceConfig,
        network_id: NetworkID,
    ) -> Self {
        let gateway_client = GatewayClient::new(networking_driver, network_id);
        Self {
            config,
            gateway_client,
        }
    }

    pub fn new_xrd_domains(networking_driver: Arc<dyn NetworkingDriver>, network_id: NetworkID) -> Result<Self> {
        if let config = Self::xrd_domains_config().get(&network_id).unwrap().clone() {
            Ok(Self::new(networking_driver, config, network_id))
        } else {
            Err(CommonError::Unknown)
        }
    }

    fn xrd_domains_config() -> HashMap<NetworkID, RadixNameServiceConfig> {
        HashMap::from([
            (NetworkID::Mainnet, RadixNameServiceConfig::xrd_domains_mainnet()),
            (NetworkID::Stokenet, RadixNameServiceConfig::xrd_domains_stokenet()),
        ])
    }
}

impl RadixNameService {
    pub async fn resolve_receiver_account_for_domain(
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
            .fetch_non_fungible_data(self.config.domains_collection_address.clone(), domain_id)
            .await?;
        let sbor_data = data.data.ok_or(CommonError::Unknown)?;

        TryFrom::try_from(sbor_data)
    }

    async fn fetch_record_details(
        &self,
        domain: Domain,
        docket: Docket
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

        let sut = SUT::new_xrd_domains(
            Arc::new(mock_antenna),
             NetworkID::Mainnet
            ).unwrap();

        let domain = Domain::new("bakirci.xrd".to_owned());
        let result = sut.fetch_domain_details(domain.clone()).await.unwrap();

        let expected_domain_details = DomainDetails::new(
            domain,
            AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap()
        );
        assert_eq!(result, expected_domain_details);
    }
}