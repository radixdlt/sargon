pub mod identified_vec_via;
mod profilesnapshot_version;
pub mod v100;

pub use profilesnapshot_version::*;

pub(crate) use uniffi::{Enum, Error, Object, Record};

uniffi::include_scaffolding!("profile");
