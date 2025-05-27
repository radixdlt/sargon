
use serde::{Deserialize, Serialize};
use crate::*;


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueArray {
    pub element_kind: ProgrammaticScryptoSborValueKind,
    pub element_type_name: Option<String>,
    pub elements: Vec<ProgrammaticScryptoSborValue>,
    pub field_name: Option<String>,
    pub type_name: Option<String>,
}

impl ProgrammaticScryptoSborValueArray {
    pub fn new(element_kind: ProgrammaticScryptoSborValueKind, elements: Vec<ProgrammaticScryptoSborValue>) -> ProgrammaticScryptoSborValueArray {
        ProgrammaticScryptoSborValueArray {
            element_kind,
            element_type_name: None,
            elements,
            field_name: None,
            type_name: None,
        }
    }
}

