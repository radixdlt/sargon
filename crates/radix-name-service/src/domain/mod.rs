mod domain;
mod domain_details;

pub use domain::*;
pub use domain_details::*;

use crate::service::RadixNameService;
use crate::prelude::*;

impl RadixNameService {
    pub(crate) async fn check_domain_authenticity(
        &self,
        domain_details: DomainDetails,
    ) -> Result<()> {
        let id = domain_details.domain.to_non_fungible_id()?;
        let domain_location = self.gateway_client.fetch_non_fungible_location(self.config.domains_collection_address, id).await.ok().flatten();
        match domain_location {
            Some(location) => {
                if location.as_account() != Some(&domain_details.owner){
                    return Err(CommonError::RnsUnauthenticDomain { reason: "Account owner missmatch".to_owned()});
                }
                return Ok(())
            }
            None => {
                return Err(CommonError::RnsUnauthenticDomain { reason: "Failed to reado domain location".to_owned() });
            }
            
        }
    }

    pub(crate) async fn fetch_domain_details(
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
        let sbor_data = data.data.ok_or(CommonError::UnexpectedNFTDataFormat)?;

        TryFrom::try_from(sbor_data)
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
}