use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RnsDomainDetails {
    pub domain: RnsDomain,
    pub owner: AccountAddress,
    pub gradient_color_start: String,
    pub gradient_color_end: String,
}

impl RnsDomainDetails {
    pub fn new(
        domain: RnsDomain,
        owner: AccountAddress,
        gradient_color_start: String,
        gradient_color_end: String,
     ) -> Self {
        Self { domain, owner, gradient_color_start, gradient_color_end }
    }
}

impl HasSampleValues for RnsDomainDetails {
    fn sample() -> Self {
        RnsDomainDetails::new(
            RnsDomain::sample(),
            AccountAddress::sample_mainnet(),
            "#FFFFFF".to_owned(),
            "#000000".to_owned(),
        )
    }

    fn sample_other() -> Self {
        RnsDomainDetails::new(
            RnsDomain::sample_other(),
            AccountAddress::sample_mainnet_other(),
            "#FFFFFF".to_owned(),
            "#000000".to_owned(),
        )
    }
}

const SCRYPTO_SBOR_DOMAIN_NAME_FIELD: &str = "name";
const SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD: &str = "address";

impl TryFrom<ScryptoSborValue> for RnsDomainDetails {
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

                let domain = RnsDomain::new(name.clone());
                let owner = AccountAddress::from_str(&owner_address)?;
                let (gradient_color_start, gradient_color_end) = domain.gradient_colors();
                Ok(RnsDomainDetails::new(
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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to build a valid ScryptoSborValue for RnsDomainDetails conversion.
    fn valid_domain_details_scrypto_value() -> ScryptoSborValue {
        // Build the "name" field as a String variant.
        let mut name_field = ProgrammaticScryptoSborValueString::new("example.xrd".to_owned());
        name_field.field_name = Some(SCRYPTO_SBOR_DOMAIN_NAME_FIELD.to_owned());
        let name_variant = ProgrammaticScryptoSborValue::String(name_field);

        // Build the owner field as an Enum variant.
        // The enum’s field name should be "address" and its inner fields must yield a reference value.
        let owner_ref = ProgrammaticScryptoSborValueReference::new(AccountAddress::sample_mainnet().to_string());
        let enum_inner = vec![ProgrammaticScryptoSborValue::Reference(owner_ref)];
        let mut owner_enum = ProgrammaticScryptoSborValueEnum::new(enum_inner, "dummy_variant".to_owned());
        owner_enum.field_name = Some(SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD.to_owned());
        let owner_variant = ProgrammaticScryptoSborValue::Enum(owner_enum);

        // Build the tuple with both fields.
        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![name_variant, owner_variant]);
        // Wrap the tuple in the top-level ScryptoSborValue.
        ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        }
    }

    #[test]
    fn test_try_from_valid() {
        let scrypto_value = valid_domain_details_scrypto_value();
        let details = RnsDomainDetails::try_from(scrypto_value).expect("Conversion should succeed");
        // Check that the domain value was set correctly
        assert_eq!(details.domain.0, "example.xrd".to_owned());
        // Check that the gradient colors were computed (we assume gradient_colors is deterministic)
        let (expected_start, expected_end) = details.domain.gradient_colors();
        assert_eq!(details.gradient_color_start, expected_start);
        assert_eq!(details.gradient_color_end, expected_end);
        let expected_owner = AccountAddress::sample_mainnet();
        assert_eq!(details.owner, expected_owner);
    }

    #[test]
    fn test_try_from_unexpected_format() {
        // Create a ScryptoSborValue with a non-tuple variant.
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Bool(ProgrammaticScryptoSborValueBool::default()),
        };
        let result = RnsDomainDetails::try_from(scrypto_value);
        assert_eq!(result, Err(CommonError::UnexpectedNFTDataFormat));
    }

    #[test]
    fn test_missing_name_field() {
        // Build a tuple with missing domain name.
        // Only include the owner field.
        let owner_ref = ProgrammaticScryptoSborValueReference::new(AccountAddress::sample_mainnet().to_string());
        let enum_inner = vec![ProgrammaticScryptoSborValue::Reference(owner_ref)];
        let mut owner_enum = ProgrammaticScryptoSborValueEnum::new(enum_inner, "dummy_variant".to_owned());
        owner_enum.field_name = Some(SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD.to_owned());
        let owner_variant = ProgrammaticScryptoSborValue::Enum(owner_enum);
        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![owner_variant]);

        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };

        let result = RnsDomainDetails::try_from(scrypto_value);
        assert!(matches!(
            result,
            Err(CommonError::MissingNFTDataField { ref field }) if field == "Domain name"
        ));
    }

    #[test]
    fn test_missing_owner_field() {
        // Build a tuple with missing owner address.
        let mut name_field = ProgrammaticScryptoSborValueString::new("example.xrd".to_owned());
        name_field.field_name = Some(SCRYPTO_SBOR_DOMAIN_NAME_FIELD.to_owned());
        let name_variant = ProgrammaticScryptoSborValue::String(name_field);
        // Do not include the owner field.
        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![name_variant]);

        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };

        let result = RnsDomainDetails::try_from(scrypto_value);
        assert!(matches!(
            result,
            Err(CommonError::MissingNFTDataField { ref field }) if field == "Domain owner address"
        ));
    }

    #[test]
    fn test_invalid_owner_address() {
        // Build a valid tuple but supply an invalid owner address that AccountAddress::from_str fails to parse.
        let mut name_field = ProgrammaticScryptoSborValueString::new("example.xrd".to_owned());
        name_field.field_name = Some(SCRYPTO_SBOR_DOMAIN_NAME_FIELD.to_owned());
        let name_variant = ProgrammaticScryptoSborValue::String(name_field);

        let owner_ref = ProgrammaticScryptoSborValueReference::new("invalid_address".to_owned());
        let enum_inner = vec![ProgrammaticScryptoSborValue::Reference(owner_ref)];
        let mut owner_enum = ProgrammaticScryptoSborValueEnum::new(enum_inner, "dummy_variant".to_owned());
        owner_enum.field_name = Some(SCRYPTO_SBOR_DOMAIN_OWNER_ADDRESS_FIELD.to_owned());
        let owner_variant = ProgrammaticScryptoSborValue::Enum(owner_enum);

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![name_variant, owner_variant]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };

        // Expect conversion to fail because AccountAddress::from_str("invalid_address") will return an error.
        let result = RnsDomainDetails::try_from(scrypto_value);
        assert!(result.is_err());
    }
}