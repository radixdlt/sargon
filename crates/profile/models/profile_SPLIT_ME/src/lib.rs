#![allow(trivial_bounds)]
#![feature(trivial_bounds)]
#![feature(let_chains)]

mod encrypted_profile;
mod profilesnapshot_version;
mod samples;
mod supporting_types;
mod v100;

pub mod prelude {

    pub use crate::encrypted_profile::*;
    pub use crate::profilesnapshot_version::*;

    pub use crate::supporting_types::*;
    pub use crate::v100::*;

    pub use account_for_display::prelude::*;
    pub use addresses::prelude::*;
    pub use core_misc::prelude::*;
    pub use entity_by_address::prelude::*;
    pub use entity_foundation::prelude::*;
    pub use factors::prelude::*;
    pub use has_sample_values::prelude::*;
    pub use hierarchical_deterministic::prelude::*;
    pub use host_info::prelude::*;
    pub use identified_vec_of::prelude::*;
    pub use profile_account::prelude::*;
    pub use profile_account_or_persona::prelude::*;
    pub use profile_app_preferences::prelude::*;
    pub use profile_base_entity::prelude::*;
    pub use profile_persona::prelude::*;
    pub use profile_persona_data::prelude::*;
    pub use transaction_models::prelude::*;

    pub(crate) use serde::{Deserialize, Serialize};

    #[cfg(test)]
    pub(crate) use serde_json::json;
}
