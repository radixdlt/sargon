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
#[display("{}", self.full())]
#[debug("{}", self.full())]
#[serde(rename_all = "camelCase")]
pub struct PersonaDataEntryName {
    pub variant: Variant,
    pub family_name: String,
    pub given_names: String,
    pub nickname: String,
}

impl PersonaDataEntryName {
    pub fn new(
        variant: Variant,
        family_name: impl AsRef<str>,
        given_names: impl AsRef<str>,
        nickname: impl AsRef<str>,
    ) -> Result<Self> {
        let family_name = family_name.as_ref().trim().to_string();
        let given_names = given_names.as_ref().trim().to_string();
        let nickname = nickname.as_ref().trim().to_string();
        if family_name.is_empty() {
            return Err(CommonError::PersonaDataInvalidNameFamilyNameEmpty);
        }
        if given_names.is_empty() {
            return Err(CommonError::PersonaDataInvalidNameGivenNamesEmpty);
        }
        Ok(Self {
            variant,
            family_name,
            given_names,
            nickname,
        })
    }

    fn full(&self) -> String {
        match self.variant {
            Variant::Western => format!(
                "{} {} {}",
                self.given_names, self.nickname, self.family_name
            ),
            Variant::Eastern => format!(
                "{} {} {}",
                self.family_name, self.nickname, self.given_names
            ),
        }
    }
}

impl HasSampleValues for PersonaDataEntryName {
    fn sample() -> Self {
        PersonaDataEntryName::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Should have a valid Name sample")
    }

    fn sample_other() -> Self {
        PersonaDataEntryName::new(Variant::Eastern, "Jun-fan", "Lee", "Bruce")
            .expect("Should have a valid Name sample")
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
)]
#[serde(rename_all = "lowercase")]
pub enum Variant {
    Western,
    Eastern,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn new_name() {
        let name = PersonaDataEntryName::new(
            Variant::Western,
            "\n Wayne\n ",
            "  Bruce  ",
            "Batman ",
        )
        .unwrap(); // testing trim
        assert_eq!(name.family_name, "Wayne");
        assert_eq!(name.given_names, "Bruce");
        assert_eq!(name.nickname, "Batman");
    }

    #[test]
    fn new_from_string_multiple_given_names() {
        let name = PersonaDataEntryName::new(
            Variant::Western,
            "Långstrump",
            "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter",
            "Pippi",
        )
        .unwrap();
        assert_eq!(
            name.given_names,
            "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter"
        );
    }

    #[test]
    fn sample() {
        let sample = PersonaDataEntryName::sample();
        assert_eq!(sample.family_name, "Wayne");
        assert_eq!(sample.given_names, "Bruce");
        assert_eq!(sample.nickname, "Batman");
    }

    #[test]
    fn sample_other() {
        let sample = PersonaDataEntryName::sample_other();
        assert_eq!(sample.family_name, "Jun-fan");
        assert_eq!(sample.given_names, "Lee");
        assert_eq!(sample.nickname, "Bruce");
    }

    #[test]
    fn empty_family_name_is_err() {
        assert_eq!(
            PersonaDataEntryName::new(
                Variant::Western,
                "",
                "Clark",
                "Superman"
            ),
            Err(CommonError::PersonaDataInvalidNameFamilyNameEmpty)
        );
    }

    #[test]
    fn spaces_trimmed_empty_family_name_is_err() {
        assert_eq!(
            PersonaDataEntryName::new(
                Variant::Western,
                "  ",
                "Clark",
                "Superman"
            ),
            Err(CommonError::PersonaDataInvalidNameFamilyNameEmpty)
        );
    }

    #[test]
    fn empty_given_names_is_err() {
        assert_eq!(
            PersonaDataEntryName::new(Variant::Western, "Kent", "", "Superman"),
            Err(CommonError::PersonaDataInvalidNameGivenNamesEmpty)
        );
    }

    #[test]
    fn spaces_trimmed_empty_given_names_is_err() {
        assert_eq!(
            PersonaDataEntryName::new(
                Variant::Western,
                "Kent",
                " ",
                "Superman"
            ),
            Err(CommonError::PersonaDataInvalidNameGivenNamesEmpty)
        );
    }

    #[test]
    fn empty_nickname_is_ok() {
        assert_eq!(
            PersonaDataEntryName::new(Variant::Western, "Kent", "Clark", "")
                .unwrap()
                .nickname,
            ""
        );
    }

    #[test]
    fn display_western() {
        let sample = PersonaDataEntryName::sample();
        assert_eq!(format!("{sample}"), "Bruce Batman Wayne")
    }

    #[test]
    fn display_eastern() {
        let sample = PersonaDataEntryName::sample_other();
        assert_eq!(format!("{sample}"), "Jun-fan Bruce Lee")
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = PersonaDataEntryName::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "variant": "western",
                "familyName": "Wayne",
                "givenNames": "Bruce",
                "nickname": "Batman"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_sample_other() {
        let model = PersonaDataEntryName::sample_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "variant": "eastern",
                "familyName": "Jun-fan",
                "givenNames": "Lee",
                "nickname": "Bruce"
            }
            "#,
        )
    }
}
