mod access_controller_recovery;
mod external_signatures_collector;
mod pre_authorization_access_controller_roles;
mod sargon_os_signing;

pub mod prelude {
    pub(crate) use crate::access_controller_recovery::*;
    pub(crate) use crate::external_signatures_collector::*;
    pub(crate) use crate::pre_authorization_access_controller_roles::*;
    pub use crate::sargon_os_signing::*;

    pub(crate) use error::prelude::*;
    pub(crate) use manifests::prelude::*;
    pub use prelude::prelude::*;
    pub(crate) use profile_supporting_types::prelude::*;
    pub(crate) use radix_engine_interface::prelude::AccessRule as ScryptoAccessRule;
    pub(crate) use sargon_os::prelude::*;
    pub(crate) use sargon_os_factors::prelude::{
        AccessControllerRuleSet, AccessRequirement, GatewayClient,
        OsMfaSigningKeys,
    };
    pub(crate) use signatures_collector::prelude::*;

    pub(crate) use signing_traits::prelude::*;

    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {
        pub(crate) use clients::prelude::*;
        pub(crate) use factor_instances_provider::prelude::*;
        pub(crate) use interactors::prelude::*;
        pub(crate) use key_derivation_traits::prelude::*;
        pub(crate) use profile_logic::prelude::*;
        pub(crate) use radix_connect_models::prelude::*;
    }
}

pub use prelude::*;
