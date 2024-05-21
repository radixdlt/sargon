#![feature(async_closure)]
#![feature(let_chains)]

mod bios;
mod sargon_os;
mod subsystems;

pub mod prelude {
    pub use crate::bios::*;
    pub use crate::sargon_os::*;
    pub use crate::subsystems::*;

    pub(crate) use clients::prelude::*;
    pub(crate) use common::prelude::*;
    pub(crate) use drivers::prelude::*;
    pub(crate) use hd::prelude::*;
    pub(crate) use profile::prelude::*;
    pub(crate) use transaction::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("sargon");
