use crate::prelude::*;

pub trait PersonaDataEntryValue: From<Self::Value> {
    type Value;
}

// We have to use macros since UniFFI does not support generics, that is the only
// reason this macro exists, if/when UniFFI supports generics, this macro should
// be replaced by `PersonaDataIdentifiedEntry<T>`.
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
                Self::with_id(PersonaDataEntryID::generate(), value)
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = $value_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl HasSampleValues for $struct_name {
            fn sample() -> Self {
                $struct_name::with_id(
                    PersonaDataEntryID::sample(),
                    <$value_type>::sample(),
                )
            }

            fn sample_other() -> Self {
                $struct_name::with_id(
                    PersonaDataEntryID::sample_other(),
                    <$value_type>::sample_other(),
                )
            }
        }
    };
}

declare_identified_entry!(PersonaDataEntryName, PersonaDataIdentifiedName);
declare_identified_entry!(
    PersonaDataEntryPhoneNumber,
    PersonaDataIdentifiedPhoneNumber
);
declare_identified_entry!(
    PersonaDataEntryEmailAddress,
    PersonaDataIdentifiedEmailAddress
);

#[cfg(test)]
mod identified_name_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataIdentifiedName;
    type V = PersonaDataEntryName;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::new(V::sample()), SUT::new(V::sample()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::sample()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::sample(), V::sample());
    }

    #[test]
    fn new() {
        let value = V::sample_other();
        let sut = SUT::with_id(PersonaDataEntryID::sample_one(), value.clone());
        assert_eq!(
            sut.id,
            "00000000-0000-0000-0000-000000000001".parse().unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            "Bruce Batman Wayne - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "Bruce Batman Wayne");
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = SUT::sample();
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
    fn json_roundtrip_sample_other() {
        let model = SUT::sample_other();
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
    type V = PersonaDataEntryPhoneNumber;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::sample(), V::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::new(V::sample()), SUT::new(V::sample()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::sample()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn new() {
        let value = V::sample_other();
        let sut =
            SUT::with_id(PersonaDataEntryID::sample_four(), value.clone());
        assert_eq!(
            sut.id,
            "00000000-0000-0000-0000-000000000004".parse().unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            "+46123456789 - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "+46123456789");
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = SUT::sample();
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
    fn json_roundtrip_sample_other() {
        let model = SUT::sample_other();
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
    type V = PersonaDataEntryEmailAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::sample(), V::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::new(V::sample()), SUT::new(V::sample()));
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(V::sample()) // generates a new ID
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn new() {
        let value = V::sample_other();
        let sut =
            SUT::with_id(PersonaDataEntryID::sample_three(), value.clone());
        assert_eq!(
            sut.id,
            "00000000-0000-0000-0000-000000000003".parse().unwrap()
        );
        assert_eq!(sut.value, value)
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            "alan@turing.hero - 00000000-0000-0000-0000-000000000001"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "alan@turing.hero");
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = SUT::sample();
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
    fn json_roundtrip_sample_other() {
        let model = SUT::sample_other();
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
