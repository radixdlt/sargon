mod display_name;
mod email_address;
mod short_string;

pub mod prelude {
    pub use crate::display_name::*;
    pub use crate::email_address::*;
    pub use crate::short_string::*;

    #[cfg(test)]
    pub(crate) use sargon_core_assert_json::prelude::*;
    pub(crate) use sargon_has_sample_values::prelude::*;

    pub(crate) use std::str::FromStr;

    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use serde_with::{DeserializeFromStr, SerializeDisplay};

    #[cfg(test)]
    pub(crate) use serde_json::json;
}
