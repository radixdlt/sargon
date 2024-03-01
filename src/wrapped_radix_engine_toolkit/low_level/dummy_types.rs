use crate::prelude::*;

macro_rules! dummy_sargon {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name:ident
    ) => {
        $(
            #[doc = $expr]
        )*
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
