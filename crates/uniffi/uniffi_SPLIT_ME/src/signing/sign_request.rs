use crate::prelude::*;
use paste::paste;

macro_rules! decl_sign_request {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        per_factor_source_input: $per_factor_source_input:ident,
        internal_per_factor_source_input: $internal_per_factor_source_input:ident,
        new_sample: $new_sample:ident,
        new_sample_other: $new_sample_other:ident,
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub factor_source_kind: FactorSourceKind,

            /// Per factor source, a set of inputs that contain information on what signables need signing,
            /// and what signables will fail if such factor source is neglected.
            pub per_factor_source: Vec<$per_factor_source_input>,
        }

        impl $struct_name {
            pub fn into_internal(&self) -> $internal_struct_name {
                self.clone().into()
            }
        }

        impl From<$internal_struct_name> for $struct_name {
            fn from(value: $internal_struct_name) -> Self {
                Self {
                    factor_source_kind: value.factor_source_kind.into(),
                    per_factor_source: value
                        .per_factor_source
                        .into_iter()
                        .map(|(id, input)| {
                            $per_factor_source_input::new(
                                id.into(),
                                input.per_transaction
                                    .into_iter()
                                    .map(|t| t.into())
                                    .collect(),
                                input.invalid_transactions_if_neglected
                                    .into_iter()
                                    .map(|t| t.into())
                                    .collect()
                            )
                        })
                        .collect(),
                }
            }
        }

        impl From<$struct_name> for $internal_struct_name
        {
            fn from(value: $struct_name) -> Self {
                Self {
                    factor_source_kind: value.factor_source_kind.into_internal(),
                    per_factor_source: value
                        .per_factor_source
                        .iter()
                        .map(|item| {
                            (
                                item.factor_source_id.into_internal(),
                                $internal_per_factor_source_input::new(
                                    item.factor_source_id.into_internal(),
                                    item.per_transaction
                                        .iter()
                                        .map(|t| t.into_internal())
                                        .collect(),
                                    item.invalid_transactions_if_neglected
                                        .iter()
                                        .map(|t| t.into_internal())
                                        .collect()
                                )

                            )
                        })
                        .collect(),
                }
            }
        }

        #[uniffi::export]
        pub fn $new_sample() -> $struct_name {
            $internal_struct_name::sample().into()
        }

        #[uniffi::export]
        pub fn $new_sample_other() -> $struct_name {
            $internal_struct_name::sample_other().into()
        }

        decl_conversion_tests_for!($struct_name);
    };
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignRequestOf $signable >] =
                sargon::SignRequest<[< Internal $signable >]>;
            type [< InternalPerFactorSourceInputOf $signable >] =
                sargon::PerFactorSourceInput<[< Internal $signable >]>;

            decl_sign_request!(
                struct_name: [< SignRequestOf $signable >],
                internal_struct_name: [< InternalSignRequestOf $signable >],
                per_factor_source_input: [< PerFactorSourceInputOf $signable >],
                internal_per_factor_source_input: [< InternalPerFactorSourceInputOf $signable >],
                new_sample: [< new_sign_request_of_ $signable:snake _sample >],
                new_sample_other: [< new_sign_request_of_ $signable:snake _sample_other >],
            );
        }
    };
}

decl_sign_request!(
    signable: TransactionIntent,
    signable_id: TransactionIntentHash
);
decl_sign_request!(
    signable: Subintent,
    signable_id: SubintentHash
);
decl_sign_request!(
    signable: AuthIntent,
    signable_id: AuthIntentHash
);
