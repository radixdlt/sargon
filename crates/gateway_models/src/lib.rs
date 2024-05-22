uniffi::setup_scaffolding!();

mod models;

pub mod prelude {

    pub use crate::models::*;

    pub use ret::prelude::*;
}

pub use prelude::*;
