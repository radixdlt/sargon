use crate::prelude::*;

macro_rules! decl_ordered_map {
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
			pub type $collection_type = IdentifiedVecOf<$element_type>;

			#[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample >]() -> $collection_type {
                $collection_type::sample()
            }

            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample_other >]() -> $collection_type {
                $collection_type::sample_other()
            }

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

            #[cfg(test)]
            mod [< $collection_type:snake _uniffi_tests >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $collection_type;

                #[test]
                fn hash_of_samples() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            [< new_ $collection_type:snake _sample >](),
                            [< new_ $collection_type:snake _sample_other >](),
                            // duplicates should get removed
                            [< new_ $collection_type:snake _sample >](),
                            [< new_ $collection_type:snake _sample_other >]()
                        ])
                        .len(),
                        2
                    );
                }

                #[test]
                fn manual_perform_uniffi_conversion_successful() {
                    let test = |sut: SUT| {
                        let ffi_side = <SUT as uniffi::Lower<crate::UniFfiTag>>::lower(sut.clone());
                        let from_ffi =
                            <SUT as uniffi::Lift<crate::UniFfiTag>>::try_lift(ffi_side).unwrap();
                        assert_eq!(from_ffi, sut);
                    };

                    test(SUT::sample());
                    test(SUT::sample_other());
                }
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
			decl_ordered_map!(
				$(
                    #[doc = $expr]
                )*
				[< $element_type s>],
				$element_type
			);
		}
	};
}

pub(crate) use decl_ordered_map;
