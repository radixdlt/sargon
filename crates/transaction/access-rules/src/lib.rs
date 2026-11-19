//! Sargon's model of on-ledger access rules: what is required to pass a rule,
//! such as a role of an access controller.
//!
//! It describes any rule the engine accepts, whoever wrote it: the wallet, a
//! dApp or another wallet. Interpretations of a rule, such as the wallet's
//! security shields, are built on top of it.

mod access_requirement;
mod access_requirement_from_scrypto;

pub mod prelude {
    pub use crate::access_requirement::*;

    pub(crate) use addresses::prelude::*;
    pub(crate) use has_sample_values::prelude::*;
    pub(crate) use network::prelude::*;
    pub(crate) use numeric::prelude::*;

    pub(crate) use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;
    pub(crate) use radix_engine_interface::prelude::{
        AccessRule as ScryptoAccessRule,
        BasicRequirement as ScryptoBasicRequirement,
        CompositeRequirement as ScryptoCompositeRequirement,
    };
}

pub use prelude::*;
