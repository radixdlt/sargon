mod log_system;

pub mod prelude {
    pub use crate::log_system::*;

    pub(crate) use drivers::prelude::*;
    pub(crate) use log::*;
    pub use prelude::prelude::*;
    #[cfg(test)]
    pub(crate) use testing::*;

    #[cfg(test)]
    mod testing {
        pub(crate) use itertools::Itertools;
    }
}

pub use prelude::*;
