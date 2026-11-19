//! Sargon's model of on-ledger access rules: what is required to pass a rule,
//! such as a role of an access controller.
//!
//! It describes any rule the engine accepts, whoever wrote it: the wallet, a
//! dApp or another wallet. Interpretations of a rule, such as the wallet's
//! security shields, are built on top of it.

mod access_controller_rule_set;
mod access_controller_rule_set_for_display;
mod access_requirement;
mod access_requirement_for_display;
mod access_requirement_from_scrypto;
mod access_requirement_simplification;
mod auth_zone;

pub mod prelude {
    pub use crate::access_controller_rule_set::*;
    pub use crate::access_controller_rule_set_for_display::*;
    pub use crate::access_requirement::*;
    pub use crate::access_requirement_for_display::*;
    pub use crate::auth_zone::*;

    pub(crate) use addresses::prelude::*;
    pub(crate) use factors::prelude::*;
    pub(crate) use has_sample_values::prelude::*;
    pub(crate) use network::prelude::*;
    pub(crate) use numeric::prelude::*;

    pub(crate) use std::collections::HashSet;

    pub(crate) use radix_common::constants::{
        ED25519_SIGNATURE_RESOURCE, SECP256K1_SIGNATURE_RESOURCE,
    };
    pub(crate) use radix_engine_interface::blueprints::access_controller::RuleSet as ScryptoRuleSet;
    pub(crate) use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;
    pub(crate) use radix_engine_interface::prelude::{
        AccessRule as ScryptoAccessRule,
        BasicRequirement as ScryptoBasicRequirement,
        CompositeRequirement as ScryptoCompositeRequirement,
    };
}

pub use prelude::*;
