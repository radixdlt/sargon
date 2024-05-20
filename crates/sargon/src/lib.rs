#![feature(async_closure)]
#![feature(let_chains)]

mod bios;
mod clients;
mod sargon_os;
mod subsystems;

pub mod prelude {
    pub use crate::bios::*;
    pub use crate::clients::*;
    pub use crate::sargon_os::*;
    pub use crate::subsystems::*;

    pub(crate) use common::prelude::*;
    pub(crate) use drivers::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("sargon");
