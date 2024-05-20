mod bios;
mod clients;
mod drivers;
mod sargon_os;
mod subsystems;

pub mod prelude {
    pub use crate::bios::*;
    pub use crate::clients::*;
    pub use crate::drivers::*;
    pub use crate::sargon_os::*;
    pub use crate::subsystems::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("sargon");
