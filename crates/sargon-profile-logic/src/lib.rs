mod logic;
mod tests;

pub mod prelude {
    pub use crate::logic::*;

    pub use sargon_profile::prelude::*;
    pub use sargon_profile_supporting_types::prelude::*;

    pub use itertools::*;
}

pub use prelude::*;
