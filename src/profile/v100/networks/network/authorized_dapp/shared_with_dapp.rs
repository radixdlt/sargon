use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
/// Something akin to `SharedToDappWithPersonaIDs<T>`.
macro_rules! declare_shared_with_dapp {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $id: ty,
        $mod_test_name: ident,
        $expected_sample_display: literal,
        $expected_sample_debug: literal,
        $expected_sample_json: literal
    ) => {
        $(
            #[doc = $expr]
        )*
        #[derive(
            Serialize,
            Deserialize,
            Clone,
            PartialEq,
            Hash,
            Eq,
            derive_more::Display,
            derive_more::Debug,
            uniffi::Record,
        )]
        #[debug("{}", self.shared_ids_string())]
        #[display("{request} - #{} ids shared", self.ids.len())]
        pub struct $struct_name {
            /// The requested quantity to be shared by user, sent by a Dapp.
            pub request: RequestedQuantity,

            /// The by user shared IDs of data identifiable data shared with the
            /// Dapp.
            pub ids: OrderedMap<$id>,
        }

        impl $struct_name {
            /// Constructs a new $struct_name where `ids` "fulfills" the `request`.
            ///
            /// # Panics
            /// Panics if `ids` does not fulfill `request`, for more information
            /// see [`RequestedQuantity::is_fulfilled_by_ids`]
            pub fn new(
                request: RequestedQuantity,
                ids: impl IntoIterator<Item = $id>,
            ) -> Self {
                let ids = OrderedMap::from_iter(ids.into_iter());
                let len = ids.len();
                assert!(
                    request.is_fulfilled_by_ids(len),
                    "ids does not fulfill request, got: #{}, but requested: {}",
                    len,
                    request
                );
                Self { request, ids }
            }

            pub fn exactly(
                ids: impl IntoIterator<Item = $id>,
            ) -> Self {
                let ids = ids.into_iter().collect_vec();
                Self::new(RequestedQuantity::exactly(ids.len() as u16), ids)
            }

            pub fn just(
                id: $id,
            ) -> Self {
               Self::exactly([id])
            }

            /// String representation of the request and shared ids.
            pub fn shared_ids_string(&self) -> String {
                let ids_str = self.ids.iter().map(|v| v.to_string()).join(", ");
                format!("{} - shared ids: [{}]", self.request, ids_str)
            }
        }

        #[cfg(test)]
        mod $mod_test_name {
            use crate::prelude::*;

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
            fn hash() {
                assert_eq!(
                    HashSet::<_>::from_iter([SUT::sample(), SUT::sample()]).len(),
                    1
                );
            }

            #[test]
            #[should_panic = "ids does not fulfill request, got: #0, but requested: AtLeast: 1"]
            fn panics_when_at_least_is_not_fulfilled() {
                _ = SUT::new(RequestedQuantity::at_least(1), [])
            }

            #[test]
            #[should_panic = "ids does not fulfill request, got: #0, but requested: Exactly: 1"]
            fn panics_when_exactly_is_not_fulfilled() {
                _ = SUT::new(RequestedQuantity::exactly(1), [])
            }

            #[test]
            #[should_panic = "Invalid quantity Exactly: 0"]
            fn panics_when_exactly_0() {
                _ = SUT::new(RequestedQuantity::exactly(0), [])
            }

            #[test]
            fn display() {
                assert_eq!(format!("{}", SUT::sample()), $expected_sample_display);
            }

            #[test]
            fn debug() {
                assert_eq!(
                    format!("{:?}", SUT::sample()), $expected_sample_debug
                );
            }

            #[test]
            fn json_roundtrip_sample() {
                let model = SUT::sample();
                assert_eq_after_json_roundtrip(
                    &model,
                    $expected_sample_json
                );
            }
        }
    };
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $id: ty,
        $expected_sample_display: literal,
        $expected_sample_debug: literal,
        $expected_sample_json: literal
    ) => {
        paste! {
            declare_shared_with_dapp!(
                $(
                    #[doc = $expr]
                )*
                $struct_name,
                $id,
                [< tests_ $struct_name:snake >],
                $expected_sample_display,
                $expected_sample_debug,
                $expected_sample_json
            );
        }
    };
}

pub(crate) use declare_shared_with_dapp;
