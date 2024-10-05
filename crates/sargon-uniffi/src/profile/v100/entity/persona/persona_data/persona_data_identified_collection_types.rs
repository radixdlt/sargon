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
        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone,
            PartialEq,
            Hash,
            Eq,
            derive_more::Display,
            derive_more::Debug,
            uniffi::Record,
        )]
        #[debug("{:?}", collection)]
        #[display("{}", self.display_string())]
        pub struct $struct_name {
            pub collection: IdentifiedVecOf<$id_ent_type>,
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
