mod docket;
mod record_details;

pub use docket::*;
pub use record_details::*;

use crate::service::RadixNameService;
use crate::prelude::*;

impl RadixNameService {
    pub(crate) async fn resolve_record(
        &self,
        domain: Domain,
        docket: Docket,
    ) -> Result<RecordDetails> {
        let record = self.fetch_record_details(domain.clone(), docket.clone())
            .await?;

        if record.domain_id != domain.to_non_fungible_id()? {
            return Err(CommonError::RnsInvalidDomainConfiguration { reason: "domain_id missmatch".to_owned() });
        }
        if record.context != docket.context {
            return Err(CommonError::RnsInvalidDomainConfiguration { reason: "context missmatch".to_owned() });
        }
        if record.directive != docket.directive {
            return Err(CommonError::RnsInvalidDomainConfiguration { reason: "directive missmatch".to_owned()});
        }

        return Ok(record)
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