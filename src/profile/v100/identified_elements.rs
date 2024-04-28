/// General rules for identified_array_of implementations
macro_rules! decl_identified_array_of {
	(
        $(
            #[doc = $expr: expr]
        )*
		$struct_type: ident,
        $element_type: ty,
		$collection_type: ty
    ) => {
        use std::ops::DerefMut;
        use $crate::prelude::*;

        paste! {
            $(
                #[doc = $expr]
            )*
			#[derive(Clone, Eq, PartialEq, Hash, derive_more::Debug, derive_more::Display, uniffi::Record)]
			pub struct $struct_type {
                secret_magic: [< $struct_type SecretMagic >]
            }

            #[derive(Clone, Eq, PartialEq, Hash, derive_more::Debug, derive_more::Display, Serialize, Deserialize)]
			pub struct [< $struct_type SecretMagic>]($collection_type);

            uniffi::custom_type!([< $struct_type SecretMagic >], $collection_type);

            impl $crate::UniffiCustomTypeConverter for [< $struct_type SecretMagic >] {
                type Builtin = $collection_type;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    Ok(Self(val))
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    obj.0
                }
            }

            impl $struct_type {
                /// Returns a reference to the element identified by `id`, if it exists.
                pub fn [< get_ $element_type:snake _by_id>](
                    &self,
                    id: &<[< $element_type >] as Identifiable>::ID,
                ) -> Option<&[< $element_type >]> {
                    self.get(id)
                }

                /// Returns a reference to the element by `index`, if it exists.
                   pub fn [< get_ $element_type:snake _at_index>](
                    &self,
                    index: usize,
                ) -> Option<&[< $element_type >]> {
                    self.get_at_index(index)
                }

                /// Returns references to **all** $struct_type, including hidden ones.
                pub fn get_all(&self) -> Vec<&[< $element_type >]> {
                    self.elements()
                }
            }

            #[uniffi::export]
            pub fn [< new_ $struct_type:snake _with_ $element_type:snake >]([< $element_type:snake >]: $element_type) -> $struct_type {
                $struct_type::just([< $element_type:snake >])
            }

            impl IntoIterator for $struct_type {
                type Item = $element_type;
                type IntoIter =
                identified_vec::identified_vec_into_iterator::IdentifiedVecIntoIterator<<$element_type as Identifiable>::ID,    $element_type>;

                fn into_iter(self) -> Self::IntoIter {
                    self.secret_magic.0.into_iter()
                }
            }

			impl Deref for $struct_type {
				type Target = $collection_type;

				fn deref(&self) -> &Self::Target {
					&self.secret_magic.0
				}
			}

			impl DerefMut for $struct_type {
				fn deref_mut(&mut self) -> &mut Self::Target {
					&mut self.secret_magic.0
				}
			}

            #[uniffi::export]
            pub fn [<$struct_type:snake _get_elements>](
                [< $struct_type:snake >]: $struct_type,
            ) -> IdentifiedVecVia<$element_type> {
                (*[< $struct_type:snake >]).clone()
            }

            #[uniffi::export]
            pub fn [< $struct_type:snake _get_ $element_type:snake _by_id>](
                [< $struct_type:snake >]: &$struct_type,
                id: &<[< $element_type >] as Identifiable>::ID,
            ) -> Option<[< $element_type >]> {
                [< $struct_type:snake >].[< get_ $element_type:snake _by_id>](id).cloned()
            }

            #[uniffi::export]
            pub fn [< $struct_type:snake _get_ $element_type:snake _at_index>](
                [< $struct_type:snake >]: &$struct_type,
                index: u64,
            ) -> Option<[< $element_type >]> {
                [< $struct_type:snake >].[< get_ $element_type:snake _at_index>](index as usize).cloned()
            }

            #[uniffi::export]
            pub fn [<$struct_type:snake _element_count>](
                [< $struct_type:snake >]: &$struct_type,
            ) -> u64 {
                (*[< $struct_type:snake >]).len() as u64
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _by_appending>](
                [< $element_type:snake >]: $element_type,
                to: &$struct_type,
            ) -> $struct_type {
                let mut copy = to.clone();
                let _ = (*copy).append([< $element_type:snake >]);
                copy
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _by_updating_or_appending>](
                [< $element_type:snake >]: $element_type,
                to: &$struct_type,
            ) -> $struct_type {
                let mut copy = to.clone();
                let _ = (*copy).update_or_append([< $element_type:snake >]);
                copy
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _by_updating_or_inserting_at_index>](
                [< $element_type:snake >]: $element_type,
                to: &$struct_type,
                index: u64,
            ) -> $struct_type {
                let mut copy = to.clone();
                let _ = (*copy).update_or_insert([< $element_type:snake >], index as usize);
                copy
            }

            #[uniffi::export]
            pub fn [< new_ $struct_type:snake _sample >]() -> $struct_type {
                $struct_type::sample()
            }

            #[uniffi::export]
            pub fn [< new_ $struct_type:snake _sample_other >]() -> $struct_type {
                $struct_type::sample_other()
            }

            #[cfg(test)]
            mod [<tests_ $struct_type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;

                #[allow(clippy::upper_case_acronyms)]
                type SUTSecretMagic = [< $struct_type SecretMagic >];

                #[allow(clippy::upper_case_acronyms)]
                type SUTElement = $element_type;

                #[test]
                fn equality() {
                    assert_eq!(SUT::sample(), SUT::sample());
                    assert_eq!(SUT::sample_other(), SUT::sample_other());
                }

                #[test]
                fn inequality() {
                    assert_ne!(SUT::sample(), SUT::sample_other());
                }

                #[test]
                fn manual_perform_uniffi_conversion() {
                    let sut = SUT::sample();
                    let identified_array = (*sut).clone();
                    let secret_magic = sut.secret_magic;

                    let ffi_side =
                        <SUTSecretMagic as crate::UniffiCustomTypeConverter>::from_custom(secret_magic.clone());
                    assert_eq!(ffi_side, identified_array);
                    let from_ffi_side =
                        <SUTSecretMagic as crate::UniffiCustomTypeConverter>::into_custom(ffi_side)
                        .unwrap();
                    assert_eq!(secret_magic, from_ffi_side);
                }

                #[test]
                fn test_new_with_single_element() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(sut.items(), vec![sut_element]);
                }

                #[test]
                fn test_get_element_by_id() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                        sut.[< get_ $element_type:snake _by_id>](&sut_element.id()),
                        Some(&sut_element)
                    );
                }

                #[test]
                fn test_length() {
                    let sut_element = SUTElement::sample();
                    let mut sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                      sut.len(),
                        1
                    );
                    _  = (*sut).append(SUTElement::sample_other());
                    assert_eq!(
                        sut.len(),
                        2
                    );
                }

            }

            #[cfg(test)]
            mod [<uniffi_tests_ $struct_type:lower>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;

                #[allow(clippy::upper_case_acronyms)]
                type SUTElement = $element_type;

                #[test]
                fn get_elements() {
                    let sut = SUT::sample();
                    let elements = (*sut).clone();

                    assert_eq!(
                        elements,
                        [<$struct_type:snake _get_elements>](sut)
                    );
                }

                #[test]
                fn test_get_at_index() {
                    let a = SUTElement::sample();
                    let b = SUTElement::sample_other();
                    let mut sut = SUT::just(a.clone());
                    sut.append(b.clone());

                    assert_eq!(
                        [< $struct_type:snake _get_ $element_type:snake _at_index>](&sut, 0),
                        Some(a)
                    );
                    assert_eq!(
                        [< $struct_type:snake _get_ $element_type:snake _at_index>](&sut, 1),
                        Some(b)
                    );
                    assert_eq!(
                        [< $struct_type:snake _get_ $element_type:snake _at_index>](&sut, 2),
                        None
                    );
                }

                #[test]
                fn test_new_with_single_element() {
                    let sut_element = SUTElement::sample();
                    let sut = [< new_ $struct_type:snake _with_ $element_type:snake >](sut_element.clone());
                    assert_eq!(
                        sut,
                        $struct_type::just(sut_element.clone())
                    )
                }

                #[test]
                fn test_get_element_by_id() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                        [< $struct_type:snake _get_ $element_type:snake _by_id>](&sut, &sut_element.id()),
                        Some(sut_element)
                    );
                }

                #[test]
                fn test_new_appending_and_length() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                        [<$struct_type:snake _element_count>](&sut),
                        1
                    );
                    let sut = [<new_ $struct_type:snake _by_appending>](SUTElement::sample_other(), &sut);
                    assert_eq!(
                        [<$struct_type:snake _element_count>](&sut),
                        2
                    );
                }

                #[test]
                fn test_new_updating_or_appending() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                        [<$struct_type:snake _element_count>](&sut),
                        1
                    );
                    let sut = [<new_ $struct_type:snake _by_updating_or_appending>](SUTElement::sample_other(), &sut);
                    assert_eq!(
                        [<$struct_type:snake _element_count>](&sut),
                        2
                    );
                }

                #[test]
                fn test_new_updating_or_inserting_at() {
                    let sut_element = SUTElement::sample();
                    let sut = $struct_type::just(sut_element.clone());
                    assert_eq!(
                        [<$struct_type:snake _element_count>](&sut),
                        1
                    );
                    let sut = [<new_ $struct_type:snake _by_updating_or_inserting_at_index>](SUTElement::sample_other(), &sut, 0);

                    assert_eq!(
                        vec![SUTElement::sample_other(), SUTElement::sample()],
                        [<$struct_type:snake _get_elements>](sut).items()
                    );
                }
            }
        }
	};
}

