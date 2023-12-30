use crate::UniffiCustomTypeConverter;
use uuid::Uuid;

impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Uuid::try_parse(val.as_str()).map_err(|e| e.into())
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}
