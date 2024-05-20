mod client;
mod endpoints;
mod methods;
mod models;

pub mod prelude {

    pub use crate::client::*;
    pub use crate::endpoints::*;
    pub use crate::methods::*;
    pub use crate::models::*;

    pub use common::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("gateway");
