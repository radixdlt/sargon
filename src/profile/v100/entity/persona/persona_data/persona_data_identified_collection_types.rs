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
        #[debug("{:?}", collection)]
        #[display("{}", self.display_string())]
        #[serde(transparent)]
        pub struct $struct_name {
            pub collection: IdentifiedVecVia<$id_ent_type>,
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::entries([])
            }
        }
        impl std::ops::Deref for $struct_name {
            type Target = IdentifiedVecVia<$id_ent_type>;
            fn deref(&self) -> &Self::Target {
                &self.collection
            }
        }
        impl $struct_name {
            pub fn entries<I>(values: I) -> Self
            where
                I: IntoIterator<Item = $id_ent_type>,
            {
                Self {
                    collection: IdentifiedVecVia::from_iter(values),
                }
            }
            pub fn new(value: $id_ent_type) -> Self {
                Self::entries([value])
            }
            /// Creates a new CollectionOf PersonaDataEntries using just the *value*, which will be given a
            /// generated ID and put in an identified entry.
            pub fn single_value(
                value: <$id_ent_type as PersonaDataEntryValue>::Value,
            ) -> Self {
                Self::new(value.into())
            }
            pub fn values<I>(values: I) -> Self
            where
                I: IntoIterator<
                    Item = <$id_ent_type as PersonaDataEntryValue>::Value,
                >,
            {
                Self::entries(values.into_iter().map(|v| v.into()))
            }
        }
        impl $struct_name {
            fn display_string(&self) -> String {
                let items =
                    self.items().into_iter().map(|v| v.to_string()).join(", ");
                format!("[{}]", items)
            }
        }

        impl HasSampleValues for $struct_name {
            fn sample() -> Self {
                $struct_name::entries([
                    <$id_ent_type>::sample(),
                    <$id_ent_type>::sample_other(),
                ])
            }
            fn sample_other() -> Self {
                $struct_name::new(<$id_ent_type>::sample_other())
            }
        }

        #[cfg(test)]
        mod $mod_test_name {
            use super::*;

            #[allow(clippy::upper_case_acronyms)]
            type SUT = $struct_name;
            type V = $id_ent_type;
            type E = $entry_type;

            #[test]
            fn new() {
                let value = V::sample_other();
                let sut = SUT::new(value.clone());
                assert_eq!(sut.collection.items(), vec![value]);
            }

            #[test]
            fn equality() {
                assert_eq!(SUT::sample(), SUT::sample());
            }

            #[test]
            fn inequality() {
                assert_ne!(SUT::sample(), SUT::sample_other());
                assert_ne!(
                    SUT::new(V::new(V::sample().value)),
                    SUT::new(V::new(V::sample().value))
                ); // generates new ID, thus not equal
            }

            #[test]
            fn new_with_value() {
                let value = E::sample();
                let sut = SUT::single_value(value.clone());
                assert_eq!(sut.collection.items().first().unwrap().value, value);
            }

            #[test]
            fn new_with_values() {
                let value0 = E::sample();
                let value1 = E::sample_other();
                let sut = SUT::values([value0.clone(), value1.clone()]);
                assert_eq!(
                    sut.collection.into_iter().map(|e| e.value).collect_vec(),
                    vec![value0, value1]
                );
            }

            #[test]
            fn hash() {
                assert_eq!(
                    HashSet::<SUT>::from([SUT::sample(), SUT::sample()]).len(),
                    1
                );
            }

            #[test]
            fn display() {
                assert_eq!(
                    format!("{}", SUT::sample()),
                    $expected_sample_display
                );
            }

            #[test]
            fn debug() {
                assert_eq!(
                    format!("{:?}", SUT::sample()),
                    $expected_sample_debug
                );
            }

            #[test]
            fn deref() {
                assert_eq!(
                    *SUT::sample().items(),
                    vec![V::sample(), V::sample_other()]
                );
            }

            #[test]
            fn json_roundtrip_sample() {
                let model = SUT::sample();
                assert_eq_after_json_roundtrip(
                    &model,
                    $expected_sample_json
                )
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

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataIdentifiedPhoneNumber`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataEntryPhoneNumber)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    phone_number,  // singular form
    phone_numbers, // plural form
    "[+46123456789, +44987654321]",
    "[+46123456789 - 00000000-0000-0000-0000-000000000001, +44987654321 - 00000000-0000-0000-0000-000000000002]",
    r#"
    [
        {
            "id": "00000000-0000-0000-0000-000000000001",
            "value": "+46123456789"
        },
        {
            "id": "00000000-0000-0000-0000-000000000002",
            "value": "+44987654321"
        }
    ]
    "#
);

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataEntryEmailAddress`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataIdentifiedEmailAddress)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    email_address,   // singular form
    email_addresses, // plural form
    "[alan@turing.hero, satoshi@nakamoto.btc]",
    "[alan@turing.hero - 00000000-0000-0000-0000-000000000001, satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002]",
    r#"
    [
        {
            "id": "00000000-0000-0000-0000-000000000001",
            "value": "alan@turing.hero"
        },
        {
            "id": "00000000-0000-0000-0000-000000000002",
            "value": "satoshi@nakamoto.btc"
        }
    ]
    "#
);
