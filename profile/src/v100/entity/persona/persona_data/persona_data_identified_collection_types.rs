use crate::prelude::*;

macro_rules! declare_collection_of_identified_entry {
    ($id_ent_type:ty,$struct_name:ident) => {
        /// A collection of identifiable PersonaData Entries.
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
        #[debug("{collection}")]
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

        impl HasPlaceholder for $struct_name {
            fn placeholder() -> Self {
                $struct_name::entries([
                    <$id_ent_type>::placeholder(),
                    <$id_ent_type>::placeholder_other(),
                ])
            }

            fn placeholder_other() -> Self {
                $struct_name::new(<$id_ent_type>::placeholder_other())
            }
        }
    };
}

declare_collection_of_identified_entry!(
    PersonaDataIdentifiedPhoneNumber,
    CollectionOfPhoneNumbers
);

declare_collection_of_identified_entry!(
    PersonaDataIdentifiedEmailAddress,
    CollectionOfEmailAddresses
);

#[cfg(test)]
mod collection_of_phone_numbers_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CollectionOfPhoneNumbers;
    type V = PersonaDataIdentifiedPhoneNumber;

    #[test]
    fn new() {
        let value = V::placeholder_other();
        let sut = SUT::new(value.clone());
        assert_eq!(sut.collection.items(), vec![value]);
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(
            SUT::new(V::new(V::placeholder().value)),
            SUT::new(V::new(V::placeholder().value))
        ); // generates new ID, thus not equal
    }

    #[test]
    fn new_with_value() {
        let value = PersonaDataEntryPhoneNumber::placeholder();
        let sut = SUT::single_value(value.clone());
        assert_eq!(sut.collection.items().first().unwrap().value, value);
    }

    #[test]
    fn new_with_values() {
        let value0 = PersonaDataEntryPhoneNumber::placeholder();
        let value1 = PersonaDataEntryPhoneNumber::placeholder_other();
        let sut = SUT::values([value0.clone(), value1.clone()]);
        assert_eq!(
            sut.collection.into_iter().map(|e| e.value).collect_vec(),
            vec![value0, value1]
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from([SUT::placeholder(), SUT::placeholder()])
                .len(),
            1
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::placeholder()),
            "[+46123456789, +44987654321]"
        );
        assert_eq!(format!("{}", SUT::placeholder_other()), "[+44987654321]");
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "[+46123456789 - 00000000-0000-0000-0000-000000000001, +44987654321 - 00000000-0000-0000-0000-000000000002]"
        );
        assert_eq!(
            format!("{:?}", SUT::placeholder_other()),
            "[+44987654321 - 00000000-0000-0000-0000-000000000002]"
        );
    }

    #[test]
    fn deref() {
        assert_eq!(
            *SUT::placeholder().items(),
            vec![V::placeholder(), V::placeholder_other()]
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
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
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            [
                {
                    "id": "00000000-0000-0000-0000-000000000002",
                    "value": "+44987654321"
                }
            ]
            "#,
        )
    }
}

// Uh copy paste of tests :/ since sharing tests with `macro_rules` does not really work

#[cfg(test)]
mod collection_of_email_addresses_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CollectionOfEmailAddresses;
    type V = PersonaDataIdentifiedEmailAddress;

    #[test]
    fn new() {
        let value = V::placeholder_other();
        let sut = SUT::new(value.clone());
        assert_eq!(sut.collection.items(), vec![value]);
    }

    #[test]
    fn new_with_value() {
        let value = PersonaDataEntryEmailAddress::placeholder();
        let sut = SUT::single_value(value.clone());
        assert_eq!(sut.collection.items().first().unwrap().value, value);
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(
            SUT::new(V::new(V::placeholder().value)),
            SUT::new(V::new(V::placeholder().value))
        ); // generates new ID, thus not equal
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from([SUT::placeholder(), SUT::placeholder()])
                .len(),
            1
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::placeholder()),
            "[alan@turing.hero, satoshi@nakamoto.btc]"
        );
        assert_eq!(
            format!("{}", SUT::placeholder_other()),
            "[satoshi@nakamoto.btc]"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "[alan@turing.hero - 00000000-0000-0000-0000-000000000001, satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002]"
        );
        assert_eq!(
            format!("{:?}", SUT::placeholder_other()),
            "[satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002]"
        );
    }

    #[test]
    fn deref() {
        assert_eq!(
            *SUT::placeholder().items(),
            vec![V::placeholder(), V::placeholder_other()]
        );
    }

    #[test]
    fn new_with_values() {
        let value0 = PersonaDataEntryEmailAddress::placeholder();
        let value1 = PersonaDataEntryEmailAddress::placeholder_other();
        let sut = SUT::values([value0.clone(), value1.clone()]);
        assert_eq!(
            sut.collection.into_iter().map(|e| e.value).collect_vec(),
            vec![value0, value1]
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
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
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            [
                {
                    "id": "00000000-0000-0000-0000-000000000002",
                    "value": "satoshi@nakamoto.btc"
                }
            ]
            "#,
        )
    }
}
