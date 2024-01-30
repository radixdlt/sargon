use crate::prelude::*;

pub type PersonaDataEntryID = Uuid;

pub trait PersonaDataEntryValue: From<Self::Value> {
    type Value;
}

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
        #[display("{value}")]
        pub struct $struct_name {
            pub id: PersonaDataEntryID,
            pub value: $value_type,
        }

        impl From<$value_type> for $struct_name {
            fn from(value: $value_type) -> Self {
                Self::new(value)
            }
        }
        impl PersonaDataEntryValue for $struct_name {
            type Value = $value_type;
        }

        impl Identifiable for $struct_name {
            type ID = PersonaDataEntryID;

            fn id(&self) -> Self::ID {
                self.id.clone()
            }
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
declare_identified_entry!(PhoneNumber, PersonaDataIdentifiedPhoneNumber);
declare_identified_entry!(EmailAddress, PersonaDataIdentifiedEmailAddress);

#[cfg(test)]
mod identified_name_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataIdentifiedName;
    type V = Name;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(SUT::new(V::placeholder()), SUT::new(V::placeholder()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::placeholder()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::placeholder(), V::placeholder());
    }

    #[test]
    fn new() {
        let value = V::placeholder_other();
        let sut = SUT::with_id(Uuid::nil(), value.clone());
        assert_eq!(
            sut.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "Bruce Batman Wayne - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::placeholder()), "Bruce Batman Wayne");
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000001",
                "value": {
                    "variant": "western",
                    "familyName": "Wayne",
                    "givenNames": "Bruce",
                    "nickname": "Batman"
                }
            }
        "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000002",
                "value": {
                    "variant": "eastern",
                    "familyName": "Jun-fan",
                    "givenNames": "Lee",
                    "nickname": "Bruce"
                }
            }
            "#,
        )
    }
}

// Uh copy paste of tests :/ since sharing tests with `macro_rules` does not really work

#[cfg(test)]
mod identified_number_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataIdentifiedPhoneNumber;
    type V = PhoneNumber;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::placeholder(), V::placeholder());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(SUT::new(V::placeholder()), SUT::new(V::placeholder()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::placeholder()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn new() {
        let value = V::placeholder_other();
        let sut = SUT::with_id(Uuid::nil(), value.clone());
        assert_eq!(
            sut.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "+46123456789 - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::placeholder()), "+46123456789");
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000001",
                "value": "+46123456789"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000002",
                "value": "+44987654321"
            }
            "#,
        )
    }
}

#[cfg(test)]
mod identified_email_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataIdentifiedEmailAddress;
    type V = EmailAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::placeholder(), V::placeholder());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(SUT::new(V::placeholder()), SUT::new(V::placeholder()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::placeholder()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn new() {
        let value = V::placeholder_other();
        let sut = SUT::with_id(Uuid::nil(), value.clone());
        assert_eq!(
            sut.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::placeholder()),
            "alan@turing.hero - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::placeholder()), "alan@turing.hero");
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000001",
                "value": "alan@turing.hero"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000002",
                "value": "satoshi@nakamoto.btc"
            }
            "#,
        )
    }
}
