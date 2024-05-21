mod clients;

uniffi::remote_type!(Uuid, common);
uniffi::remote_type!(Timestamp, common);
uniffi::remote_type!(Url, common);

pub mod prelude {

    pub use crate::clients::*;

    pub use common::prelude::*;
    pub use drivers::prelude::*;
    pub use gateway_models::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("clients");
