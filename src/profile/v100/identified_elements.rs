use std::ops::DerefMut;

use crate::prelude::*;

/// General rules for identified_array_of implementations
macro_rules! decl_identified_array_of {
	(
        $(
            #[doc = $expr: expr]
        )*
        $element_type: ty,
		$struct_type: ident,
		$collection_type: ty
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
			#[derive(Clone, Eq, PartialEq, Hash, derive_more::Debug, derive_more::Display, Serialize, Deserialize, uniffi::Record)]
            #[serde(transparent)]
			pub struct $struct_type {
                secret_magic: [< $struct_type SecretMagic >]
            }

            #[derive(Clone, Eq, PartialEq, Hash, derive_more::Debug, derive_more::Display, Serialize, Deserialize)]
			pub struct [< $struct_type SecretMagic>]($collection_type);

            uniffi::custom_type!([< $struct_type SecretMagic >], $collection_type);

            impl crate::UniffiCustomTypeConverter for [< $struct_type SecretMagic >] {
                type Builtin = $collection_type;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    Ok(Self(val))
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    obj.0
                }
            }

            impl [< $element_type s >] {
                /// Returns a reference to the element identified by `id`, if it exists.
                pub fn [< get_ $element_type:snake _by_id>](
                    &self,
                    id: &<[< $element_type >] as Identifiable>::ID,
                ) -> Option<&[< $element_type >]> {
                    self.get(id)
                }

                /// Returns references to **all** $struct_type, including hidden ones.
                pub fn get_all(&self) -> Vec<&[< $element_type >]> {
                    self.elements()
                }
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
                [< $struct_type:snake >]: $struct_type,
                id: &<[< $element_type >] as Identifiable>::ID,
            ) -> Option<[< $element_type >]> {
                [< $struct_type:snake >].[< get_ $element_type:snake _by_id>](id).cloned()
            }

            #[uniffi::export]
            pub fn [<$struct_type:snake _element_count>](
                [< $struct_type:snake >]: $struct_type,
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

            #[cfg(test)]
            mod [<tests_ $struct_type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;
                
                type SUTSecretMagic = [< $struct_type SecretMagic >];

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


            }

            #[cfg(test)]
            mod [<uniffi_tests_ $struct_type:lower>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_type;

                #[test]
                fn get_elements() {
                    let sut = SUT::sample();
                    let elements = (*sut).clone();

                    assert_eq!(
                        elements,
                        [<$struct_type:snake _get_elements>](sut)
                    );
                }
            }
        }
	};
}

/// Impl rules for identified_array_of implementations which can be empty
macro_rules! dec_can_be_empty_impl {
    (
        $element_type: ty,
        $struct_type: ty,
        $secret_magic: ty
    ) => {
        paste! {

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

            impl [< $element_type s >] {

                #[allow(clippy::should_implement_trait)]
                pub fn from_iter<I>([<  $struct_type:lower >]: I) -> Self
                where
                    I: IntoIterator<Item = $element_type>,
                {
                    Self {
                        secret_magic: $secret_magic(IdentifiedVecVia::from_iter([< $struct_type:lower >]))
                    }
                }

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

                pub fn [< with_ $element_type:snake >]([< $element_type:snake >]: $element_type) -> Self {
                    Self::[< with_ $struct_type:snake >]([[< $element_type:snake >]])
                }
            }

            // Trait: Default
            impl Default for $struct_type {
                /// Instantiates a new empty collection.
                fn default() -> Self {
                    Self::[< with_ $struct_type:snake >]([])
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
                    let sut = [<new_ $struct_type:snake>]( IdentifiedVecVia::from_iter([[< $element_type >]::sample()]) );
                    assert_eq!(
                        1,
                        sut.len()
                    );
                }
            }
        }
    }
}

/// Impl rules for identified_array_of implementations which must not be empty
macro_rules! dec_never_empty_impl {
    (
        $element_type: ty,
        $struct_type: ty,
        $secret_magic: ty
    ) => {
        paste! {

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake _removed_by_id>](
                [< id_of_ $element_type:snake >]: &<[< $element_type >] as Identifiable>::ID,
                from: &$struct_type,
            ) -> Result<$struct_type> {
                let mut copy = from.clone();
                let _ = (*copy).remove_by_id([< id_of_ $element_type:snake >]);
                if copy.is_empty() {
                    Err(CommonError::Unknown)
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
                    Err(CommonError::Unknown)
                } else {
                    Ok(copy)
                }
            }

            impl [< $element_type s >] {

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

                pub fn [< with_ $struct_type:snake >]<I>([< $struct_type:snake >]: I) -> Result<Self>
                where
                    I: IntoIterator<Item = $element_type>,
                {
                    Self::from_iter([< $struct_type:snake >])
                }

                pub fn [< with_ $element_type:snake >]([< $element_type:snake >]: $element_type) -> Self {
                    Self::[< with_ $struct_type:snake >]([[< $element_type:snake >]]).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_ $struct_type:snake>](
                [< $struct_type:snake >]: IdentifiedVecVia<$element_type>,
            ) -> Result<$struct_type> {
                $struct_type::from_iter([< $struct_type:snake >])
            }

            #[uniffi::export]
            pub fn [< new_ $struct_type:snake _with_ $element_type:snake >]([< $element_type:snake >]: $element_type) -> $struct_type {
                $struct_type::[< with_ $element_type:snake >]([< $element_type:snake >])
            }

            #[cfg(test)]
            mod [<uniffi_impl_tests_ $struct_type:snake>] {
                use super::*;

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
            }
        }
    }
}

macro_rules! decl_can_be_empty_identified_array_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $element_type: ty
    ) => {
        paste! {
			decl_identified_array_of!(
				$(
                    #[doc = $expr]
                )*
				$element_type,
				[< $element_type s >],
				IdentifiedVecVia<$element_type>
			);

            dec_can_be_empty_impl!(
                $element_type,
                [< $element_type s >],
                [< $element_type s SecretMagic >]
            );
		}
	};
}

macro_rules! decl_never_empty_identified_array_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $element_type: ty
    ) => {
        paste! {
			decl_identified_array_of!(
				$(
                    #[doc = $expr]
                )*
				$element_type,
				[< $element_type s >],
				IdentifiedVecVia<$element_type>
			);

            dec_never_empty_impl!(
                $element_type,
                [< $element_type s >],
                [< $element_type s SecretMagic >]
            );
		}
	};
}

decl_can_be_empty_identified_array_of!(
    /// An ordered set of [`Account`]s on a specific network, most commonly
    /// the set is non-empty, since wallets guide user to create a first
    /// Account.
    Account
);

decl_can_be_empty_identified_array_of!(
    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    EntityFlag
);

decl_can_be_empty_identified_array_of!(
    /// An ordered set of [`Persona`]s on a specific network.
    Persona
);

decl_can_be_empty_identified_array_of!(
    /// An ordered set of ['AuthorizedDapp`]s on a specific network.
    AuthorizedDapp
);

decl_can_be_empty_identified_array_of!(
    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with DApps, but might be Android or iPhone
    /// clients as well.
    P2PLink
);

decl_never_empty_identified_array_of!(
    /// A collection of [`FactorSource`]s generated by a wallet or manually added by user.
    /// MUST never be empty.
    FactorSource
);

decl_never_empty_identified_array_of!(
    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    ProfileNetwork
);
