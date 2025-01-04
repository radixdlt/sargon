#![feature(async_closure)]

mod collector;
mod host_interaction;

#[cfg(test)]
mod tests;

mod derivation_testing;

#[cfg(test)]
pub(crate) use tests::*;

pub mod prelude {

    pub(crate) use sargon_addresses::prelude::*;
    pub(crate) use sargon_core_collections::prelude::Just;
    pub(crate) use sargon_core_error::prelude::*;
    pub(crate) use sargon_factors::prelude::*;
    pub(crate) use sargon_factors_supporting_types::prelude::*;
    pub(crate) use sargon_hierarchical_deterministic::prelude::*;

    pub use crate::collector::*;
    pub use crate::derivation_testing::*;
    pub use crate::host_interaction::*;

    pub(crate) use std::sync::{Arc, RwLock};

    pub(crate) use itertools::Itertools;
    pub(crate) use log::*;

    pub(crate) use indexmap::{IndexMap, IndexSet};
    pub(crate) use std::collections::{HashMap, HashSet};
}

pub use prelude::*;
