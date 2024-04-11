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
            pub fn [<get_ $struct_type:snake >](
                [< $struct_type:snake >]: $struct_type,
            ) -> IdentifiedVecVia<$element_type> {
                (*[< $struct_type:snake >]).clone()
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
                        [<get_ $struct_type:snake >](sut)
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
    /// An ordered set of [`Persona`]s on a specific network.
    Persona
);

decl_can_be_empty_identified_array_of!(
    /// An ordered set of ['AuthorizedDapp`]s on a specific network.
    AuthorizedDapp
);

decl_never_empty_identified_array_of!(
    /// A collection of [`FactorSource`]s generated by a wallet or manually added by user.
    /// MUST never be empty.
    FactorSource
);
