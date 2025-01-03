use crate::prelude::*;
use paste::paste;
use sargon::IndexMap;

macro_rules! decl_sign_response {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        per_factor_outcome: $per_factor_outcome:ident,
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub per_factor_outcome: Vec<$per_factor_outcome>,
        }

        impl $struct_name {
            pub fn into_internal(&self) -> $internal_struct_name {
                self.clone().into()
            }
        }

        impl From<$internal_struct_name> for $struct_name {
            fn from(value: $internal_struct_name) -> Self {
                Self {
                    per_factor_outcome: value
                        .per_factor_outcome
                        .into_iter()
                        .map(|(_, per_factor)| per_factor.into())
                        .collect(),
                }
            }
        }

        impl From<$struct_name> for $internal_struct_name {
            fn from(value: $struct_name) -> Self {
                Self::new(sargon::IndexMap::from_iter(
                    value.per_factor_outcome.into_iter().map(|item| {
                        let internal_outcome = item.into_internal();
                        (internal_outcome.factor_source_id(), internal_outcome)
                    }),
                ))
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
                per_factor_outcome: [< PerFactorOutcomeOf $signable_id >],
            );
        }
    };
}

decl_sign_response!(TransactionIntentHash);
decl_sign_response!(SubintentHash);
decl_sign_response!(AuthIntentHash);
