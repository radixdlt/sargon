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
