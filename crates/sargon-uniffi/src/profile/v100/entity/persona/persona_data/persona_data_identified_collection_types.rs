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
        $entry_type: ty,
        $struct_name: ident,
        $mod_test_name: ident,
        $expected_sample_display: literal,
        $expected_sample_debug: literal,
        $expected_sample_json: literal
    ) => {
        use sargon::$struct_name as InternalCollection;

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
            pub collection: Vec<$id_ent_type>,
        }

        impl From<InternalCollection>
            for $struct_name
        {
            fn from(value: InternalCollection) -> Self {
                Self {
                    collection: value.collection.into_iter().map(|x| x.into()).collect(),
                }
            }
        }

        impl Into<InternalCollection>
            for $struct_name
        {
            fn into(self) -> InternalCollection {
                Self {
                    collection: self.collection.into_iter().map(|x| x.into()).collect(),
                }
            }
        }
    };
    (
        $(
            #[doc = $expr: expr]
        )*
        $singular_form: ident,
        $plural_form: ident,
        $expected_sample_display: literal,
        $expected_sample_debug: literal,
        $expected_sample_json: literal
    ) => {
        paste! {
            declare_collection_of_identified_entry!(
                $(
                    #[doc = $expr]
                )*
                [< PersonaDataIdentified $singular_form:camel>],    // "email_address" => "PersonaDataIdentifiedEmailAddress"
                [< PersonaDataEntry $singular_form:camel>],         // "email_address" => "PersonaDataEntryEmailAddress"
                [< CollectionOf $plural_form:camel>],               // "email_addresses" => "CollectionOfEmailAddresses"
                [< tests_collection_of_ $plural_form >],
                $expected_sample_display,
                $expected_sample_debug,
                $expected_sample_json
            );
        }
    };
}

pub(crate) use declare_collection_of_identified_entry;
