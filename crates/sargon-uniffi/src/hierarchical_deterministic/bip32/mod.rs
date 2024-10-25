mod hardened;
mod hd_path;
mod hd_path_component;
mod key_space;
mod securified;
mod u30;
mod u31;
mod unhardened;
mod unsecurified;
mod unsecurified_hardened;

pub use hardened::*;
pub use hd_path::*;
pub use hd_path_component::*;
pub use key_space::*;
pub use securified::*;
pub use u30::*;
pub use u31::*;
pub use unhardened::*;
pub use unsecurified::*;
pub use unsecurified_hardened::*;

use sargon::{GLOBAL_OFFSET_HARDENED, GLOBAL_OFFSET_HARDENED_SECURIFIED};

#[uniffi::export]
pub fn bip32_constant_global_offset_hardened() -> u32 {
    GLOBAL_OFFSET_HARDENED
}
#[uniffi::export]
pub fn bip32_constant_global_offset_securified() -> u32 {
    GLOBAL_OFFSET_HARDENED_SECURIFIED
}
