mod persona_data;
mod shared_persona_data;

pub mod prelude {
    pub use crate::persona_data::*;
    pub use crate::shared_persona_data::*;

    pub(crate) use addresses::prelude::*;
    pub(crate) use core_misc::prelude::*;
    pub(crate) use has_sample_values::prelude::*;
    pub(crate) use identified_vec_of::prelude::*;
    pub(crate) use short_string::prelude::*;

    pub(crate) use serde::{Deserialize, Serialize};

    pub(crate) use std::str::FromStr;
    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {

        pub(crate) use std::collections::HashSet;

        pub(crate) use serde_json::json;
    }
}

pub use prelude::*;
