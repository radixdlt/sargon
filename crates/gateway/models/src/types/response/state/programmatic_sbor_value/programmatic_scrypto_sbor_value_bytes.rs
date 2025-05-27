

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueBytes {

    pub element_kind: ProgrammaticScryptoSborValueKind,

    pub element_type_name: Option<String>,
    /// The name of the field which hosts this value. This property is only included if this value is a child of a `Tuple` or `Enum` with named fields. This property is ignored when the value is used as an input to the API. 

    pub field_name: Option<String>,
    /// Hex-encoded binary blob.

    pub hex: String,
    /// The name of the type of this value. This is only output when a schema is present and the type has a name. This property is ignored when the value is used as an input to the API. 

    pub type_name: Option<String>,
}

impl ProgrammaticScryptoSborValueBytes {
    pub fn new(element_kind: ProgrammaticScryptoSborValueKind, hex: String) -> ProgrammaticScryptoSborValueBytes {
        ProgrammaticScryptoSborValueBytes {
            element_kind,
            element_type_name: None,
            field_name: None,
            hex,
            type_name: None,
        }
    }
}

