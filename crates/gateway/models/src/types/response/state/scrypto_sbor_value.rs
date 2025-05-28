use crate::*;

use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScryptoSborValue {
    pub programmatic_json: ProgrammaticScryptoSborValue,
}
pub trait ScryptoSborValueFieldExtraction {
    fn get_string_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueString>;
    fn get_enum_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueEnum>;
    fn get_reference_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueReference>;
    fn first_reference_field(
        &self,
    ) -> Option<ProgrammaticScryptoSborValueReference>;
}

impl ScryptoSborValueFieldExtraction for Vec<ProgrammaticScryptoSborValue> {
    fn get_string_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueString> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::String(str_sbor_value) => {
                if str_sbor_value.field_name == Some(name.to_owned()) {
                    return Some(str_sbor_value.clone());
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    fn get_enum_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueEnum> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Enum(enum_sbor_value) => {
                if enum_sbor_value.field_name == Some(name.to_owned()) {
                    return Some(enum_sbor_value.clone());
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    fn get_reference_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueReference> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Reference(reference_sbor_value) => {
                if reference_sbor_value.field_name == Some(name.to_owned()) {
                    return Some(reference_sbor_value.clone());
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    fn first_reference_field(
        &self,
    ) -> Option<ProgrammaticScryptoSborValueReference> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Reference(reference_sbor_value) => {
                return Some(reference_sbor_value.clone())
            }
            _ => return None,
        })
    }
}
