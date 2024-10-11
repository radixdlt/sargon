use crate::prelude::*;
use sargon::PersonaDataEntryName as InternalPersonaDataEntryName;
use sargon::PersonaDataNameVariant as InternalPersonaDataNameVariant;

/// A persons name they have chosen to associated with a Persona, e.g. "Bruce 'Batman' Wayne" using Western name variant,
/// or `"Jun-fan 'Bruce' Lee"` using Eastern name variant (family name comes before given name(s)).
///
/// Nickname is optional in the sense that it can be left blank. Family name and given names are never empty.
///
/// If a name has multiple given names, they all go into the `given_names` String, e.g. Pippi Longstocking's real name -
/// her Swedish name - is in full: "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter Långstrump", where her
/// given names: "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter" are put in the `given_names` field, and
/// "Långstrump" (Longstocking) is her family name.
#[derive(Clone, PartialEq, Hash, Eq, uniffi::Record)]
pub struct PersonaDataEntryName {
    pub variant: PersonaDataNameVariant,
    pub family_name: String,
    pub given_names: String,
    pub nickname: String,
}

#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Enum)]
pub enum PersonaDataNameVariant {
    Western,
    Eastern,
}

impl From<InternalPersonaDataEntryName> for PersonaDataEntryName {
    fn from(value: InternalPersonaDataEntryName) -> Self {
        Self {
            variant: value.variant.into(),
            family_name: value.family_name,
            given_names: value.given_names,
            nickname: value.nickname,
        }
    }
}

impl Into<InternalPersonaDataEntryName> for PersonaDataEntryName {
    fn into(self) -> InternalPersonaDataEntryName {
        InternalPersonaDataEntryName {
            variant: self.variant.into(),
            family_name: self.family_name,
            given_names: self.given_names,
            nickname: self.nickname,
        }
    }
}
