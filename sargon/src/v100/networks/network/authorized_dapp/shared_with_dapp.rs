use crate::prelude::*;

// We have to use macros since UniFFI does not support generics, that is the only
// reason this macro exists, if/when UniFFI supports generics, this macro should
// be replaced by `SharedToDappWithPersonaIDs<T>`.
macro_rules! declare_shared_with_dapp {
    ($id:ty,$struct_name:ident) => {
        /// IDs that have been shared with an Dapp the user has interacted with
        /// that fulfill a Dapp request's specified [`RequestedQuantity`].
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
            /// # Panic
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
    };
}

declare_shared_with_dapp!(
    AccountAddress,
    SharedToDappWithPersonaAccountAddresses
);
declare_shared_with_dapp!(
    PersonaDataEntryID,
    SharedToDappWithPersonaIDsOfPersonaDataEntries
);

impl HasPlaceholder for SharedToDappWithPersonaIDsOfPersonaDataEntries {
    fn placeholder() -> Self {
        Self::new(
            RequestedQuantity::at_least(2),
            IdentifiedVecVia::from_iter([
                PersonaDataEntryID::placeholder_one(),
                PersonaDataEntryID::placeholder_two(),
                PersonaDataEntryID::placeholder_four(),
            ]),
        )
    }

    fn placeholder_other() -> Self {
        Self::new(
            RequestedQuantity::exactly(1),
            IdentifiedVecVia::from_iter(
                [PersonaDataEntryID::placeholder_one()],
            ),
        )
    }
}

impl HasPlaceholder for SharedToDappWithPersonaAccountAddresses {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}
impl SharedToDappWithPersonaAccountAddresses {
    pub fn placeholder_mainnet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                AccountAddress::placeholder_mainnet(),
                AccountAddress::placeholder_mainnet_other(),
            ]),
        )
    }
    pub fn placeholder_mainnet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::placeholder_mainnet_other(),
            ]),
        )
    }
    pub fn placeholder_stokenet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                AccountAddress::placeholder_stokenet(),
                AccountAddress::placeholder_stokenet_other(),
            ]),
        )
    }
    pub fn placeholder_stokenet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::placeholder_stokenet_other(),
            ]),
        )
    }
}

#[cfg(test)]
mod shared_accounts_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SharedToDappWithPersonaAccountAddresses;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<_>::from_iter([SUT::placeholder(), SUT::placeholder()])
                .len(),
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
        assert_eq!(
            format!("{}", SUT::placeholder()),
            "Exactly: 2 - #2 ids shared"
        );
        assert_eq!(
            format!("{}", SUT::placeholder_other()),
            "AtLeast: 1 - #1 ids shared"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "Exactly: 2 - shared ids: [account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease, account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master]"
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
		    	"request": {
		    		"quantifier": "exactly",
		    		"quantity": 2
		    	},
		    	"ids": [
		    		"account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
		    		"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
		    	]
		    }
        "#,
        );
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"request": {
					"quantifier": "atLeast",
					"quantity": 1
				},
				"ids": [
					"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
				]
			}
        "#,
        );
    }
}

#[cfg(test)]
mod shared_collection_of_ids_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SharedToDappWithPersonaIDsOfPersonaDataEntries;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<_>::from_iter([SUT::placeholder(), SUT::placeholder()])
                .len(),
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
        assert_eq!(
            format!("{}", SUT::placeholder()),
            "AtLeast: 2 - #3 ids shared"
        );
        assert_eq!(
            format!("{}", SUT::placeholder_other()),
            "Exactly: 1 - #1 ids shared"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "AtLeast: 2 - shared ids: [00000000-0000-0000-0000-000000000001, 00000000-0000-0000-0000-000000000002, 00000000-0000-0000-0000-000000000004]"
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
		    	"request": {
		    		"quantifier": "atLeast",
		    		"quantity": 2
		    	},
		    	"ids": ["00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002", "00000000-0000-0000-0000-000000000004"]
		    }
        "#,
        );
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"request": {
					"quantifier": "exactly",
					"quantity": 1
				},
				"ids": ["00000000-0000-0000-0000-000000000001"]
			}
        "#,
        );
    }
}
