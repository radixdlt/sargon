/// Declares a `uniffi::Object` exported type holding
/// an inner type with an `RwLock` so that we can
/// take the value from an `Arc<Ref>`, thus avoiding
/// expensive clone operations.
macro_rules! decl_ref_named {
    (
        $struct_name: ty,
        $inner: ty
    ) => {
        paste! {
            use crate::prelude::*;
            use std::sync::{Arc, RwLock};

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
                #[uniffi::constructor]
                pub(crate) fn new(inner: $inner) -> Arc<Self> {
                    Arc::new(Self::from(inner))
                }

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
        }
    };
}

macro_rules! decl_ref {
    ($inner: ty) => {
        paste! {
            decl_ref_named!(
                [< Ref $inner:camel >],
                $inner
            );
        }
    };
}

pub(crate) use decl_ref;
pub(crate) use decl_ref_named;
