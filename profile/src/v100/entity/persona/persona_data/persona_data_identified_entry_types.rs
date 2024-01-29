use crate::prelude::*;

pub type PersonaDataEntryID = Uuid;

macro_rules! declare_identified_entry {
    ($value_type:ty,$struct_name:ident) => {
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
        #[debug("{} - {}", value, id)]
        #[display("{} - {}", value, id)]
        pub struct $struct_name {
            pub id: PersonaDataEntryID,
            pub value: $value_type,
        }

        impl $struct_name {
            pub(crate) fn with_id(
                id: PersonaDataEntryID,
                value: $value_type,
            ) -> Self {
                Self { id, value }
            }
            pub fn new(value: $value_type) -> Self {
                Self::with_id(id(), value)
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = $value_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl HasPlaceholder for $struct_name {
            fn placeholder() -> Self {
                $struct_name::with_id(
                    Uuid::from_str("00000000-0000-0000-0000-000000000001")
                        .unwrap(),
                    <$value_type>::placeholder(),
                )
            }

            fn placeholder_other() -> Self {
                $struct_name::with_id(
                    Uuid::from_str("00000000-0000-0000-0000-000000000002")
                        .unwrap(),
                    <$value_type>::placeholder_other(),
                )
            }
        }
    };
}

declare_identified_entry!(Name, PersonaDataIdentifiedName);

#[cfg(test)]
mod identified_name_testes {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataIdentifiedName;

    #[test]
    fn new() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Name counstruction should not fail");
        let identified_entry = SUT::with_id(Uuid::nil(), name.clone());
        assert_eq!(
            identified_entry.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(identified_entry.value, name)
    }

    #[test]
    fn display() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Name counstruction should not fail");
        let identified_entry = SUT::with_id(Uuid::nil(), name);
        assert_eq!(
            format!("{identified_entry}"),
            "Bruce Batman Wayne - 00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn placeholder() {
        let placeholder = SUT::placeholder();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert_eq!(placeholder.value.to_string(), "Bruce Batman Wayne");
    }

    #[test]
    fn placeholder_other() {
        let placeholder = SUT::placeholder_other();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000002"
        );
        assert_eq!(placeholder.value.to_string(), "Jun-fan Bruce Lee");
    }

    #[test]
    fn json_roundtrip_batman() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000001",
                "value": {
                    "variant": "Western",
                    "familyName": "Wayne",
                    "givenName": "Bruce",
                    "nickname": "Batman"
                }
             }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_bruce_lee() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000002",
                "value": {
                    "variant": "Eastern",
                    "familyName": "Jun-fan",
                    "givenName": "Lee",
                    "nickname": "Bruce"
                }
            }
            "#,
        )
    }
}
