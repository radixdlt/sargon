mod log_system;

pub mod prelude {
    pub use crate::log_system::*;

    pub use prelude::prelude::*;
    pub(crate) use drivers::prelude::*;
    pub(crate) use log::*;


    #[cfg(test)]
    mod testing {
        pub(crate) use itertools::Itertools;
    }
    #[cfg(test)]
    pub(crate) use testing::*;
}

pub use prelude::*;
