#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]

mod factor_list_kind;
mod role_kind;
mod roles_matrices_structures;
mod threshold;

pub mod prelude {
    pub use crate::factor_list_kind::*;
    pub use crate::role_kind::*;
    pub use crate::roles_matrices_structures::*;

    pub use addresses::prelude::*;
    pub use cap26_models::prelude::*;
    pub use core_collections::prelude::{Just, JustKV};
    pub use core_utils::prelude::*;
    pub use factors::prelude::*;
    pub use has_sample_values::prelude::*;
    pub use hierarchical_deterministic::prelude::*;
    pub use identified_vec_of::prelude::*;
    pub use network::prelude::*;
    pub use next_derivation_index_ephemeral::prelude::*;
    pub use short_string::prelude::*;

    pub(crate) use radix_engine_interface::{
        blueprints::{
            access_controller::RuleSet as ScryptoRuleSet,
            resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible,
        },
        prelude::{
            AccessRule as ScryptoAccessRule,
            BasicRequirement as ScryptoBasicRequirement,
            CompositeRequirement as ScryptoCompositeRequirement,
        },
    };

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use serde::{Deserialize, Serialize};
    #[cfg(test)]
    pub(crate) use serde_json::json;

    pub(crate) use std::collections::HashSet;

    pub(crate) use std::str::FromStr;

    pub(crate) use std::sync::RwLock;
}
