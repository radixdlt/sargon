mod drivers;

uniffi::setup_scaffolding!();

uniffi::remote_type!(Uuid, sargoncommon);
uniffi::remote_type!(Timestamp, sargoncommon);
uniffi::remote_type!(Url, sargoncommon);

pub mod prelude {

    pub use crate::drivers::*;

    pub use profile::prelude::*;
    pub use sargoncommon::prelude::Result;
}

pub use prelude::*;
