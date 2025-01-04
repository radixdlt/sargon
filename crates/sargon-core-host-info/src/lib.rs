mod host_info;

pub mod prelude {
    pub use crate::host_info::*;

    #[cfg(test)]
    pub(crate) use sargon_core_assert_json::prelude::*;
    pub(crate) use sargon_has_sample_values::prelude::*;

    pub(crate) use std::str::FromStr;

    pub(crate) use serde::{Deserialize, Serialize};

    #[cfg(test)]
    pub(crate) use serde_json::json;

    #[cfg(test)]
    pub(crate) use std::collections::HashSet;
}
