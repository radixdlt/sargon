mod is_network_aware;
mod network_id;

pub mod prelude {
    pub use crate::is_network_aware::*;
    pub use crate::network_id::*;

    #[cfg(test)]
    pub(crate) use sargon_core_assert_json::prelude::*;
    pub(crate) use sargon_has_sample_values::prelude::*;

    pub(crate) use radix_common::network::NetworkDefinition as ScryptoNetworkDefinition;
    pub(crate) use strum::FromRepr;

    #[cfg(test)]
    pub(crate) use serde_json::json;
    #[cfg(test)]
    pub(crate) use std::collections::BTreeSet;
}
