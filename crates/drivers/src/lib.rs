mod drivers;

uniffi::remote_type!(Uuid, common);
uniffi::remote_type!(Timestamp, common);
uniffi::remote_type!(Url, common);

pub mod prelude {

    pub use crate::drivers::*;

    pub use common::prelude::*;
    pub use profile::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("drivers");
