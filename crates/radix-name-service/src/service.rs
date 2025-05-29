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
            records_collection_address,
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

    pub fn new_xrd_domains(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
    ) -> Result<Self> {
        if let config =
            Self::xrd_domains_config().get(&network_id).unwrap().clone()
        {
            Ok(Self::new(networking_driver, config, network_id))
        } else {
            Err(CommonError::Unknown)
        }
    }

    fn xrd_domains_config() -> HashMap<NetworkID, RadixNameServiceConfig> {
        HashMap::from([
            (
                NetworkID::Mainnet,
                RadixNameServiceConfig::xrd_domains_mainnet(),
            ),
            (
                NetworkID::Stokenet,
                RadixNameServiceConfig::xrd_domains_stokenet(),
            ),
        ])
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedReceiver {
    pub domain: DomainDetails,
    pub account: AccountAddress,
}

impl HasSampleValues for ResolvedReceiver {
    fn sample() -> Self {
        ResolvedReceiver::new(
            DomainDetails::sample(),
            AccountAddress::sample_mainnet()
        )
    }

    fn sample_other() -> Self {
        ResolvedReceiver::new(
            DomainDetails::sample_other(),
            AccountAddress::sample_mainnet_other()
        )
    }
}

impl ResolvedReceiver {
    pub fn new(domain: DomainDetails, account: AccountAddress) -> Self {
        Self { domain, account }
    }
}

impl RadixNameService {
    pub async fn resolve_receiver_account_for_domain(
        &self,
        domain: Domain,
    ) -> Result<ResolvedReceiver> {
        let domain_details =
        self.fetch_domain_details(domain.clone()).await?;
    if domain != domain_details.domain {
        return Err(CommonError::Unknown);
    }

    self.check_domain_authenticity(domain_details.clone())
        .await?;

        let record = self
            .resolve_record(domain.clone(), Docket::wildcard_receiver())
            .await?;
        let account = match record.value {
            ProgrammaticScryptoSborValue::String(account_str) => {
                AccountAddress::from_str(&account_str.value)?
            }
            _ => return Err(CommonError::Unknown),
        };

        Ok(ResolvedReceiver::new(domain_details, account ))
    }
}

impl RadixNameService {
    async fn resolve_record(
        &self,
        domain: Domain,
        docket: Docket,
    ) -> Result<RecordDetails> {
        let record = self.fetch_record_details(domain.clone(), docket.clone())
            .await?;

        if record.domain_id != domain.to_non_fungible_id()? {
            return Err(CommonError::Unknown);
        }
        if record.context != docket.context {
            return Err(CommonError::Unknown);
        }
        if record.directive != docket.directive {
            return Err(CommonError::Unknown);
        }

        return Ok(record)
    }

    async fn check_domain_authenticity(
        &self,
        domain_details: DomainDetails,
    ) -> Result<()> {
        let id = domain_details.domain.to_non_fungible_id()?;
        let domain_location = self.gateway_client.fetch_non_fungible_location(self.config.domains_collection_address, id).await?;
        match domain_location {
            Some(location) => {
                if location.as_account() != Some(&domain_details.owner){
                    return Err(CommonError::Unknown);
                }
                return Ok(())
            }
            None => {
                return Err(CommonError::Unknown);
            }
            
        }
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
            .fetch_non_fungible_data(
                self.config.domains_collection_address.clone(),
                domain_id,
            )
            .await?;
        let sbor_data = data.data.ok_or(CommonError::Unknown)?;

        TryFrom::try_from(sbor_data)
    }

    async fn fetch_record_details(
        &self,
        domain: Domain,
        docket: Docket,
    ) -> Result<RecordDetails> {
        let record_id = docket.to_non_fungible_id(domain.clone())?;

        let data = self
            .gateway_client
            .fetch_non_fungible_data(
                self.config.records_collection_address.clone(),
                record_id,
            )
            .await?;
        let sbor_data = data.data.ok_or(CommonError::Unknown)?;

        TryFrom::try_from(sbor_data)
    }
}

#[cfg(test)]
mod pub_api_tests {
    use super::*;
    use prelude::fixture_gw_model;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixNameService;

    #[actix_rt::test]
    async fn test_resolve_receiver_account_for_domain() {
        let (_, json) = fixture_and_json::<StateNonFungibleDataResponse>(
            fixture_gw_model!("state/request_non_fungible_data_domain_record"),
        )
        .unwrap();

        let body = json.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::with_spy(200, body, |req, v| {});

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let domain = Domain::new("grenadine.xrd".to_owned());
        let result = sut
            .resolve_receiver_account_for_domain(domain.clone())
            .await
            .unwrap();

            let expected_domain_details = DomainDetails::new(
                domain,
                AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap(),
                "#FF5722".to_owned(),
                "#D32F2F".to_owned(),
            );

        assert_eq!(
            result,
            ResolvedReceiver::new(
                expected_domain_details,
                AccountAddress::from_str("account_rdx128pu3gp74hgl0a9d6d899vd0nn8wh5z0syrkvp28hd492dk0u8fe8t").unwrap()
            )
        );
    }
}

#[cfg(test)]
mod fetch_tests {
    use super::*;
    use prelude::fixture_gw_model;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixNameService;

    #[actix_rt::test]
    async fn test_fetch_domain_details() {
        let (_, json) = fixture_and_json::<StateNonFungibleDataResponse>(
            fixture_gw_model!("state/request_non_fungible_data_domain"),
        )
        .unwrap();

        let body = json.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::with_spy(200, body, |req, v| {});

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let domain = Domain::new("bakirci.xrd".to_owned());
        let result = sut.fetch_domain_details(domain.clone()).await.unwrap();

        let expected_domain_details = DomainDetails::new(
            domain,
            AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap(),
            "#FF5722".to_owned(),
            "#D32F2F".to_owned(),
        );
        assert_eq!(result, expected_domain_details);
    }

    #[actix_rt::test]
    async fn test_fetch_domain_record_details() {
        let (_, json) = fixture_and_json::<StateNonFungibleDataResponse>(
            fixture_gw_model!("state/request_non_fungible_data_domain_record"),
        )
        .unwrap();

        let body = json.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::with_spy(200, body, |req, v| {});

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let domain = Domain::new("grenadine.xrd".to_owned());
        let docket = Docket::wildcard_receiver();
        let result = sut
            .fetch_record_details(domain.clone(), docket.clone())
            .await
            .unwrap();

        let expected_details = RecordDetails::new(
            domain.to_non_fungible_id().unwrap(),
            docket.context,
            docket.directive,
            ProgrammaticScryptoSborValue::String(ProgrammaticScryptoSborValueString::new("account_rdx128pu3gp74hgl0a9d6d899vd0nn8wh5z0syrkvp28hd492dk0u8fe8t".to_owned())),
        );

        assert_eq!(result, expected_details);
    }
}
