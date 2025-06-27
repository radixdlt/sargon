mod collector;

#[cfg(test)]
mod tests;

mod derivation_testing;

#[cfg(test)]
pub(crate) use tests::*;

pub mod prelude {

    pub(crate) use error::prelude::*;
    pub(crate) use factors::prelude::*;

    pub(crate) use hierarchical_deterministic::prelude::*;
    pub(crate) use key_derivation_traits::prelude::*;
    pub use prelude::prelude::*;

    pub use crate::collector::*;

    pub(crate) use indexmap::{IndexMap, IndexSet};
    pub(crate) use log::*;
    pub(crate) use std::collections::HashMap;

    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {
        pub(crate) use crate::derivation_testing::*;
        pub(crate) use cap26_models::prelude::*;
        pub(crate) use core_collections::prelude::Just;
        pub(crate) use itertools::Itertools;
    }
}

pub use prelude::*;
