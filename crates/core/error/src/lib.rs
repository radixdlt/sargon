#![allow(internal_features)]
#![feature(core_intrinsics)]

mod common_error;
mod secure_storage_access_error_kind;

pub mod prelude {
    pub use crate::common_error::*;
    pub use crate::secure_storage_access_error_kind::*;
}
