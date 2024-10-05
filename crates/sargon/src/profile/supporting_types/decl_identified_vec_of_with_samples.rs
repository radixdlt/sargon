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
            $(
                #[doc = $expr]
            )*
            #[allow(non_camel_case_types)]
			pub type $collection_type = IdentifiedVecOf<$element_type>;

            #[cfg(test)]
            mod [< $collection_type:snake _tests >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $collection_type;

                #[test]
                fn test_ids() {
                    assert_eq!(SUT::sample().ids().into_iter().cloned().collect_vec(), SUT::sample().get_all().into_iter().map(|i| i.id()).collect_vec());
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
