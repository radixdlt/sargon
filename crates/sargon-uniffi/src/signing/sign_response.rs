use crate::prelude::*;
use sargon::IndexMap;
use sargon::IndexSet;
use paste::paste;

macro_rules! decl_sign_response {
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignResponseOf $signable_id >] =
                sargon::SignResponse<[< Internal $signable_id >]>;

            /// The response of a batch signing request, either a PolyFactor or MonoFactor signing
            /// request, matters not, because the goal is to have signed all transactions with
            /// enough keys (derivation paths) needed for it to be valid when submitted to the
            /// Radix network.
            #[derive(Clone, PartialEq, Eq, uniffi::Record)]
            pub struct [< SignResponseOf $signable_id >] {
                pub per_factor_source: Vec<[< SignaturesPerFactorSourceOf $signable_id >]>,
            }

            impl [< SignResponseOf $signable_id >] {
                pub fn into_internal(&self) -> [< InternalSignResponseOf $signable_id >] {
                    self.clone().into()
                }
            }

            impl From<[< InternalSignResponseOf $signable_id >]>
                for [< SignResponseOf $signable_id >]
            {
                fn from(value: [< InternalSignResponseOf $signable_id >]) -> Self {
                    Self {
                        per_factor_source: value.signatures
                            .into_iter()
                            .map(|(id, hd_signatures)| {
                                [< SignaturesPerFactorSourceOf $signable_id >]::new(
                                    id.into(),
                                    hd_signatures.into_iter().map(|s| s.into()).collect()
                                )
                            })
                            .collect()
                    }
                }
            }

            impl From<[< SignResponseOf $signable_id >]>
                for [< InternalSignResponseOf $signable_id >]
            {
                fn from(value: [< SignResponseOf $signable_id >]) -> Self {
                    Self::new(
                        sargon::IndexMap::from_iter(value.per_factor_source.into_iter().map(
                            |item| {
                                (
                                    item.factor_source_id.into_internal(),
                                    sargon::IndexSet::from_iter(
                                        item.hd_signatures.into_iter().map(|s| s.into_internal()),
                                    ),
                                )
                            },
                        )),
                    )
                }
            }

            decl_conversion_tests_for!([< SignResponseOf $signable_id >]);
        }
    };
}

decl_sign_response!(TransactionIntentHash);
decl_sign_response!(SubintentHash);