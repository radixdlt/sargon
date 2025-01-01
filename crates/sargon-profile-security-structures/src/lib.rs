#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]

mod agnostic_paths;
mod factor_list_kind;
mod role_kind;
mod roles_matrices_structures;

pub mod prelude {
    pub use crate::agnostic_paths::*;
    pub use crate::factor_list_kind::*;
    pub use crate::role_kind::*;
    pub use crate::roles_matrices_structures::*;

    pub use sargon_addresses::prelude::*;
    pub use sargon_core::prelude::*;
    pub use sargon_factors::prelude::*;
    pub use sargon_hierarchical_deterministic::prelude::*;

    pub(crate) use serde::*;

    pub(crate) use radix_engine_interface::prelude::{
        AccessRule as ScryptoAccessRule,
        BasicRequirement as ScryptoBasicRequirement,
        CompositeRequirement as ScryptoCompositeRequirement,
    };

    pub(crate) use radix_engine_interface::blueprints::{
        access_controller::RuleSet as ScryptoRuleSet,
        resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible,
    };
}
