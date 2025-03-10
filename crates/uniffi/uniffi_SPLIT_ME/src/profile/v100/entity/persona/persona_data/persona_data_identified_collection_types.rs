use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
/// Something akin to: `CollectionOfIdentifiedPersonaDataEntries<T>`.
macro_rules! declare_collection_of_identified_entry {
    (
        $(
            #[doc = $expr: expr]
        )*
        $id_ent_type: ty,
        $struct_name: ident,
    ) => {
        paste! {
            use sargon::[< CollectionOf $struct_name>] as [< InternalCollectionOf $struct_name>];

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                PartialEq,
                Hash,
                Eq,
                InternalConversion,
                uniffi::Record,
            )]
            pub struct [< CollectionOf $struct_name>] {
                pub collection: Vec<[< PersonaDataIdentified $id_ent_type >]>,
            }

            impl FromInternal<[< InternalCollectionOf $struct_name>], Vec<[< PersonaDataIdentified $id_ent_type >]>> for [< CollectionOf $struct_name>]
            {
                fn into_type(self) -> Vec<[< PersonaDataIdentified $id_ent_type >]> {
                    self.collection.into_iter().map([< PersonaDataIdentified $id_ent_type >]::from).collect()
                }
            }

            impl
            IntoInternal<Vec<[< PersonaDataIdentified $id_ent_type >]>, [< CollectionOf $struct_name>]>
            for Vec<[< PersonaDataIdentified $id_ent_type >]>
        {
            fn into_internal(self) -> [< CollectionOf $struct_name>] {
                [< CollectionOf $struct_name>] {
                    collection: self
                }
            }
        }


            decl_conversion_tests_for!([< CollectionOf $struct_name>]);
        }
    };
}

pub(crate) use declare_collection_of_identified_entry;
