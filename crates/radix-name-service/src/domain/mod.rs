mod domain;
mod domain_details;

pub use domain::*;
pub use domain_details::*;

use crate::prelude::*;
use crate::service::RadixNameService;

impl RadixNameService {
    pub(crate) async fn check_domain_authenticity(
        &self,
        domain_details: RnsDomainDetails,
    ) -> Result<()> {
        let id = domain_details.domain.root_domain()?.to_non_fungible_id()?;
        let domain_location = self
            .gateway_client
            .fetch_non_fungible_location(
                self.config.domains_collection_address,
                id,
            )
            .await
            .ok()
            .flatten();

        match domain_location {
            Some(location) => {
                if location.as_account() != Some(&domain_details.owner) {
                    return Err(CommonError::RnsUnauthenticDomain {
                        reason: "Account owner mismatch".to_owned(),
                    });
                }
                Ok(())
            }
            None => Err(CommonError::RnsUnauthenticDomain {
                reason: "Failed to read domain location".to_owned(),
            }),
        }
    }

    pub(crate) async fn fetch_domain_details(
        &self,
        domain: RnsDomain,
    ) -> Result<RnsDomainDetails> {
        let domain_id = domain.to_non_fungible_id()?;

        let data = self
            .gateway_client
            .fetch_non_fungible_data(
                self.config.domains_collection_address,
                domain_id,
            )
            .await?;
        let sbor_data =
            data.data.ok_or(CommonError::UnexpectedNFTDataFormat)?;

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
            MockNetworkingDriver::with_spy(200, body, |req, _| {
                let nft_data_request = serde_json::from_slice::<
                    StateNonFungibleDataRequest,
                >(req.body.bytes())
                .unwrap();
                assert_eq!(
                    nft_data_request.resource_address,
                    RadixNameServiceConfig::xrd_domains_mainnet()
                        .domains_collection_address
                );
                assert_eq!(
                    nft_data_request.non_fungible_ids,
                    vec![RnsDomain::new("bakirci.xrd".to_owned())
                        .to_non_fungible_id()
                        .unwrap()]
                );
            });

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let domain = RnsDomain::new("bakirci.xrd".to_owned());
        let result = sut.fetch_domain_details(domain.clone()).await.unwrap();

        let expected_domain_details = RnsDomainDetails::new(
            domain,
            AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap(),
            "#FF5722".to_owned(),
            "#D32F2F".to_owned(),
        );
        assert_eq!(result, expected_domain_details);
    }

    #[actix_rt::test]
    async fn test_check_domain_authenticity_missing_ancestor_address() {
        let nft_location_item = StateNonFungibleLocationResponseItem {
            non_fungible_id: NonFungibleLocalId::from_str(
                "[9a5fb8db4539384dfe275647bfef559e]",
            )
            .unwrap(),
            is_burned: false,
            last_updated_at_state_version: 123456789,
            owning_vault_address: VaultAddress::sample_mainnet(),
            owning_vault_parent_ancestor_address: None,
            owning_vault_global_ancestor_address: None,
        };
        let nft_location_response = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample(),
            resource_address: ResourceAddress::sample_mainnet(),
            non_fungible_ids: vec![nft_location_item],
        };

        let domain_details = RnsDomainDetails::new(
            RnsDomain::new("bakirci.xrd".to_owned()),
            AccountAddress::sample(),
            "#FF5722".to_owned(),
            "#D32F2F".to_owned(),
        );

        let body = nft_location_response.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::with_spy(200, body, |req, _| {
                let location_request = serde_json::from_slice::<
                    StateNonFungibleLocationRequest,
                >(req.body.bytes())
                .unwrap();
                assert_eq!(
                    location_request.resource_address,
                    RadixNameServiceConfig::xrd_domains_mainnet()
                        .domains_collection_address
                        .0
                );
                assert_eq!(
                    location_request.non_fungible_ids,
                    vec![RnsDomain::new("bakirci.xrd".to_owned())
                        .to_non_fungible_id()
                        .unwrap()]
                )
            });

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let result =
            sut.check_domain_authenticity(domain_details.clone()).await;

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_check_domain_authenticity_mismatched_owner() {
        let nft_location_item = StateNonFungibleLocationResponseItem {
            non_fungible_id: NonFungibleLocalId::from_str(
                "[9a5fb8db4539384dfe275647bfef559e]",
            )
            .unwrap(),
            is_burned: false,
            last_updated_at_state_version: 123456789,
            owning_vault_address: VaultAddress::sample_mainnet(),
            owning_vault_parent_ancestor_address: Some(Address::Account(
                AccountAddress::sample_mainnet(),
            )),
            owning_vault_global_ancestor_address: Some(Address::Account(
                AccountAddress::sample_mainnet(),
            )),
        };
        let nft_location_response = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample(),
            resource_address: ResourceAddress::sample_mainnet(),
            non_fungible_ids: vec![nft_location_item],
        };

        let domain_details = RnsDomainDetails::new(
            RnsDomain::new("bakirci.xrd".to_owned()),
            AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap(),
            "#FF5722".to_owned(),
            "#D32F2F".to_owned(),
        );

        let body = nft_location_response.serialize_to_bytes().unwrap();

        let mock_antenna = MockNetworkingDriver::new(200, body);

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let result =
            sut.check_domain_authenticity(domain_details.clone()).await;

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_check_domain_authenticity_valid() {
        let nft_location_item = StateNonFungibleLocationResponseItem {
            non_fungible_id: NonFungibleLocalId::from_str(
                "[9a5fb8db4539384dfe275647bfef559e]",
            )
            .unwrap(),
            is_burned: false,
            last_updated_at_state_version: 123456789,
            owning_vault_address: VaultAddress::sample_mainnet(),
            owning_vault_parent_ancestor_address: Some(Address::Account(
                AccountAddress::sample_mainnet(),
            )),
            owning_vault_global_ancestor_address: Some(Address::Account(
                AccountAddress::sample_mainnet(),
            )),
        };
        let nft_location_response = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample(),
            resource_address: ResourceAddress::sample_mainnet(),
            non_fungible_ids: vec![nft_location_item],
        };

        let domain_details = RnsDomainDetails::new(
            RnsDomain::new("bakirci.xrd".to_owned()),
            AccountAddress::sample_mainnet(),
            "#FF5722".to_owned(),
            "#D32F2F".to_owned(),
        );

        let body = nft_location_response.serialize_to_bytes().unwrap();

        let mock_antenna = MockNetworkingDriver::new(200, body);

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let result =
            sut.check_domain_authenticity(domain_details.clone()).await;

        assert!(result.is_ok());
    }
}
