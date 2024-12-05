use crate::prelude::*;
use paste::paste;

macro_rules! decl_invalid_transaction_if_neglected {
    (
        struct_name: $struct_name:ident,
        signable_id: $signable_id:ident,
    ) => {
        /// A list of entities which would fail in a transaction if we would
        /// neglect certain factor source, either by user explicitly skipping
        /// it or if implicitly neglected due to failure.
        #[derive(
            Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
        )]
        pub struct $struct_name {
            /// The intent hash of the transaction which would be invalid if a
            /// certain factor source would be neglected, either if user
            /// explicitly skipped it or implicitly neglected due to failure.
            pub signable_id: $signable_id,

            /// The entities in the transaction which would fail auth.
            pub entities_which_would_fail_auth: Vec<AddressOfAccountOrPersona>,
        }
    };
    ($type:ty) => {
        paste! {
            use sargon::[< $type >] as [< Internal $type >];

            type [< InternalInvalidTransactionIfNeglectedOf $type >] =
                sargon::InvalidTransactionIfNeglected<[< Internal $type >]>;

            decl_invalid_transaction_if_neglected!(
                struct_name: [< InvalidTransactionIfNeglectedOf $type >],
                signable_id: [< $type >],
            );
        }
    };
}

decl_invalid_transaction_if_neglected!(TransactionIntentHash);
decl_invalid_transaction_if_neglected!(SubintentHash);
