mod interaction_id;
mod p2p_links;
mod wallet_account;
mod wallet_interaction;
mod wallet_persona;
mod well_known_client;

#[allow(dead_code)]
mod mobile;

pub mod prelude {
    // RE-EXPORT MODULES
    pub use crate::interaction_id::*;
    pub use crate::mobile::*;
    pub use crate::p2p_links::*;
    pub use crate::wallet_account::*;
    pub use crate::wallet_interaction::*;
    pub use crate::wallet_persona::*;
    pub use crate::well_known_client::*;

    // INTERNAL DEPENDENCIES
    pub(crate) use addresses::prelude::*;
    pub(crate) use bytes::prelude::*;
    pub(crate) use core_misc::prelude::*;

    pub(crate) use ecc::prelude::*;
    pub(crate) use entity_foundation::prelude::*;
    pub(crate) use hash::prelude::*;
    pub(crate) use identified_vec_of::prelude::*;
    pub(crate) use prelude::prelude::*;
    pub(crate) use profile_persona_data::prelude::*;
    pub(crate) use profile_security_structures::prelude::*;
    pub(crate) use radix_connect_models::prelude::*;

    pub(crate) use transaction_models::prelude::*;

    // EXTERNAL DEPENDENCIES
    pub(crate) use serde::{
        de, Deserialize, Deserializer, Serialize, Serializer,
    };
    pub(crate) use serde_with::{serde_as, DisplayFromStr};

    // STD DEPENDENCIES
    pub(crate) use std::collections::HashMap;

    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {
        pub(crate) use drivers::prelude::MockNetworkingDriver;
        pub(crate) use serde_json::json;
        pub(crate) use std::collections::BTreeSet;
    }
}

pub use prelude::*;
