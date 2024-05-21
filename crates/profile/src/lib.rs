#![feature(async_closure)]
#![feature(let_chains)]

mod cap26_derivation;
mod encrypted;
mod logic;
mod profilesnapshot_version;
mod supporting_types;
mod v100;

uniffi::remote_type!(Uuid, common);
uniffi::remote_type!(Timestamp, common);
uniffi::remote_type!(Url, common);

pub mod prelude {

    pub use crate::cap26_derivation::*;
    pub use crate::encrypted::*;
    pub use crate::logic::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::supporting_types::*;
    pub use crate::v100::*;

    pub use common::prelude::*;
    pub use hd::prelude::*;
    pub use ret::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("profile");
