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
                ids: IdentifiedVecVia<$id_ent_type>,
            ) -> Self {
                Self { request, ids }
            }
        }

        impl HasPlaceholder for $struct_name {
            fn placeholder() -> Self {
                $struct_name::new(
                    RequestedQuantity::exactly(2),
                    IdentifiedVecVia::from_iter([
                        <$id_ent_type>::placeholder(),
                        <$id_ent_type>::placeholder_other(),
                    ]),
                )
            }

            fn placeholder_other() -> Self {
                $struct_name::new(
                    RequestedQuantity::at_least(1),
                    IdentifiedVecVia::from_iter([
                        <$id_ent_type>::placeholder_other(),
                    ]),
                )
            }
        }
    };
}

declare_shared_with_dapp!(AccountAddress, SharedAccounts);

declare_shared_with_dapp!(PersonaDataEntryID, SharedCollection);

impl SharedAccounts {
    pub fn placeholder_mainnet() -> Self {
        let value = Self::placeholder();
        assert!(&value.ids.iter().all(|a| a.network_id == NetworkID::Mainnet));
        value
    }
    pub fn placeholder_mainnet_other() -> Self {
        let value = Self::placeholder_other();
        assert!(&value.ids.iter().all(|a| a.network_id == NetworkID::Mainnet));
        value
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
