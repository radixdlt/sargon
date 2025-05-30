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
                    .get_non_fungible_local_id_field(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD)
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain record domain id".to_owned() })?;

                    
                let context_str = tuple
                    .fields
                    .get_string_field(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD)
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain record docket context".to_owned() })?;

                let directive_str = tuple
                    .fields
                    .get_enum_field(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD)
                    .and_then(|field| field.fields.first_string_field())
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain record docket directive".to_owned() })?;

                let value = tuple
                    .fields
                    .get_enum_field(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD)
                    .and_then(|field| field.fields.first().cloned())
                    .ok_or(CommonError::MissingNFTDataField { field: "Domain record value".to_owned() })?;

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
mod tests {
    use super::*;

    // Helper: Build a valid ScryptoSborValue for RecordDetails conversion.
    fn valid_record_details_value() -> ScryptoSborValue {
        // Domain id field – using a string variant which is then interpreted as a NonFungibleLocalId.
        let mut domain_id_field = ProgrammaticScryptoSborValueNonFungibleLocalId::new("[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        domain_id_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD.to_owned());
        let domain_id_variant = ProgrammaticScryptoSborValue::NonFungibleLocalId(domain_id_field);

        // Context field.
        let mut context_field = ProgrammaticScryptoSborValueString::new(DocketContext::Receivers.to_string());
        context_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD.to_owned());
        let context_variant = ProgrammaticScryptoSborValue::String(context_field);

        // Directive field: as an enum whose inner fields return a string.
        let inner_directive = {
            let mut inner = ProgrammaticScryptoSborValueString::new("*".to_owned());
            // No need to set field_name on the inner value.
            inner
        };
        let directive_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_directive)], "dummy_variant".to_owned());
        let mut directive_enum = directive_enum;
        directive_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD.to_owned());
        let directive_variant = ProgrammaticScryptoSborValue::Enum(directive_enum);
        
        // Value field: as an enum holding a simple string.
        let inner_value = ProgrammaticScryptoSborValueString::new("some value".to_owned());
        let value_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_value)], "dummy_variant".to_owned());
        let mut value_enum = value_enum;
        value_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD.to_owned());
        let value_variant = ProgrammaticScryptoSborValue::Enum(value_enum);

        // Build the tuple containing all four fields. The order does not matter as long as the helper methods find them by field name.
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
        let record = RecordDetails::try_from(scrypto_value).map_err(|e| {
            panic!("Failed to convert ScryptoSborValue to RecordDetails: {:?}", e);
        }).unwrap();
        // Check domain_id conversion (assuming NonFungibleLocalId wraps a string)
        assert_eq!(record.domain_id.to_string(), "[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        // Check docket context conversion – DocketContext::from_str is used in conversion.
        let expected_context = DocketContext::Receivers;
        assert_eq!(record.context, expected_context);
        // Check the directive is correctly constructed.
        let expected_directive = Directive::new("*".to_owned());
        assert_eq!(record.directive, expected_directive);

        if let ProgrammaticScryptoSborValue::String(inner_str) = record.value {
            assert_eq!(inner_str.value, "some value".to_owned());
        } else {
            panic!("Value field is not a string");
        }
    }

    #[test]
    fn test_try_from_unexpected_format() {
        // Provide a ScryptoSborValue with a non-tuple variant.
        // For example, a simple Bool (assuming such a variant exists in your codebase).
        let scrypto_value = ScryptoSborValue {
            programmatic_json: ProgrammaticScryptoSborValue::Bool(ProgrammaticScryptoSborValueBool::default()),
        };
        let result = RecordDetails::try_from(scrypto_value);
        assert_eq!(result, Err(CommonError::UnexpectedNFTDataFormat));
    }

    #[test]
    fn test_missing_domain_id() {
        // Build a tuple without a domain_id field.
        // Only include context, directive, and value.
        let mut context_field = ProgrammaticScryptoSborValueString::new(DocketContext::Receivers.to_string());
        context_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD.to_owned());
        let context_variant = ProgrammaticScryptoSborValue::String(context_field);
        
        let inner_directive = ProgrammaticScryptoSborValueString::new("*".to_owned());
        let directive_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_directive)], "dummy_variant".to_owned());
        let mut directive_enum = directive_enum;
        directive_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD.to_owned());
        let directive_variant = ProgrammaticScryptoSborValue::Enum(directive_enum);
        
        let inner_value = ProgrammaticScryptoSborValueString::new("some value".to_owned());
        let value_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_value)], "dummy_variant".to_owned());
        let mut value_enum = value_enum;
        value_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD.to_owned());
        let value_variant = ProgrammaticScryptoSborValue::Enum(value_enum);
        
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
        // Build a tuple without a context field.
        let mut domain_id_field = ProgrammaticScryptoSborValueNonFungibleLocalId::new("[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        domain_id_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD.to_owned());
        let domain_id_variant = ProgrammaticScryptoSborValue::NonFungibleLocalId(domain_id_field);

        let inner_directive = ProgrammaticScryptoSborValueString::new("*".to_owned());
        let directive_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_directive)], "dummy_variant".to_owned());
        let mut directive_enum = directive_enum;
        directive_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD.to_owned());
        let directive_variant = ProgrammaticScryptoSborValue::Enum(directive_enum);

        let inner_value = ProgrammaticScryptoSborValueString::new("some value".to_owned());
        let value_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_value)], "dummy_variant".to_owned());
        let mut value_enum = value_enum;
        value_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD.to_owned());
        let value_variant = ProgrammaticScryptoSborValue::Enum(value_enum);

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
        // Build a tuple without a directive field.
        let mut domain_id_field = ProgrammaticScryptoSborValueNonFungibleLocalId::new("[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        domain_id_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD.to_owned());
        let domain_id_variant = ProgrammaticScryptoSborValue::NonFungibleLocalId(domain_id_field);

        let mut context_field = ProgrammaticScryptoSborValueString::new(DocketContext::Receivers.to_string());
        context_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD.to_owned());
        let context_variant = ProgrammaticScryptoSborValue::String(context_field);
        
        let inner_value = ProgrammaticScryptoSborValueString::new("some value".to_owned());
        let value_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_value)], "dummy_variant".to_owned());
        let mut value_enum = value_enum;
        value_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD.to_owned());
        let value_variant = ProgrammaticScryptoSborValue::Enum(value_enum);

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
        // Build a tuple without a value field.
        let mut domain_id_field = ProgrammaticScryptoSborValueNonFungibleLocalId::new("[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        domain_id_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD.to_owned());
        let domain_id_variant = ProgrammaticScryptoSborValue::NonFungibleLocalId(domain_id_field);

        let mut context_field = ProgrammaticScryptoSborValueString::new(DocketContext::Receivers.to_string());
        context_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD.to_owned());
        let context_variant = ProgrammaticScryptoSborValue::String(context_field);

        let inner_directive = ProgrammaticScryptoSborValueString::new("*".to_owned());
        let directive_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_directive)], "dummy_variant".to_owned());
        let mut directive_enum = directive_enum;
        directive_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD.to_owned());
        let directive_variant = ProgrammaticScryptoSborValue::Enum(directive_enum);

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

    // Optionally, if DocketContext::from_str can fail on an invalid input, test that as well.
    #[test]
    fn test_invalid_docket_context() {
        // Build a tuple with an invalid docket context.
        let mut domain_id_field = ProgrammaticScryptoSborValueNonFungibleLocalId::new("[9a5fb8db4539384dfe275647bfef559e]".to_owned());
        domain_id_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOMAIN_ID_FIELD.to_owned());
        let domain_id_variant = ProgrammaticScryptoSborValue::NonFungibleLocalId(domain_id_field);

        // Use an invalid string for docket context (assuming that DocketContext::from_str will reject it).
        let mut context_field = ProgrammaticScryptoSborValueString::new("invalid_context".to_owned());
        context_field.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_CONTEXT_FIELD.to_owned());
        let context_variant = ProgrammaticScryptoSborValue::String(context_field);
        
        let inner_directive = ProgrammaticScryptoSborValueString::new("*".to_owned());
        let directive_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_directive)], "dummy_variant".to_owned());
        let mut directive_enum = directive_enum;
        directive_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_DIRECTIVE_FIELD.to_owned());
        let directive_variant = ProgrammaticScryptoSborValue::Enum(directive_enum);
        
        let inner_value = ProgrammaticScryptoSborValueString::new("some value".to_owned());
        let value_enum = ProgrammaticScryptoSborValueEnum::new(vec![ProgrammaticScryptoSborValue::String(inner_value)], "dummy_variant".to_owned());
        let mut value_enum = value_enum;
        value_enum.field_name = Some(SCRYPTO_SBOR_RECORD_DOCKET_VALUE_FIELD.to_owned());
        let value_variant = ProgrammaticScryptoSborValue::Enum(value_enum);
        
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
        // Expect an error because the docket context is invalid.
        assert!(result.is_err());
    }
}