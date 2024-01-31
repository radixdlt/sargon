use crate::prelude::*;

macro_rules! declare_shared_with_dapp {
    ($id_ent_type:ty,$struct_name:ident) => {
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
            pub request: RequestedQuantity,
            pub ids: IdentifiedVecVia<$id_ent_type>,
        }

        impl $struct_name {
            pub fn new(
                request: RequestedQuantity,
                ids: impl Into<IdentifiedVecVia<$id_ent_type>>,
            ) -> Self {
                Self {
                    request,
                    ids: ids.into(),
                }
            }

            pub fn shared_ids_string(&self) -> String {
                let ids_str = self.ids.iter().map(|v| v.to_string()).join(", ");
                format!("{} - shared ids: [{}]", self.request, ids_str)
            }
        }
    };
}

declare_shared_with_dapp!(AccountAddress, SharedAccounts);
declare_shared_with_dapp!(PersonaDataEntryID, SharedCollection);

impl HasPlaceholder for SharedAccounts {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}
impl SharedAccounts {
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
    type SUT = SharedAccounts;

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
