use crate::prelude::*;

/// A persons name they have chosen to associated with a Persona, e.g. "Bruce 'Batman' Wayne" using Western name variant,
/// or `"Jun-fan 'Bruce' Lee"` using Eastern name variant (family name comes before given name(s)).
///
/// Nickname is optional in the sense that it can be left blank. Family name and given names are never empty.
///
/// If a name has multiple given names, they all go into the `given_names` String, e.g. Pippi Longstocking's real name -
/// her Swedish name - is in full: "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter Långstrump", where her
/// given names: "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter" are put in the `given_names` field, and
/// "Långstrump" (Longstocking) is her family name.
#[derive(
    Clone,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.full())]
#[debug("{}", self.full())]
pub struct PersonaDataEntryName {
    pub variant: PersonaDataNameVariant,
    pub family_name: String,
    pub given_names: String,
    pub nickname: String,
}

#[derive(
    Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
)]
#[serde(rename_all = "lowercase")]
pub enum PersonaDataNameVariant {
    Western,
    Eastern,
}
