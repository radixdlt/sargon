use gateway_client_and_api::NonFungibleResourceAddress;

use crate::prelude::*;

#[derive(PartialEq, Eq, Clone)]
struct Domain(String);

#[derive(PartialEq, Eq, Clone)]
struct DomainDetails {
    domain: Domain,
    owner: AccountAddress,
}

struct RadixNameService {
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

    async fn check_domain_authenticity(
        &self,
        domain_details: DomainDetails,
    ) -> Result<()> {
        Ok(())
    }

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
}

impl TryFrom<ScryptoSborValue> for DomainDetails {
    type Error = CommonError;

    fn try_from(value: ScryptoSborValue) -> Result<Self> {
        match value.programmatic_json {
            ProgrammaticScryptoSborValue::Tuple(tuple) => {
                let name = tuple
                    .fields
                    .get_string_field("name")
                    .map(|field| field.value)
                    .ok_or(CommonError::Unknown)?;
                let owner_address = tuple
                    .fields
                    .get_enum_field("address")
                    .and_then(|field| {
                        field
                            .fields
                            .first_reference_field()
                            .map(|field| field.value)
                    })
                    .ok_or(CommonError::Unknown)?;

                Ok(DomainDetails {
                    domain: Domain(name),
                    owner: AccountAddress::from_str(&owner_address)?,
                })
            }
            _ => Err(CommonError::Unknown),
        }
    }
}

impl Domain {
    fn to_non_fungible_id(&self) -> Result<NonFungibleLocalId> {
        domain_to_non_fungible_id(&self.0, true)
    }
}
