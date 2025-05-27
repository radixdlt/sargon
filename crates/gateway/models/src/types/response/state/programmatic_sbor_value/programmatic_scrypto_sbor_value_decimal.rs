

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueDecimal {
    /// The name of the field which hosts this value. This property is only included if this value is a child of a `Tuple` or `Enum` with named fields. This property is ignored when the value is used as an input to the API. 

    pub field_name: Option<String>,
    /// The name of the type of this value. This is only output when a schema is present and the type has a name. This property is ignored when the value is used as an input to the API. 

    pub type_name: Option<String>,

    pub value: String,
}

impl ProgrammaticScryptoSborValueDecimal {
    pub fn new(value: String) -> ProgrammaticScryptoSborValueDecimal {
        ProgrammaticScryptoSborValueDecimal {
            field_name: None,
            type_name: None,
            value,
        }
    }
}

