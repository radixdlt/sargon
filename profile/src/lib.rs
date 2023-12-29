mod hierarchical_deterministic;
mod identified_vec_via;
mod profilesnapshot_version;
mod v100;
mod wallet_kit_common;

pub mod prelude {

    pub(crate) use thiserror::Error as ThisError;

    pub use crate::hierarchical_deterministic::*;
    pub use crate::identified_vec_via::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::v100::*;
    pub use crate::wallet_kit_common::*;
}

pub use prelude::*;
pub use url::Url;
pub use uuid::Uuid;

// Use `url::Url` as a custom type, with `String` as the Builtin
impl UniffiCustomTypeConverter for Url {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Url::parse(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

uniffi::include_scaffolding!("radix_wallet_kit");
