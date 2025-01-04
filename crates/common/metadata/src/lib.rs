mod metadata;

pub mod prelude {
    pub use crate::metadata::*;

    pub(crate) use radix_engine_interface::prelude::{
        MetadataValue as ScryptoMetadataValue,
        ToMetadataEntry as ScryptoToMetadataEntry,
    };

    pub(crate) use serde::{Deserialize, Serialize};

    #[cfg(test)]
    pub(crate) use std::str::FromStr;
}
