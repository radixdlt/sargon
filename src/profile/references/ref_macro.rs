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
            pub struct $struct_name {
                pub inner: RwLock<Option<$inner>>,
            }

            impl $struct_name {
                pub fn with_inner(inner: $inner) -> Self {
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
                    {
                        match self.inner.read() {
                            Ok(ref rhs) => match other.inner.read() {
                                Ok(ref lhs) => match rhs.as_ref() {
                                    Some(r) => match lhs.as_ref() {
                                        Some(l) => r == l,
                                        None => false,
                                    },
                                    None => lhs.as_ref().is_none(),
                                },
                                _ => false,
                            },
                            _ => false,
                        }
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
                pub fn new(inner: $inner) -> Arc<Self> {
                    Arc::new(Self::from(inner))
                }

                pub fn take(self: Arc<Self>) -> Result<$inner> {
                    self.inner
                        .try_write()
                        .unwrap()
                        .take()
                        .ok_or(CommonError::InnerValueAlreadyTakenFromReferenceContainer { type_name: type_name::<$inner>() })
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
