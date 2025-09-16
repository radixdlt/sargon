use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgrammaticScryptoSborValueMap {
    pub entries: Vec<ProgrammaticScryptoSborValueMapEntry>,
    /// The name of the field which hosts this value. This property is only included if this value is a child of a `Tuple` or `Enum` with named fields. This property is ignored when the value is used as an input to the API.
    pub field_name: Option<String>,

    pub key_kind: ProgrammaticScryptoSborValueKind,

    pub key_type_name: Option<String>,
    /// The name of the type of this value. This is only output when a schema is present and the type has a name. This property is ignored when the value is used as an input to the API.
    pub type_name: Option<String>,

    pub value_kind: ProgrammaticScryptoSborValueKind,

    pub value_type_name: Option<String>,
}

impl ProgrammaticScryptoSborValueMap {
    pub fn new(
        entries: Vec<ProgrammaticScryptoSborValueMapEntry>,
        key_kind: ProgrammaticScryptoSborValueKind,
        value_kind: ProgrammaticScryptoSborValueKind,
    ) -> ProgrammaticScryptoSborValueMap {
        ProgrammaticScryptoSborValueMap {
            entries,
            field_name: None,
            key_kind,
            key_type_name: None,
            type_name: None,
            value_kind,
            value_type_name: None,
        }
    }
}
