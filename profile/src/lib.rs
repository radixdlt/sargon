mod hierarchical_deterministic;
mod identified_vec_via;
mod logic;
mod profilesnapshot_version;
mod v100;
mod wallet;
mod wallet_kit_common;

pub mod prelude {

    pub(crate) use thiserror::Error as ThisError;

    pub use crate::hierarchical_deterministic::*;
    pub use crate::identified_vec_via::*;
    pub use crate::logic::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::v100::*;
    pub use crate::wallet::*;
    pub use crate::wallet_kit_common::*;
}

use iso8601_timestamp::Timestamp;
pub use prelude::*;
pub use url::Url;
pub use uuid::Uuid;

// Use `url::Url` as a custom type, with `String` as the Builtin
#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Url {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Url::parse(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Timestamp {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Timestamp::parse(val.as_str())
            .ok_or_else(|| CommonError::InvalidISO8601String)
            .map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Uuid::try_parse(val.as_str()).map_err(|e| e.into())
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

uniffi::include_scaffolding!("radix_wallet_kit");
