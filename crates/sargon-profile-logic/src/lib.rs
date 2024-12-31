mod logic;
mod supporting_types;
mod tests;

pub mod prelude {
    pub use crate::logic::*;
    pub use crate::supporting_types::*;

    pub use sargon_profile::prelude::*;

    pub use itertools::Itertools;
}

pub use prelude::*;
