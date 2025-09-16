use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueReference {
    /// The name of the field which hosts this value. This property is only included if this value is a child of a `Tuple` or `Enum` with named fields. This property is ignored when the value is used as an input to the API.
    pub field_name: Option<String>,
    /// The name of the type of this value. This is only output when a schema is present and the type has a name. This property is ignored when the value is used as an input to the API.
    pub type_name: Option<String>,
    /// Bech32m-encoded human readable version of the address.
    pub value: String,
}

impl ProgrammaticScryptoSborValueReference {
    pub fn new(value: String) -> ProgrammaticScryptoSborValueReference {
        ProgrammaticScryptoSborValueReference {
            field_name: None,
            type_name: None,
            value,
        }
    }
}
