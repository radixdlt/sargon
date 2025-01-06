mod log_system;

pub mod prelude {
    pub use crate::log_system::*;

    pub(crate) use drivers::prelude::*;

    pub(crate) use log::*;

    pub(crate) use std::sync::{Arc, RwLock};

    #[cfg(test)]
    mod testing {
        pub(crate) use itertools::Itertools;
    }
    #[cfg(test)]
    pub(crate) use testing::*;
}

pub use prelude::*;