/// Impl rules for identified_array_of implementations which can be empty
macro_rules! decl_can_be_empty_impl {
    (
        $struct_type: ty,
        $element_type: ty,
        $secret_magic: ty
    ) => {
        paste! {

            impl FromIterator<$element_type> for $struct_type {
                fn from_iter<I>([<  $struct_type:lower >]: I) -> Self
                where
                    I: IntoIterator<Item = $element_type>,
                {
                    Self {
                        secret_magic: $secret_magic(IdentifiedVecVia::from_iter([< $struct_type:lower >]))
                    }
                }

            }

            impl Serialize for $struct_type {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
                where
                    S: Serializer,
                {
                    self.secret_magic.serialize(serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for $struct_type {
                fn deserialize<D: Deserializer<'de>>(
                    d: D,
                ) -> Result<$struct_type, D::Error> {
                    $secret_magic::deserialize(d).map(|secret_magic| {
                        $struct_type {
                            secret_magic
                        }
                    })
                    .map_err(de::Error::custom)
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _removed_by_id>](
                [< id_of_ $element_type:snake >]: &<[< $element_type >] as Identifiable>::ID,
                from: &$struct_type,
            ) -> $struct_type {
                let mut copy = from.clone();
                let _ = (*copy).remove_by_id([< id_of_ $element_type:snake >]);
                copy
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _removed_element>](
                [< $element_type:snake >]: &$element_type,
                from: &$struct_type,
            ) -> $struct_type {
                let mut copy = from.clone();
                let _ = (*copy).remove([< $element_type:snake >]);
                copy
            }

            impl $struct_type {

                /// Creates a new empty collection.
                pub fn new() -> Self {
                    Self::from_iter([])
                }

                pub fn [< with_ $struct_type:snake >]<I>([< $struct_type:snake >]: I) -> Self
                where
                    I: IntoIterator<Item = $element_type>,
                {
                    Self::from_iter([< $struct_type:snake >])
                }

                pub fn just([< $element_type:snake >]: $element_type) -> Self {
                    Self::from_iter([[< $element_type:snake >]])
                }
            }

            // Trait: Default
            impl Default for $struct_type {
                /// Instantiates a new empty collection.
                fn default() -> Self {
                    Self::from_iter([])
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake>](
                [< $struct_type:snake >]: IdentifiedVecVia<$element_type>,
            ) -> $struct_type {
                $struct_type::from_iter([< $struct_type:snake >])
            }

            #[cfg(test)]
            mod [<uniffi_impl_tests_ $struct_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;

                #[allow(clippy::upper_case_acronyms)]
                type SUTElement = $element_type;

                #[test]
                fn new_from_empty() {
                    let sut = [<new_ $struct_type:snake>](IdentifiedVecVia::from_iter([]));
                    assert_eq!(
                        0,
                        sut.len()
                    );
                }

                #[test]
                fn new_from_value() {
                    let sut = [<new_ $struct_type:snake>]( IdentifiedVecVia::from_iter([SUTElement::sample()]) );
                    assert_eq!(
                        1,
                        sut.len()
                    );
                }

                #[test]
                fn remove_by_id_to_empty() {
                    let sut_element = SUTElement::sample();
                    let old = SUT::just(sut_element.clone());
                    let new = [<new_ $struct_type:snake _removed_by_id>](&sut_element.id(), &old);
                    assert_eq!(old.len(), 1);
                    assert_eq!(new.len(), 0);
                }

                #[test]
                fn remove_by_element() {
                    let sut_element = SUTElement::sample();
                    let old = SUT::just(sut_element.clone());
                    let new = [<new_ $struct_type:snake _removed_element>](&sut_element, &old);
                    assert_eq!(old.len(), 1);
                    assert_eq!(new.len(), 0);
                }
            }
        }
    }
}

/// Impl rules for identified_array_of implementations which must not be empty
macro_rules! decl_never_empty_impl {
    (
        $struct_type: ty,
        $element_type: ty,
        $secret_magic: ty
    ) => {
        paste! {

            impl Serialize for $struct_type {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
                where
                    S: Serializer,
                {
                    self.secret_magic.serialize(serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for $struct_type {
                fn deserialize<D: Deserializer<'de>>(
                    d: D,
                ) -> Result<$struct_type, D::Error> {
                    Vec::<$element_type>::deserialize(d)
                    .and_then(|vec| {
                        Self::from_iter(vec.into_iter())
                        .map_err(de::Error::custom)
                    })
                    .map_err(de::Error::custom)
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _removed_by_id>](
                [< id_of_ $element_type:snake >]: &<[< $element_type >] as Identifiable>::ID,
                from: &$struct_type,
            ) -> Result<$struct_type> {
                let mut copy = from.clone();
                let _ = (*copy).remove_by_id([< id_of_ $element_type:snake >]);
                if copy.is_empty() {
                    Err(CommonError::[< $struct_type MustNotBeEmpty >])
                } else {
                    Ok(copy)
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _removed_element>](
                [< $element_type:snake >]: &$element_type,
                from: &$struct_type,
            ) -> Result<$struct_type> {
                let mut copy = from.clone();
                let _ = (*copy).remove([< $element_type:snake >]);
                if copy.is_empty() {
                    Err(CommonError::[< $struct_type MustNotBeEmpty >])
                } else {
                    Ok(copy)
                }
            }

            impl $struct_type {

                #[allow(clippy::should_implement_trait)]
                pub fn from_iter<I>([<  $struct_type:snake >]: I) -> Result<Self>
                where
                    I: IntoIterator<Item = $element_type>,
                {
                    let vector = IdentifiedVecVia::from_iter([< $struct_type:snake >]);
                    if vector.is_empty() {
                        Err(CommonError::[< $struct_type MustNotBeEmpty >])
                    } else {
                        Ok(Self {
                            secret_magic: $secret_magic(vector)
                        })
                    }
                }

                pub fn just([< $element_type:snake >]: $element_type) -> Self {
                    Self::from_iter([[< $element_type:snake >]]).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake>](
                [< $struct_type:snake >]: IdentifiedVecVia<$element_type>,
            ) -> Result<$struct_type> {
                $struct_type::from_iter([< $struct_type:snake >])
            }

            #[cfg(test)]
            mod [<uniffi_impl_tests_ $struct_type:snake>] {
                use super::*;


                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;

                #[allow(clippy::upper_case_acronyms)]
                type SUTElement = $element_type;


                #[test]
                #[should_panic]
                fn new_from_empty_error() {
                    [<new_ $struct_type:snake>](IdentifiedVecVia::from_iter([])).unwrap();
                }

                #[test]
                fn new_from_value() {
                    let sut = [<new_ $struct_type:snake>]( IdentifiedVecVia::from_iter([[< $element_type >]::sample()]) ).unwrap();
                    assert_eq!(
                        1,
                        sut.len()
                    );
                }

                #[test]
                fn remove_by_id_to_empty_throws() {
                    let sut_element = SUTElement::sample();
                    let sut = SUT::just(sut_element.clone());
                    assert!(
                        [<new_ $struct_type:snake _removed_by_id>](&sut_element.id(), &sut).is_err()
                    );
                }

                #[test]
                fn remove_by_element_empty_throws() {
                    let sut_element = SUTElement::sample();
                    let sut = SUT::just(sut_element.clone());
                    assert!(
                        [<new_ $struct_type:snake _removed_element>](&sut_element, &sut).is_err()
                    );
                }

                #[test]
                fn remove_by_id_from_two_elements_to_one_is_ok() {
                    let sut_element = SUTElement::sample();
                    let old = SUT::from_iter([sut_element.clone(), SUTElement::sample_other()]).unwrap();
                    let new = [<new_ $struct_type:snake _removed_by_id>](&sut_element.id(), &old).unwrap();
                    assert_eq!(old.len(), 2);
                    assert_eq!(new.len(), 1);
                }

                #[test]
                fn remove_by_element_from_two_elements_to_one_is_ok() {
                    let sut_element = SUTElement::sample();
                    let old = SUT::from_iter([sut_element.clone(), SUTElement::sample_other()]).unwrap();
                    let new = [<new_ $struct_type:snake _removed_element>](&sut_element, &old).unwrap();
                    assert_eq!(old.len(), 2);
                    assert_eq!(new.len(), 1);
                }
            }
        }
    }
}

macro_rules! decl_can_be_empty_identified_array_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_type: ty,
        $element_type: ty
    ) => {
        paste! {
			decl_identified_array_of!(
				$(
                    #[doc = $expr]
                )*
				$struct_type,
				$element_type,
				IdentifiedVecVia<$element_type>
			);

            decl_can_be_empty_impl!(
                $struct_type,
                $element_type,
                [< $struct_type SecretMagic >]
            );
		}
	};
}

macro_rules! decl_never_empty_identified_array_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_type: ty,
        $element_type: ty
    ) => {
        paste! {
			decl_identified_array_of!(
				$(
                    #[doc = $expr]
                )*
				$struct_type,
				$element_type,
				IdentifiedVecVia<$element_type>
			);

            decl_never_empty_impl!(
                $struct_type,
                $element_type,
                [< $struct_type SecretMagic >]
            );
		}
	};
}

pub(crate) use decl_can_be_empty_identified_array_of;
pub(crate) use decl_can_be_empty_impl;
pub(crate) use decl_identified_array_of;
pub(crate) use decl_never_empty_identified_array_of;
pub(crate) use decl_never_empty_impl;
