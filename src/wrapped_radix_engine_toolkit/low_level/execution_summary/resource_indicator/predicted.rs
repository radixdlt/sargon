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


        $(
            #[doc = $expr]
        )*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
        pub struct $struct_name {
            pub value: $wrapped_type,
            pub instruction_index: u64,
        }
        impl $struct_name {
            pub fn new(value: impl Into<$wrapped_type>, instruction_index: u64) -> Self {
                Self {
                    value: value.into(),
                    instruction_index
                }
            }
            pub fn from_ret<T>(ret_predicted: RetPredicted<T>) -> Self where T: Into<$wrapped_type> {
                Self::new(
                    ret_predicted.value,
                    ret_predicted.instruction_index as u64
                )
            }
        }


        #[cfg(test)]
        mod $mod_test_name {
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

type ScryptoNonFungibleLocalIds = IndexSet<ScryptoNonFungibleLocalId>;
type RetPredictedNonFungibleLocalIds = RetPredicted<ScryptoNonFungibleLocalIds>;

impl From<RetPredictedNonFungibleLocalIds> for PredictedNonFungibleLocalIds {
    fn from(value: RetPredictedNonFungibleLocalIds) -> Self {
        Self::new(
            value
                .value
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect_vec(),
            value.instruction_index as u64,
        )
    }
}

impl HasSampleValues for PredictedDecimal {
    fn sample() -> Self {
        Self::new(Decimal::one(), 0)
    }

    fn sample_other() -> Self {
        Self::new(Decimal::three(), 1)
    }
}

impl HasSampleValues for PredictedNonFungibleLocalIds {
    fn sample() -> Self {
        Self::new(
            vec![
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            0,
        )
    }

    fn sample_other() -> Self {
        Self::new(vec![NonFungibleLocalId::sample_other()], 1)
    }
}
