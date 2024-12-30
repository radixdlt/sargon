#![feature(async_closure)]

mod collector;
mod host_interaction;

// #[cfg(test)]
// mod tests;

// mod derivation_testing;

// #[cfg(test)]
// pub(crate) use tests::*;

pub mod prelude {

    pub(crate) use sargon_addresses::prelude::*;
    pub(crate) use sargon_core::prelude::*;
    pub(crate) use sargon_factors::prelude::*;
    pub(crate) use sargon_hierarchical_deterministic::prelude::*;

    pub use crate::collector::*;
    // pub(crate) use crate::derivation_testing::*;
    pub use crate::host_interaction::*;
}

pub use prelude::*;
