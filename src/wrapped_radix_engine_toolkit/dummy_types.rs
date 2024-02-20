use crate::prelude::*;

macro_rules! dummy_sargon {
    ($struct_name:ident) => {
        #[derive(
            Serialize,
            Deserialize,
            Clone,
            Debug,
            Default,
            PartialEq,
            Eq,
            Hash,
            uniffi::Record,
        )]
        pub struct $struct_name {}
    };
}
