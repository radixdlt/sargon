use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DomainDetails {
    pub domain: Domain,
    pub owner: AccountAddress,
}

impl DomainDetails {
    pub fn new(domain: Domain, owner: AccountAddress) -> Self {
        Self { domain, owner }
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
                    domain: Domain::new(name),
                    owner: AccountAddress::from_str(&owner_address)?,
                })
            }
            _ => Err(CommonError::Unknown),
        }
    }
}
