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
            pub ids: IdentifiedVecVia<$id>,
        }

        impl $struct_name {
            /// Constructs a new $struct_name where `ids` "fulfills" the `request`.
            ///
            /// # Panics
            /// Panics if `ids` does not fulfill `request`, for more information
            /// see [`RequestedQuantity::is_fulfilled_by_ids`]
            pub fn new(
                request: RequestedQuantity,
                ids: impl Into<IdentifiedVecVia<$id>>,
            ) -> Self {
                let ids = ids.into();
                let len = ids.len();
                assert!(
                    request.is_fulfilled_by_ids(len),
                    "ids does not fulfill request, got: #{}, but requested: {}",
                    len,
                    request
                );
                Self { request, ids }
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
                _ = SUT::new(RequestedQuantity::at_least(1), IdentifiedVecVia::new())
            }

            #[test]
            #[should_panic = "ids does not fulfill request, got: #0, but requested: Exactly: 1"]
            fn panics_when_exactly_is_not_fulfilled() {
                _ = SUT::new(RequestedQuantity::exactly(1), IdentifiedVecVia::new())
            }

            #[test]
            #[should_panic = "Invalid quantity Exactly: 0"]
            fn panics_when_exactly_0() {
                _ = SUT::new(RequestedQuantity::exactly(0), IdentifiedVecVia::new())
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

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaAccountAddresses,
    AccountAddress,
    "Exactly: 2 - #2 ids shared",
    "Exactly: 2 - shared ids: [account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr, account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264]",
    r#"
    {
        "request": {
            "quantifier": "exactly",
            "quantity": 2
        },
        "ids": [
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
        ]
    }
    "#
);

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaIDsOfPersonaDataEntries,
    PersonaDataEntryID,
    "AtLeast: 2 - #3 ids shared",
    "AtLeast: 2 - shared ids: [00000000-0000-0000-0000-000000000001, 00000000-0000-0000-0000-000000000002, 00000000-0000-0000-0000-000000000004]",
    r#"
    {
        "request": {
            "quantifier": "atLeast",
            "quantity": 2
        },
        "ids": ["00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002", "00000000-0000-0000-0000-000000000004"]
    }
    "#
);

impl HasSampleValues for SharedToDappWithPersonaIDsOfPersonaDataEntries {
    fn sample() -> Self {
        Self::new(
            RequestedQuantity::at_least(2),
            IdentifiedVecVia::from_iter([
                PersonaDataEntryID::sample_one(),
                PersonaDataEntryID::sample_two(),
                PersonaDataEntryID::sample_four(),
            ]),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            RequestedQuantity::exactly(1),
            IdentifiedVecVia::from_iter([PersonaDataEntryID::sample_one()]),
        )
    }
}

impl HasSampleValues for SharedToDappWithPersonaAccountAddresses {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}
impl SharedToDappWithPersonaAccountAddresses {
    pub fn sample_mainnet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_mainnet(),
                AccountAddress::sample_mainnet_other(),
            ]),
        )
    }
    pub fn sample_mainnet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_mainnet_other(),
            ]),
        )
    }
    pub fn sample_stokenet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_stokenet(),
                AccountAddress::sample_stokenet_other(),
            ]),
        )
    }
    pub fn sample_stokenet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_stokenet_other(),
            ]),
        )
    }
}
