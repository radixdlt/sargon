use crate::prelude::*;
use paste::paste;
use sargon::IndexMap;

macro_rules! decl_sign_response {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        per_factor_source: $per_factor_source:ident,
    ) => {
        /// The response of a batch signing request, either a PolyFactor or MonoFactor signing
        /// request, matters not, because the goal is to have signed all transactions with
        /// enough keys (derivation paths) needed for it to be valid when submitted to the
        /// Radix network.
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub per_factor_source: Vec<$per_factor_source>,
        }

        impl $struct_name {
            pub fn into_internal(&self) -> $internal_struct_name {
                self.clone().into()
            }
        }

        impl From<$internal_struct_name> for $struct_name {
            fn from(value: $internal_struct_name) -> Self {
                Self {
                    per_factor_source: value.signatures
                        .into_iter()
                        .map(|(id, hd_signatures)| {
                            $per_factor_source::new(
                                id.into(),
                                hd_signatures.into_iter().map(|s| s.into()).collect()
                            )
                        })
                        .collect()
                }
            }
        }

        impl From<$struct_name> for $internal_struct_name {
            fn from(value: $struct_name) -> Self {
                Self::new(
                    sargon::IndexMap::from_iter(value.per_factor_source.into_iter().map(
                        |item| {
                            (
                                item.factor_source_id.into_internal(),
                                item.hd_signatures.into_internal()
                            )
                        },
                    )),
                )
            }
        }

        decl_conversion_tests_for!($struct_name);
    };
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignResponseOf $signable_id >] =
                sargon::SignResponse<[< Internal $signable_id >]>;

            decl_sign_response!(
                struct_name: [< SignResponseOf $signable_id >],
                internal_struct_name: [< InternalSignResponseOf $signable_id >],
                per_factor_source: [< SignaturesPerFactorSourceOf $signable_id >],
            );
        }
    };
}

decl_sign_response!(TransactionIntentHash);
decl_sign_response!(SubintentHash);
