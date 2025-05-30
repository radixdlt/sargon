use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DomainDetails {
    pub domain: Domain,
    pub owner: AccountAddress,
    pub gradient_color_start: String,
    pub gradient_color_end: String,
}

impl DomainDetails {
    pub fn new(
        domain: Domain,
        owner: AccountAddress,
        gradient_color_start: String,
        gradient_color_end: String,
     ) -> Self {
        Self { domain, owner, gradient_color_start, gradient_color_end }
    }
}

impl HasSampleValues for DomainDetails {
    fn sample() -> Self {
        DomainDetails::new(
            Domain::sample(),
            AccountAddress::sample_mainnet(),
            "#FFFFFF".to_owned(),
            "#000000".to_owned(),
        )
    }

    fn sample_other() -> Self {
        DomainDetails::new(
            Domain::sample_other(),
            AccountAddress::sample_mainnet_other(),
            "#FFFFFF".to_owned(),
            "#000000".to_owned(),
        )
    }
}

const SCRYPTO_SBOR_DOMAIN_NAME_FIELD: &str = "name";
const SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD: &str = "address";

impl TryFrom<ScryptoSborValue> for DomainDetails {
    type Error = CommonError;

    fn try_from(value: ScryptoSborValue) -> Result<Self> {
        match value.programmatic_json {
            ProgrammaticScryptoSborValue::Tuple(tuple) => {
                let name = tuple
                    .fields
                    .get_string_field(SCRYPTO_SBOR_DOMAIN_NAME_FIELD)
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain name".to_owned() })?;

                let owner_address = tuple
                    .fields
                    .get_enum_field(SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD)
                    .and_then(|field| {
                        field
                            .fields
                            .first_reference_field()
                            .map(|field| field.value)
                    })
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain owner address".to_owned() })?;

                let domain = Domain::new(name.clone());
                let owner = AccountAddress::from_str(&owner_address)?;
                let (gradient_color_start, gradient_color_end) = domain.gradient_colors();
                Ok(DomainDetails::new(
                    domain,
                    owner,
                    gradient_color_start,
                    gradient_color_end
                ))
            }
            _ => Err(CommonError::UnexpectedNFTDataFormat),
        }
    }
}