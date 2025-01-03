use crate::prelude::*;
use paste::paste;
use sargon::IndexMap;

macro_rules! decl_sign_response {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        per_factor_outcome: $per_factor_outcome:ident,
        new_from_outcomes: $new_from_outcomes:ident,
        new_from_skipping_factors: $new_from_skipping_factors:ident,
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
                        .map(|(id, outcome)| $per_factor_outcome::new(id.into(), outcome.into()))
                        .collect(),
                }
            }
        }

        impl From<$struct_name> for $internal_struct_name {
            fn from(value: $struct_name) -> Self {
                Self {
                    per_factor_outcome: sargon::IndexMap::from_iter(
                        value.per_factor_outcome.into_iter().map(|item| {
                            let factor_source_id = item.factor_source_id.into_internal();
                            let internal_outcome = item.outcome.into_internal();
                            (factor_source_id, internal_outcome)
                        }),
                    ),
                }
            }
        }


        #[uniffi::export]
        pub fn $new_from_outcomes(
            outcomes: Vec<$per_factor_outcome>
        ) -> Result<$struct_name> {
            $internal_struct_name::new_from_outcomes(
                sargon::IndexMap::from_iter(outcomes.into_iter().map(|item| {
                    let factor_source_id = item.factor_source_id.into_internal();
                    let internal_outcome = item.outcome.into_internal();
                    (factor_source_id, internal_outcome)
                }))
            ).into_result()
        }

        #[uniffi::export]
        pub fn $new_from_skipping_factors(
            factors: Vec<FactorSourceIDFromHash>
        ) -> $struct_name {
            $internal_struct_name::user_skipped_factors(
                sargon::IndexSet::from_iter(factors.iter().map(|f| f.into_internal()))
            ).into()
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
                new_from_outcomes: [< new_sign_response_of_ $signable_id:snake _from_outcomes >],
                new_from_skipping_factors: [< new_sign_response_of_ $signable_id:snake _from_skipping_factors >],
            );
        }
    };
}

decl_sign_response!(TransactionIntentHash);
decl_sign_response!(SubintentHash);
decl_sign_response!(AuthIntentHash);
