use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RecordDetails {
    pub domain_id: NonFungibleLocalId,
    pub context: DocketContext,
    pub directive: Directive,
    pub value: ProgrammaticScryptoSborValue,
}

impl RecordDetails {
    pub fn new(
        domain_id: NonFungibleLocalId,
        context: DocketContext,
        directive: Directive,
        value: ProgrammaticScryptoSborValue,
    ) -> Self {
        Self {
            domain_id,
            context,
            directive,
            value,
        }
    }
}

impl RecordDetails {
    pub fn validate(&self, domain: &Domain, docket: &Docket) -> Result<()> {
        if self.domain_id != domain.to_non_fungible_id()? {
            return Err(CommonError::RnsInvalidDomainConfiguration {
                reason: "RecordDetails domain_id does not match Docket"
                    .to_owned(),
            });
        }

        if self.context != docket.context {
            return Err(CommonError::RnsInvalidDomainConfiguration {
                reason: "RecordDetails context does not match Docket context"
                    .to_owned(),
            });
        }

        if self.directive != docket.directive {
            return Err(CommonError::RnsInvalidDomainConfiguration {
                reason:
                    "RecordDetails directive does not match Docket directive"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

const SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD: &str = "domain_id";
const SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD: &str = "context";
const SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD: &str = "directive";
const SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD: &str = "value";

impl TryFrom<ScryptoSborValue> for RecordDetails {
    type Error = CommonError;

    fn try_from(value: ScryptoSborValue) -> Result<Self> {
        match value.programmatic_json {
            ProgrammaticScryptoSborValue::Tuple(tuple) => {
                let domain_id = tuple
                    .fields
                    .get_non_fungible_local_id_field(
                        SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
                    )
                    .ok_or(CommonError::MissingNFTDataField {
                        field: "Domain record domain id".to_owned(),
                    })?;

                let context_str = tuple
                    .fields
                    .get_string_field(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD)
                    .ok_or(CommonError::MissingNFTDataField {
                        field: "Domain record docket context".to_owned(),
                    })?;

                let directive_str = tuple
                    .fields
                    .get_enum_field(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD)
                    .and_then(|field| field.fields.first_string_field())
                    .ok_or(CommonError::MissingNFTDataField {
                        field: "Domain record docket directive".to_owned(),
                    })?;

                let value = tuple
                    .fields
                    .get_enum_field(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD)
                    .and_then(|field| field.fields.first().cloned())
                    .ok_or(CommonError::MissingNFTDataField {
                        field: "Domain record value".to_owned(),
                    })?;

                let context = DocketContext::from_str(&context_str)?;
                let directive = Directive::new(directive_str);

                Ok(RecordDetails {
                    domain_id,
                    context,
                    directive,
                    value,
                })
            }
            _ => Err(CommonError::UnexpectedNFTDataFormat),
        }
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_record_details_validation_success() {
        let domain = Domain::new("example.com".to_owned());
        let docket = Docket::wildcard_receiver();
        let domain_id = domain.to_non_fungible_id().unwrap();
        let context = DocketContext::Receivers;
        let directive = Directive::wildcard();
        let value = ProgrammaticScryptoSborValue::String(
            ProgrammaticScryptoSborValueString::new(
                AccountAddress::sample_mainnet().to_string(),
            ),
        );

        let record_details =
            RecordDetails::new(domain_id, context, directive, value);
        assert!(record_details.validate(&domain, &docket).is_ok());
    }

    #[test]
    fn test_record_details_validation_domain_mismatch() {
        let domain = Domain::new("example.com".to_owned());
        let docket = Docket::wildcard_receiver();
        let domain_id = Domain::new("example2.com".to_owned())
            .to_non_fungible_id()
            .unwrap();
        let context = DocketContext::Receivers;
        let directive = Directive::wildcard();
        let value = ProgrammaticScryptoSborValue::String(
            ProgrammaticScryptoSborValueString::new(
                AccountAddress::sample_mainnet().to_string(),
            ),
        );

        let record_details =
            RecordDetails::new(domain_id, context, directive, value);
        assert!(record_details.validate(&domain, &docket).is_err());
    }

    #[test]
    fn test_record_details_validation_context_mismatch() {
        let domain = Domain::new("example.com".to_owned());
        let docket = Docket::wildcard_receiver();
        let domain_id = domain.to_non_fungible_id().unwrap();
        let context = DocketContext::Delegation; // Different context
        let directive = Directive::wildcard();
        let value = ProgrammaticScryptoSborValue::String(
            ProgrammaticScryptoSborValueString::new(
                AccountAddress::sample_mainnet().to_string(),
            ),
        );

        let record_details =
            RecordDetails::new(domain_id, context, directive, value);
        assert!(record_details.validate(&domain, &docket).is_err());
    }

    #[test]
    fn test_record_details_validation_directive_mismatch() {
        let domain = Domain::new("example.com".to_owned());
        let docket = Docket::wildcard_receiver();
        let domain_id = domain.to_non_fungible_id().unwrap();
        let context = DocketContext::Receivers;
        let directive = Directive::new("specific_directive".to_owned()); // Different directive
        let value = ProgrammaticScryptoSborValue::String(
            ProgrammaticScryptoSborValueString::new(
                AccountAddress::sample_mainnet().to_string(),
            ),
        );

        let record_details =
            RecordDetails::new(domain_id, context, directive, value);
        assert!(record_details.validate(&domain, &docket).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a string field with a specified field name.
    fn new_string_field(
        value: &str,
        field_name: &str,
    ) -> ProgrammaticScryptoSborValue {
        let mut field =
            ProgrammaticScryptoSborValueString::new(value.to_owned());
        field.field_name = Some(field_name.to_owned());
        ProgrammaticScryptoSborValue::String(field)
    }

    // Helper function to create a NonFungibleLocalId field with a specified field name.
    fn new_nf_local_id_field(
        value: &str,
        field_name: &str,
    ) -> ProgrammaticScryptoSborValue {
        let mut field = ProgrammaticScryptoSborValueNonFungibleLocalId::new(
            value.to_owned(),
        );
        field.field_name = Some(field_name.to_owned());
        ProgrammaticScryptoSborValue::NonFungibleLocalId(field)
    }

    // Helper function to create an enum field wrapping a string field.
    fn new_enum_field(
        value: &str,
        field_name: &str,
    ) -> ProgrammaticScryptoSborValue {
        let inner = new_string_field(value, ""); // inner field does not require a name
        let mut enum_variant = ProgrammaticScryptoSborValueEnum::new(
            vec![inner],
            "dummy_variant".to_owned(),
        );
        enum_variant.field_name = Some(field_name.to_owned());
        ProgrammaticScryptoSborValue::Enum(enum_variant)
    }

    // Helper to build a valid ScryptoSborValue for RecordDetails conversion.
    fn valid_record_details_value() -> ScryptoSborValue {
        let domain_id_variant = new_nf_local_id_field(
            "[9a5fb8db4539384dfe275647bfef559e]",
            SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
        );
        let context_variant = new_string_field(
            &DocketContext::Receivers.to_string(),
            SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD,
        );
        let directive_variant =
            new_enum_field("*", SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD);
        let value_variant = new_enum_field(
            "some value",
            SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD,
        );

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            domain_id_variant,
            context_variant,
            directive_variant,
            value_variant,
        ]);
        ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        }
    }

    #[test]
    fn test_try_from_valid() {
        let scrypto_value = valid_record_details_value();
        let record = RecordDetails::try_from(scrypto_value)
            .expect("Conversion should succeed");
        // Check domain_id conversion.
        assert_eq!(
            record.domain_id.to_string(),
            "[9a5fb8db4539384dfe275647bfef559e]".to_owned()
        );
        // Check docket context conversion.
        let expected_context = DocketContext::Receivers;
        assert_eq!(record.context, expected_context);
        // Check the directive.
        let expected_directive = Directive::new("*".to_owned());
        assert_eq!(record.directive, expected_directive);
        // Check the value field: it should be a string with "some value".
        if let ProgrammaticScryptoSborValue::String(inner_str) = record.value {
            assert_eq!(inner_str.value, "some value".to_owned());
        } else {
            panic!("Value field is not a string");
        }
    }

    #[test]
    fn test_try_from_unexpected_format() {
        // Provide a ScryptoSborValue with a non-tuple variant.
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Bool(
                ProgrammaticScryptoSborValueBool::default(),
            ),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert_eq!(result, Err(CommonError::UnexpectedNFTDataFormat));
    }

    #[test]
    fn test_missing_domain_id() {
        // Build a tuple without the domain_id field.
        let context_variant = new_string_field(
            &DocketContext::Receivers.to_string(),
            SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD,
        );
        let directive_variant =
            new_enum_field("*", SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD);
        let value_variant = new_enum_field(
            "some value",
            SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD,
        );

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            context_variant,
            directive_variant,
            value_variant,
        ]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert!(matches!(result,
            Err(CommonError::MissingNFTDataField { ref field })
                if field == "Domain record domain id"
        ));
    }

    #[test]
    fn test_missing_context_field() {
        // Build a tuple without the context field.
        let domain_id_variant = new_nf_local_id_field(
            "[9a5fb8db4539384dfe275647bfef559e]",
            SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
        );
        let directive_variant =
            new_enum_field("*", SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD);
        let value_variant = new_enum_field(
            "some value",
            SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD,
        );

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            domain_id_variant,
            directive_variant,
            value_variant,
        ]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert!(matches!(result,
            Err(CommonError::MissingNFTDataField { ref field })
                if field == "Domain record docket context"
        ));
    }

    #[test]
    fn test_missing_directive_field() {
        // Build a tuple without the directive field.
        let domain_id_variant = new_nf_local_id_field(
            "[9a5fb8db4539384dfe275647bfef559e]",
            SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
        );
        let context_variant = new_string_field(
            &DocketContext::Receivers.to_string(),
            SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD,
        );
        let value_variant = new_enum_field(
            "some value",
            SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD,
        );

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            domain_id_variant,
            context_variant,
            value_variant,
        ]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert!(matches!(result,
            Err(CommonError::MissingNFTDataField { ref field })
                if field == "Domain record docket directive"
        ));
    }

    #[test]
    fn test_missing_value_field() {
        // Build a tuple without the value field.
        let domain_id_variant = new_nf_local_id_field(
            "[9a5fb8db4539384dfe275647bfef559e]",
            SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
        );
        let context_variant = new_string_field(
            &DocketContext::Receivers.to_string(),
            SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD,
        );
        let directive_variant =
            new_enum_field("*", SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD);

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            domain_id_variant,
            context_variant,
            directive_variant,
        ]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert!(matches!(result,
            Err(CommonError::MissingNFTDataField { ref field })
                if field == "Domain record value"
        ));
    }

    #[test]
    fn test_invalid_docket_context() {
        // Build a tuple with an invalid docket context.
        let domain_id_variant = new_nf_local_id_field(
            "[9a5fb8db4539384dfe275647bfef559e]",
            SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD,
        );
        let context_variant = new_string_field(
            "invalid_context",
            SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD,
        );
        let directive_variant =
            new_enum_field("*", SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD);
        let value_variant = new_enum_field(
            "some value",
            SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD,
        );

        let tuple = ProgrammaticScryptoSborValueTuple::new(vec![
            domain_id_variant,
            context_variant,
            directive_variant,
            value_variant,
        ]);
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Tuple(tuple),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert!(result.is_err());
    }
}
