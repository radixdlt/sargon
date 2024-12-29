#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

mod decl_identified_vec_of_with_samples;
mod identifiable;
mod identified_vec_of;
mod identified_vec_of_display_debug;
mod identified_vec_of_iterator;
mod identified_vec_of_modify;
mod identified_vec_of_query;
mod identified_vec_of_serde;
mod identified_vec_of_validation_import_export;

#[cfg(test)]
mod user;

pub mod prelude {

    pub use sargon_core_assert_json::prelude::*;
    pub use sargon_core_error::prelude::*;

    pub use crate::decl_identified_vec_of_with_samples::*;
    pub use crate::identifiable::*;
    pub use crate::identified_vec_of::*;
    pub use crate::identified_vec_of_display_debug::*;
    pub use crate::identified_vec_of_iterator::*;
    pub use crate::identified_vec_of_modify::*;
    pub use crate::identified_vec_of_query::*;
    pub use crate::identified_vec_of_serde::*;
    pub use crate::identified_vec_of_validation_import_export::*;

    pub use itertools::Itertools;
    pub use std::fmt::{Debug, Display};
    pub use std::hash::Hash as StdHash;
}

#[cfg(test)]
pub use user::*;

pub use prelude::*;
