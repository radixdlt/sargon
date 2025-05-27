use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueBool {
    pub field_name: Option<String>,
    pub type_name: Option<String>,
    pub value: bool,
}

impl ProgrammaticScryptoSborValueBool {
    pub fn new(value: bool) -> ProgrammaticScryptoSborValueBool {
        ProgrammaticScryptoSborValueBool {
            field_name: None,
            type_name: None,
            value,
        }
    }
}
