use crate::prelude::*;

macro_rules! declare_collection_of_identified_entry {
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
        #[debug("{collection}")]
        #[display("{collection}")]
        #[serde(transparent)]
        pub struct $struct_name {
            pub collection: IdentifiedVecVia<$id_ent_type>,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::values([])
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = IdentifiedVecVia<$id_ent_type>;

            fn deref(&self) -> &Self::Target {
                &self.collection
            }
        }

        impl $struct_name {
            pub fn values<I>(values: I) -> Self
            where
                I: IntoIterator<Item = $id_ent_type>,
            {
                Self {
                    collection: IdentifiedVecVia::from_iter(values),
                }
            }

            pub fn new(value: $id_ent_type) -> Self {
                Self::values([value])
            }
        }

        impl HasPlaceholder for $struct_name {
            fn placeholder() -> Self {
                $struct_name::values([
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
    fn display() {
        let value = V::placeholder();
        let sut = SUT::new(value.clone());
        assert_eq!(format!("{}", sut), format!("[{}]", value));
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
    fn display() {
        let value = V::placeholder();
        let sut = SUT::new(value.clone());
        assert_eq!(format!("{}", sut), format!("[{}]", value));
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
