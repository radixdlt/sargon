mod delete_account;
mod sargon_os_sync_accounts;

pub mod prelude {
    pub use super::delete_account::*;
    pub use super::sargon_os_sync_accounts::*;

    pub(crate) use addresses::prelude::*;
    pub(crate) use error::prelude::*;
    pub(crate) use gateway_client_and_api::prelude::*;
    pub(crate) use manifests::prelude::*;
    pub(crate) use sargon_os::prelude::*;

    pub(crate) use indexmap::IndexMap;
    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {
        pub(crate) use std::sync::Arc;
    }
}

pub use prelude::*;
