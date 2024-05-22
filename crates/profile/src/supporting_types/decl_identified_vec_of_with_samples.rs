use crate::prelude::*;

#[macro_export]
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
            #[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize, std::hash::Hash, derive_more::Display, derive_more::Debug)]
            #[debug("{:?}", self.0)]
            #[display("{}", self.0)]
            #[serde(transparent)]
			pub struct $collection_type(pub IdentifiedVecOf<$element_type>);

            impl From<IdentifiedVecOf<$element_type>> for $collection_type {
                fn from(value: IdentifiedVecOf<$element_type>) -> Self {
                    Self(value)
                }
            }

            uniffi::custom_newtype!($collection_type, IdentifiedVecOf<$element_type>);

            impl std::ops::Deref for $collection_type {
                type Target = IdentifiedVecOf<$element_type>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $collection_type {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl $collection_type {
                #[inline]
                pub fn iter(&self) -> IdentifiedVecOfIterator<$element_type> {
                    // IdentifiedVecOfIterator {
                    //     ordered_map: self,
                    //     index: self.0,
                    // }
                    self.0.iter()
                }

                /// Creates a new empty `IdentifiedVecOf`.
                pub fn new() -> Self {
                    IdentifiedVecOf::new().into()
                }

                /// Creates a new `IdentifiedVecOf` with one single item.
                pub fn just(item: $element_type) -> Self {
                    Self::from_iter([item])
                }
            }

            impl FromIterator<$element_type> for $collection_type
            {
                fn from_iter<T: IntoIterator<Item = $element_type>>(iter: T) -> Self {
                    Self::from(IdentifiedVecOf::<$element_type>::from_iter(iter))
                }
            }

            impl IntoIterator for $collection_type
            {
                type Item = $element_type;
                type IntoIter = OwnedIdentifiedVecOfIterator<$element_type>;

                fn into_iter(self) -> Self::IntoIter {
                    // OwnedIdentifiedVecOfIterator {
                    //     ordered_map: self.0,
                    //     index: 0,
                    // }
                    self.0.into_iter()
                }
            }

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

                // #[test]
                // fn manual_perform_uniffi_conversion_successful() {
                //     let test = |sut: SUT| {
                //         let ffi_side = <SUT as uniffi::Lower<crate::UniFfiTag>>::lower(sut.clone());
                //         let from_ffi =
                //             <SUT as uniffi::Lift<crate::UniFfiTag>>::try_lift(ffi_side).unwrap();
                //         assert_eq!(from_ffi, sut);
                //     };

                //     test(SUT::sample());
                //     test(SUT::sample_other());
                // }
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
