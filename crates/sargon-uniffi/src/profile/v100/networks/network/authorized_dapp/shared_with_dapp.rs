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
    ) => {
        paste! {
        use sargon::$struct_name as [<Internal $struct_name>];

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone,
            PartialEq,
            Hash,
            Eq,
             uniffi::Record,
        )]
        pub struct $struct_name {
            /// The requested quantity to be shared by user, sent by a Dapp.
            pub request: RequestedQuantity,

            /// The by user shared IDs of data identifiable data shared with the
            /// Dapp.
            pub ids: Vec<$id>,
        }

        impl From<[<Internal $struct_name>]> for $struct_name {
            fn from(value: [<Internal $struct_name>]) -> Self {
                Self {
                    request: value.request.into(),
                    ids: value.ids.into_iter().map(Into::into).collect(),
                }
            }
        }

        impl Into<[<Internal $struct_name>]> for $struct_name {
            fn into(self) -> [<Internal $struct_name>] {
                [<Internal $struct_name>] {
                    request: self.request.into(),
                    ids: self.ids.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
    };
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $id: ty,
    ) => {
        paste! {
            declare_shared_with_dapp!(
                $(
                    #[doc = $expr]
                )*
                $struct_name,
                $id,
            );
        }
    };
}

pub(crate) use declare_shared_with_dapp;
