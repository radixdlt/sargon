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
        #[debug("{request} - {ids}")]
        #[display("{request}")]
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
