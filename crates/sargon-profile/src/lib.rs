#![allow(trivial_bounds)]
#![allow(incomplete_features)]
#![feature(trivial_bounds)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]

mod encrypted;
mod mfa;
mod profilesnapshot_version;
mod samples;
mod supporting_types;
mod v100;

pub mod prelude {

    pub use identified_vec_of::prelude::*;
    pub use sargon_addresses::prelude::*;
    pub use sargon_core::prelude::*;
    pub use sargon_factors::prelude::*;
    pub use sargon_hierarchical_deterministic::prelude::*;
    pub use sargon_transaction_models::prelude::*;

    pub use crate::encrypted::*;
    pub use crate::mfa::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::samples::*;
    pub use crate::supporting_types::*;
    pub use crate::v100::*;

    pub(crate) use radix_engine_interface::{
        blueprints::{
            access_controller::{
                RecoveryProposal as ScryptoRecoveryProposal,
                RuleSet as ScryptoRuleSet,
            },
            account::{
                DefaultDepositRule as ScryptoDefaultDepositRule,
                ResourcePreference as ScryptoResourcePreference,
                ACCOUNT_SECURIFY_IDENT as SCRYPTO_ACCOUNT_SECURIFY_IDENT,
            },
            identity::IDENTITY_SECURIFY_IDENT as SCRYPTO_IDENTITY_SECURIFY_IDENT,
            resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible,
        },
        prelude::{
            AccessRule as ScryptoAccessRule,
            BasicRequirement as ScryptoBasicRequirement,
            CompositeRequirement as ScryptoCompositeRequirement,
            FungibleResourceRoles as ScryptoFungibleResourceRoles,
            MetadataInit as ScryptoMetadataInit,
            MetadataValue as ScryptoMetadataValue,
            ModuleConfig as ScryptoModuleConfig,
            NonFungibleResourceRoles as ScryptoNonFungibleResourceRoles,
            OwnerRole as ScryptoOwnerRole,
            RoleAssignmentInit as ScryptoRoleAssignmentInit,
            ToMetadataEntry as ScryptoToMetadataEntry,
            UncheckedUrl as ScryptoUncheckedUrl,
        },
    };
}
