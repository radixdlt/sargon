use crate::prelude::*;

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
    ) => {
        paste! {
        use sargon::$struct_name as [<Internal $struct_name>];

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
            pub id: PersonaDataEntryID,
            pub value: $value_type,
        }

        impl From<[<Internal $struct_name>]>
            for $struct_name
        {
            fn from(value: [<Internal $struct_name>]) -> Self {
                Self {
                    id: value.id.into(),
                    value: value.value.into(),
                }
            }
        }

        impl Into<[<Internal $struct_name>]>
            for $struct_name
        {
            fn into(self) -> [<Internal $struct_name>] {
                [<Internal $struct_name>] {
                    id: self.id.into(),
                    value: self.value.into(),
                }
            }
        }

            #[uniffi::export]
            pub fn [< $struct_name:snake _sample >]() -> $struct_name {
                [<Internal $struct_name>]::sample().into()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _sample_other >]() -> $struct_name {
                [<Internal $struct_name>]::sample_other().into()
            }
        }
    };

    (
        $(
            #[doc = $expr: expr]
        )*
        $type:ident,
    ) => {
        paste! {
            declare_identified_entry!(
                $(
                    #[doc = $expr]
                )*
                [< PersonaDataIdentified $type>],                   // "PhoneNumber" => "PersonaDataIdentifiedPhoneNumber"
                [< PersonaDataEntry $type>],                        // "PhoneNumber" => "PersonaDataEntryPhoneNumber"
            );
        }
    };
}

declare_identified_entry!(
    /// An identifiable Persona name. Essentially it is a tuple of a
    /// [`(PersonaDataEntryName, Uuid)`].
    Name,
);

declare_identified_entry!(
    /// An identifiable Persona phone number. Essentially it is a tuple of a
    /// [`(PersonaDataEntryPhoneNumber, Uuid)`].
    PhoneNumber,
);

declare_identified_entry!(
    /// An identifiable Persona email address. Essentially it is a tuple of a
    /// [`(PersonaDataEntryEmailAddress, Uuid)`].
    EmailAddress,
);
