#![allow(internal_features)]
#![feature(core_intrinsics)]

mod common_error;

pub mod prelude {

    pub use crate::common_error::*;

    pub(crate) use log::*;
}
