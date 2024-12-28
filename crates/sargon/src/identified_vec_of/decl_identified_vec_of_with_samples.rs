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
            #[allow(non_camel_case_types)]
            #[derive(
                Clone,
                PartialEq,
                Eq,
                Hash,
                Serialize,
                Deserialize,
                Default,
                derive_more::Debug,
                derive_more::Deref,
                derive_more::DerefMut
            )]
            #[serde(transparent)]
			pub struct $collection_type(pub $crate::IdentifiedVecOf<$element_type>);

            impl std::fmt::Display for $collection_type where $element_type: std::fmt::Display {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0.description())
                }
            }

            impl $collection_type {
                pub fn new() -> Self {
                    Self($crate::IdentifiedVecOf::new())
                }

                /// Creates a new collection with one single item.
                pub fn just(item: $element_type) -> Self {
                    Self::from_iter([item])
                }
            }

            impl From<$crate::IdentifiedVecOf<$element_type>> for $collection_type {
                fn from(value: $crate::IdentifiedVecOf<$element_type>) -> Self {
                    Self(value)
                }
            }

            // impl $collection_type {
            //     #[inline]
            //     pub fn iter(&self) -> IdentifiedVecOfIterator<$element_type> {
            //         self.0.iter()
            //     }
            // }


            impl FromIterator<$element_type> for $collection_type
            {
                fn from_iter<T: IntoIterator<Item = $element_type>>(iter: T) -> Self {
                    $crate::IdentifiedVecOf::<$element_type>::from_iter(iter).into()
                }
            }



            impl<'a> IntoIterator
    for &'a $collection_type
{
    type Item = $element_type;
    type IntoIter = IdentifiedVecOfIterator<'a, $element_type>;

    fn into_iter(self) -> Self::IntoIter {
        IdentifiedVecOfIterator {
            ordered_map: self,
            index: 0,
        }
    }
}

impl IntoIterator
    for $collection_type
{
    type Item = $element_type;
    type IntoIter = OwnedIdentifiedVecOfIterator<$element_type>;

    fn into_iter(self) -> Self::IntoIter {
        OwnedIdentifiedVecOfIterator {
            ordered_map: self.0,
            index: 0,
        }
    }
}





            // #[cfg(test)]
            // mod [< $collection_type:snake _tests >] {
            //     use super::*;

            //     #[allow(clippy::upper_case_acronyms)]
            //     type SUT = $collection_type;

            //     #[test]
            //     fn test_ids() {
            //         assert_eq!(SUT::sample().ids().into_iter().cloned().collect_vec(), SUT::sample().get_all().into_iter().map(|i| i.id()).collect_vec());
            //     }
            // }
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

pub use decl_identified_vec_of;
