#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

mod agnostic_paths;
mod next_derivation_entity_index_with_ephemeral_offsets;
mod next_derivation_entity_index_with_ephemeral_offsets_for_factor_source;

pub mod prelude {
    pub use crate::agnostic_paths::*;
    pub use crate::next_derivation_entity_index_with_ephemeral_offsets::*;
    pub use crate::next_derivation_entity_index_with_ephemeral_offsets_for_factor_source::*;

    pub(crate) use identified_vec_of::prelude::*;
    pub(crate) use sargon_addresses::prelude::*;
    pub(crate) use sargon_core_network::prelude::*;
    pub(crate) use sargon_factors::prelude::*;
    pub(crate) use sargon_has_sample_values::prelude::*;
    pub(crate) use sargon_hierarchical_deterministic::prelude::*;

    pub(crate) use indexmap::IndexSet;
    pub(crate) use std::collections::HashMap;
    pub(crate) use std::str::FromStr;
    pub(crate) use std::sync::RwLock;

    #[cfg(test)]
    pub(crate) use serde_json::json;
    pub(crate) use serde_with::{DeserializeFromStr, SerializeDisplay};
}
