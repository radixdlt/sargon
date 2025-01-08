use crate::prelude::*;

/// A macro that generates a XYZVersion type, which is a typed version of `u64`.
macro_rules! decl_version_type {
    ($name:ident) => {
        paste! {
            use sargon::[<$name Version>] as [<Internal $name Version>];

            uniffi::custom_newtype!([<$name Version>], u64);

            #[derive(Clone, PartialEq, InternalConversion)]
            pub struct [<$name Version>](u64);
        }
    };
}

pub(crate) use decl_version_type;
