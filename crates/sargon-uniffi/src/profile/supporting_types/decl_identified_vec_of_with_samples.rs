use crate::prelude::*;

macro_rules! decl_identified_vec_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $collection_type: ident,
        $element_type: ident
    ) => {
        paste! {
            use sargon::$collection_type as [< Internal $collection_type >];

            $(
                #[doc = $expr]
            )*
            #[allow(non_camel_case_types)]
			pub type $collection_type = Vec<$element_type>;

            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample >]() -> $collection_type {
                [< Internal $collection_type >]::sample().into_vec()
            }

            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample_other >]() -> $collection_type {
                [< Internal $collection_type >]::sample_other().into_vec()
            }
		}
	};
    (
        $(
            #[doc = $expr: expr]
        )*
        $element_type: ident
    ) => {
		paste! {
			decl_identified_vec_of!(
				$(
                    #[doc = $expr]
                )*
				[< $element_type s>],
				$element_type
			);
		}
	};
}

pub(crate) use decl_identified_vec_of;
