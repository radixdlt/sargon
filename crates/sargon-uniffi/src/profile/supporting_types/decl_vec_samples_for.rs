use crate::prelude::*;

macro_rules! decl_vec_samples_for {
    (
        $(
            #[doc = $expr: expr]
        )*
        $collection_type: ident,
        $element_type: ident
    ) => {
        paste! {
            use sargon::$collection_type as [< Internal $collection_type >];
            
            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample >]() -> Vec<$element_type> {
                [< Internal $collection_type >]::sample().into_type()
            }

            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample_other >]() -> Vec<$element_type> {
                [< Internal $collection_type >]::sample_other().into_type()
            }
		}
	};
}

pub(crate) use decl_vec_samples_for;
