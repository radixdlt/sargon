use crate::prelude::*;

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
pub struct Name {
    pub variant: Variant,
    pub family_name: String,
    pub given_names: String,
    pub nickname: String,
}

impl Name {
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

impl HasPlaceholder for Name {
    fn placeholder() -> Self {
        Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Should have a valid Name placeholder")
    }

    fn placeholder_other() -> Self {
        Name::new(Variant::Eastern, "Jun-fan", "Lee", "Bruce")
            .expect("Should have a valid Name placeholder")
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
        let name =
            Name::new(Variant::Western, "\n Wayne\n ", "  Bruce  ", "Batman ")
                .unwrap(); // testing trim
        assert_eq!(name.family_name, "Wayne");
        assert_eq!(name.given_names, "Bruce");
        assert_eq!(name.nickname, "Batman");
    }

    #[test]
    fn new_from_string_multiple_given_names() {
        let name = Name::new(
            Variant::Western,
            "Långstrump".to_string(),
            "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter"
                .to_string(),
            "Pippi".to_string(),
        )
        .unwrap();
        assert_eq!(
            name.given_names,
            "Pippilotta Viktualia Rullgardina Krusmynta Efraimsdotter"
        );
    }

    #[test]
    fn placeholder() {
        let placeholder = Name::placeholder();
        assert_eq!(placeholder.family_name, "Wayne");
        assert_eq!(placeholder.given_names, "Bruce");
        assert_eq!(placeholder.nickname, "Batman");
    }

    #[test]
    fn placeholder_other() {
        let placeholder = Name::placeholder_other();
        assert_eq!(placeholder.family_name, "Jun-fan");
        assert_eq!(placeholder.given_names, "Lee");
        assert_eq!(placeholder.nickname, "Bruce");
    }

    #[test]
    fn empty_family_name_is_err() {
        assert_eq!(
            Name::new(Variant::Western, "", "Clark", "Superman"),
            Err(CommonError::PersonaDataInvalidNameFamilyNameEmpty)
        );
    }

    #[test]
    fn spaces_trimmed_empty_family_name_is_err() {
        assert_eq!(
            Name::new(Variant::Western, "  ", "Clark", "Superman"),
            Err(CommonError::PersonaDataInvalidNameFamilyNameEmpty)
        );
    }

    #[test]
    fn empty_given_names_is_err() {
        assert_eq!(
            Name::new(Variant::Western, "Kent", "", "Superman"),
            Err(CommonError::PersonaDataInvalidNameGivenNamesEmpty)
        );
    }

    #[test]
    fn spaces_trimmed_empty_given_names_is_err() {
        assert_eq!(
            Name::new(Variant::Western, "Kent", " ", "Superman"),
            Err(CommonError::PersonaDataInvalidNameGivenNamesEmpty)
        );
    }

    #[test]
    fn empty_nickname_is_ok() {
        assert_eq!(
            Name::new(Variant::Western, "Kent", "Clark", "")
                .unwrap()
                .nickname,
            ""
        );
    }

    #[test]
    fn display_western() {
        let placeholder = Name::placeholder();
        assert_eq!(format!("{placeholder}"), "Bruce Batman Wayne")
    }

    #[test]
    fn display_eastern() {
        let placeholder = Name::placeholder_other();
        assert_eq!(format!("{placeholder}"), "Jun-fan Bruce Lee")
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = Name::placeholder();
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
    fn json_roundtrip_placeholder_other() {
        let model = Name::placeholder_other();
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
