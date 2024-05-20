mod modules;

pub mod prelude {
    pub use crate::modules::*;

    pub(crate) use core::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("profile");
