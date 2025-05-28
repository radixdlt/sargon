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

impl TryFrom<ScryptoSborValue> for RecordDetails {
    type Error = CommonError;

    fn try_from(value: ScryptoSborValue) -> Result<Self> {
        match value.programmatic_json {
            ProgrammaticScryptoSborValue::Tuple(tuple) => {
                let domain_id = tuple
                    .fields
                    .get_non_fungible_local_id_field("domain_id")
                    .ok_or(CommonError::Unknown)?;
                let context_str = tuple
                    .fields
                    .get_string_field("context")
                    .ok_or(CommonError::Unknown)?;
                let directive_str = tuple
                    .fields
                    .get_enum_field("directive")
                    .and_then(|field| field.fields.first_string_field())
                    .ok_or(CommonError::Unknown)?;

                let value = tuple
                    .fields
                    .get_enum_field("value")
                    .and_then(|field| field.fields.first().cloned())
                    .ok_or(CommonError::Unknown)?;

                let context = DocketContext::from_str(&context_str)?;
                let directive = Directive::new(directive_str);

                Ok(RecordDetails {
                    domain_id,
                    context,
                    directive,
                    value,
                })
            }
            _ => Err(CommonError::Unknown),
        }
    }
}