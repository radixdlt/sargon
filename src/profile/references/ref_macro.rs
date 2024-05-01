/// Declares a `uniffi::Object` exported type holding
/// an inner type with an `RwLock` so that we can
/// take the value from an `Arc<Ref>`, thus avoiding
/// expensive clone operations.
macro_rules! decl_ref_named {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ty,
        $inner: ty
    ) => {
        paste! {
            use crate::prelude::*;
            use std::sync::{Arc, RwLock};

            $(
                #[doc = $expr]
            )*
            #[derive(Debug, uniffi::Object)]
            #[uniffi::export(Debug, Eq, Hash)]
            pub(crate) struct $struct_name {
                inner: RwLock<Option<$inner>>,
            }

            impl $struct_name {
                pub(crate) fn with_inner(inner: $inner) -> Self {
                    Self {
                        inner: RwLock::new(Some(inner)),
                    }
                }
            }

            impl std::hash::Hash for $struct_name {
                /// Manual hash impl since Ref type holds `RwLock` which is
                /// not Hash.
                fn hash<H>(&self, state: &mut H)
                where
                    H: std::hash::Hasher,
                {
                    match self.inner.read() {
                        Ok(ref guard) => {
                            state.write_u8(1);
                            match guard.as_ref() {
                                Some(prof) => {
                                    prof.hash(state);
                                    state.write_u8(100);
                                }
                                None => state.write_u8(200),
                            }
                        }
                        _ => {
                            state.write_u8(255);
                        }
                    }
                }
            }

            impl Eq for $struct_name {}
            impl PartialEq for $struct_name {
                /// Manual Eq impl since Ref type holds `RwLock` which is
                /// not Eq.
                fn eq(&self, other: &Self) -> bool {
                    match (self.inner.try_read(), other.inner.try_read()) {
                        (Ok(ref lhs), Ok(ref rhs)) => match(lhs.as_ref(), rhs.as_ref()) {
                            (Some(l), Some(r)) => l == r,
                            _ => false
                        },
                        _ => false,
                    }
                }
            }

            impl From<$inner> for $struct_name {
                fn from(value: $inner) -> Self {
                    Self::with_inner(value)
                }
            }

            #[uniffi::export]
            impl $struct_name {

                /// Constructor for the reference type passing the inner value.
                /// You can read out the inner value using `take`. Typically you
                /// would not construct `RefFoo` on the FFI side and call `take`
                /// on it to read out the inner. The typical pattern is that you
                /// on the FFI side call a UniFFI exported Rust function that
                /// takes `RefFoo`, which you construct on the FFI side and pass
                /// to the function, which then returns `RefBar` and you read out
                /// the "bar" value using `take()`.
                ///
                /// Two examples are:
                ///
                /// 1. Profile JSON decoding:
                /// `[FFI] RefBytes <~ Rust Sargon ~> RefProfile [FFI]`
                /// then FFI calls `take()` on `RefProfile` to get the Profile.
                ///
                /// 2. Profile JSON encoding:
                /// `[FFI] RefProfile <~ Rust Sargon ~> RefBytes [FFI]`
                /// then FFI calls `take()` on `RefBytes` to get the JSON bytes.
                ///
                #[uniffi::constructor]
                pub(crate) fn new(inner: $inner) -> Arc<Self> {
                    Arc::new(Self::from(inner))
                }

                /// Reads out the `inner` value and **consumes** this reference
                /// type, meaning if you on the FFI side call `take` again, an
                /// error is thrown, since the value is no longer there. This
                /// enables us to on the Rust side avoid using expensive `clone()`.
                pub(crate) fn take(self: Arc<Self>) -> Result<$inner> {
                    self.inner
                        .try_write()
                        .expect("Should only acquire write lock once.")
                        .take()
                        .ok_or(CommonError::InnerValueAlreadyTakenFromReferenceContainer { type_name: type_name::<$inner>() })
                }
            }

            impl HasSampleValues for $struct_name {
                fn sample() -> Self {
                    Self::with_inner($inner::sample())
                }
                fn sample_other() -> Self {
                    Self::with_inner($inner::sample_other())
                }
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $struct_name:snake _sample >]() -> $struct_name {
                $struct_name::sample()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $struct_name:snake _sample_other >]() -> $struct_name {
                $struct_name::sample_other()
            }

            #[cfg(test)]
            mod [< test_ $struct_name:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;

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
                fn hash_of_samples() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            SUT::sample(),
                            SUT::sample_other(),
                            // duplicates should get removed
                            SUT::sample(),
                            SUT::sample_other(),
                        ])
                        .len(),
                        2
                    );
                }

                #[test]
                fn new_then_take() {
                    let arced = SUT::new($inner::sample());
                    assert_eq!(arced.take().unwrap(), $inner::sample());
                }
            }

            #[cfg(test)]
            mod [< uniffi_ test_ $struct_name:snake >] {
                use super::*;

                #[test]
                fn inequality() {
                    assert_ne!(
                        [< new_ $struct_name:snake _sample >](),
                        [< new_ $struct_name:snake _sample_other >]()
                    );
                }
            }
        }
    };
}

macro_rules! decl_ref {
    (
        $(
            #[doc = $expr: expr]
        )*
        $inner: ty
    ) => {
        paste! {
            decl_ref_named!(
                $(
                    #[doc = $expr]
                )*
                [< Ref $inner:camel >],
                $inner
            );
        }
    };
}

pub(crate) use decl_ref;
pub(crate) use decl_ref_named;
