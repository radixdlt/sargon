mod identified_vec_via;
mod profilesnapshot_version;
mod v100;

pub mod prelude {
    pub use crate::identified_vec_via::*;
    pub use crate::profilesnapshot_version::*;
    pub use crate::v100::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("profile");
