use crate::*;

use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScryptoSborValue {
    pub programmatic_json: ProgrammaticScryptoSborValue,
}
pub trait ScryptoSborValueFieldExtraction {
    fn get_string_field(&self, name: &str) -> Option<String>;
    fn get_enum_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueEnum>;
    fn get_reference_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueReference>;
    fn get_non_fungible_local_id_field(
        &self,
        name: &str,
    ) -> Option<NonFungibleLocalId>;

    fn first_string_field(&self) -> Option<String>;

    fn first_reference_field(
        &self,
    ) -> Option<ProgrammaticScryptoSborValueReference>;
}

impl ScryptoSborValueFieldExtraction for Vec<ProgrammaticScryptoSborValue> {
    fn get_string_field(&self, name: &str) -> Option<String> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::String(str_sbor_value) => {
                if str_sbor_value.field_name == Some(name.to_owned()) {
                    Some(str_sbor_value.value.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn get_enum_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueEnum> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Enum(enum_sbor_value) => {
                if enum_sbor_value.field_name == Some(name.to_owned()) {
                    Some(enum_sbor_value.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn get_reference_field(
        &self,
        name: &str,
    ) -> Option<ProgrammaticScryptoSborValueReference> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Reference(reference_sbor_value) => {
                if reference_sbor_value.field_name == Some(name.to_owned()) {
                    Some(reference_sbor_value.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn first_reference_field(
        &self,
    ) -> Option<ProgrammaticScryptoSborValueReference> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::Reference(reference_sbor_value) => {
                Some(reference_sbor_value.clone())
            }
            _ => None,
        })
    }

    fn first_string_field(&self) -> Option<String> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::String(string_sbor_value) => {
                Some(string_sbor_value.value.clone())
            }
            _ => None,
        })
    }

    fn get_non_fungible_local_id_field(
        &self,
        name: &str,
    ) -> Option<NonFungibleLocalId> {
        self.iter().find_map(|field| match field {
            ProgrammaticScryptoSborValue::NonFungibleLocalId(
                non_fungible_local_id_sbor_value,
            ) => {
                if non_fungible_local_id_sbor_value.field_name
                    == Some(name.to_owned())
                {
                    NonFungibleLocalId::from_str(
                        &non_fungible_local_id_sbor_value.value,
                    )
                    .ok()
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}
