use crate::prelude::*;

pub trait PersonaDataEntryValue: From<Self::Value> {
    type Value;
}

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
/// Something akin to: `PersonaDataIdentifiedEntry<T>`.
macro_rules! declare_identified_entry {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $value_type: ty,
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
            pub fn with_id(
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

        paste! {
            #[uniffi::export]
            pub fn [< $struct_name:snake _sample >]() -> $struct_name {
                $struct_name::sample()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _sample_other >]() -> $struct_name {
                $struct_name::sample_other()
            }

            #[cfg(test)]
            mod [< uniffi_ $mod_test_name >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;

                #[test]
                fn test_roundtrip() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            [< $struct_name:snake _sample >](),
                            [< $struct_name:snake _sample_other >](),
                            // duplicates should get removed
                            [< $struct_name:snake _sample >](),
                            [< $struct_name:snake _sample_other >](),
                        ])
                        .len(),
                        2
                    );
                }
            }
        }


        #[cfg(test)]
        mod $mod_test_name {
            use super::*;
            #[allow(clippy::upper_case_acronyms)]
            type SUT = $struct_name;
            type V = $value_type;
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
                    $expected_sample_debug
                );
            }
            #[test]
            fn display() {
                assert_eq!(format!("{}", SUT::sample()), $expected_sample_display);
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
        $type:ident,
        $expected_sample_display: literal,
        $expected_sample_debug: literal,
        $expected_sample_json: literal
    ) => {
        paste! {
            declare_identified_entry!(
                $(
                    #[doc = $expr]
                )*
                [< PersonaDataIdentified $type>],                   // "PhoneNumber" => "PersonaDataIdentifiedPhoneNumber"
                [< PersonaDataEntry $type>],                        // "PhoneNumber" => "PersonaDataEntryPhoneNumber"
                [< tests_persona_data_identified_ $type:snake >],   // "PhoneNumber" => "tests_persona_data_identified_phone_number"
                $expected_sample_display,
                $expected_sample_debug,
                $expected_sample_json
            );
        }
    };
}

declare_identified_entry!(
    /// An identifiable Persona name. Essentially it is a tuple of a
    /// [`(PersonaDataEntryName, Uuid)`].
    Name,
    "Bruce Batman Wayne",
    "Bruce Batman Wayne - 00000000-0000-0000-0000-000000000001",
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
    "#
);

declare_identified_entry!(
    /// An identifiable Persona phone number. Essentially it is a tuple of a
    /// [`(PersonaDataEntryPhoneNumber, Uuid)`].
    PhoneNumber,
    "+46123456789",
    "+46123456789 - 00000000-0000-0000-0000-000000000001",
    r#"
    {
        "id": "00000000-0000-0000-0000-000000000001",
        "value": "+46123456789"
    }
    "#
);

declare_identified_entry!(
    /// An identifiable Persona email address. Essentially it is a tuple of a
    /// [`(PersonaDataEntryEmailAddress, Uuid)`].
    EmailAddress,
    "alan@turing.hero",
    "alan@turing.hero - 00000000-0000-0000-0000-000000000001",
    r#"
    {
        "id": "00000000-0000-0000-0000-000000000001",
        "value": "alan@turing.hero"
    }
    "#
);
