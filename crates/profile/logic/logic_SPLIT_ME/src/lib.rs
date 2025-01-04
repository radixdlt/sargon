mod logic;
mod tests;

pub mod prelude {
    pub use crate::logic::*;

    pub use profile::prelude::*;
    pub use profile_supporting_types::prelude::*;

    pub use itertools::*;

    pub(crate) use serde::{Deserialize, Serialize};
}

pub use prelude::*;
