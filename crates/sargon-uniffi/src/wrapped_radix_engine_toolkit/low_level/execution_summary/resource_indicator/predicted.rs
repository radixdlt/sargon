use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
///
/// Declares a struct with a "generic" value, with an `instruction_index`.
macro_rules! decl_predicted {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $wrapped_type: ty,
        $mod_test_name: ident
    ) => {
        paste! {
        use sargon::$struct_name as [< Internal $struct_name >];

        $(
            #[doc = $expr]
        )*
        #[derive(Clone,  PartialEq, Eq, Hash,  uniffi::Record)]
        pub struct $struct_name {
            pub value: $wrapped_type,
            pub instruction_index: u64,
        }

        impl From<[< Internal $struct_name >]> for $struct_name {
            fn from(value: [< Internal $struct_name >]) -> Self {
                Self {
                    value: value.value.into(),
                    instruction_index: value.instruction_index,
                }
            }
        }

        impl Into<[< Internal $struct_name >]> for $struct_name {
            fn into(self) -> [< Internal $struct_name >] {
                [< Internal $struct_name >]{
                    value: self.value.into(),
                    instruction_index: self.instruction_index,
                }
            }
        }
    }
    };

    (
        $(
            #[doc = $expr: expr]
        )*
        $wrapped_type:ty
    ) => {
        paste! {
            decl_predicted!(
                $(
                    #[doc = $expr]
                )*
                [< Predicted $wrapped_type >],
                $wrapped_type,
                [< tests_ predicted_ $wrapped_type:snake >]
            );
        }
    };
}

decl_predicted!(
    /// A PredictedDecimal is not a guaranteed amount, but a approximated based
    /// on the contents of the transaction manifest and the state of the ledger
    /// at the time of analysis (preview).
    Decimal
);

decl_predicted!(
    /// A prediction of a collection of NonFungibleLocalId
    PredictedNonFungibleLocalIds,
    Vec<NonFungibleLocalId>,
    tests_predicted_non_fungible_local_ids
);