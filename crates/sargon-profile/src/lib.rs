#![allow(trivial_bounds)]
#![allow(incomplete_features)]
#![feature(trivial_bounds)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]

mod encrypted_profile;
mod profilesnapshot_version;
mod samples;
mod supporting_types;
mod v100;

pub mod prelude {

    pub use identified_vec_of::prelude::*;
    pub use sargon_addresses::prelude::*;
    pub use sargon_core_entity::prelude::*;
    pub use sargon_core_host_info::prelude::*;
    pub use sargon_core_misc::prelude::*;
    pub use sargon_factors::prelude::*;
    pub use sargon_has_sample_values::prelude::*;
    pub use sargon_hierarchical_deterministic::prelude::*;
    pub use sargon_profile_app_preferences::prelude::*;
    pub use sargon_transaction_models::prelude::*;

    pub use crate::encrypted_profile::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::samples::*;
    pub use crate::supporting_types::*;
    pub use crate::v100::*;

    pub(crate) use enum_as_inner::EnumAsInner;

    pub(crate) use serde::{
        ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
    };

    #[cfg(test)]
    pub(crate) use serde_json::json;

    #[cfg(test)]
    pub(crate) use std::collections::HashSet;
}
