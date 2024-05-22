uniffi::setup_scaffolding!();

mod clients;

uniffi::remote_type!(Uuid, sargoncommon);
uniffi::remote_type!(Timestamp, sargoncommon);
uniffi::remote_type!(Url, sargoncommon);

pub mod prelude {

    pub use crate::clients::*;

    pub use drivers::prelude::*;
    pub use gateway_models::prelude::*;
    pub use sargoncommon::prelude::Result;
}

pub use prelude::*;
