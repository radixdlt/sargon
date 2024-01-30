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
    pub given_name: String,
    pub nickname: String,
}

impl Name {
    pub fn new(
        variant: Variant,
        family_name: impl AsRef<str>,
        given_name: impl AsRef<str>,
        nickname: impl AsRef<str>,
    ) -> Result<Self> {
        let family_name = family_name.as_ref().trim().to_string();
        let given_name = given_name.as_ref().trim().to_string();
        let nickname = nickname.as_ref().trim().to_string();
        if family_name.is_empty()
            || given_name.is_empty()
            || nickname.is_empty()
        {
            return Err(CommonError::InvalidDisplayNameEmpty);
        }
        Ok(Self {
            variant,
            family_name,
            given_name,
            nickname,
        })
    }

    fn full(&self) -> String {
        match self.variant {
            Variant::Western => format!(
                "{} {} {}",
                self.given_name, self.nickname, self.family_name
            ),
            Variant::Eastern => format!(
                "{} {} {}",
                self.family_name, self.nickname, self.given_name
            ),
        }
    }
}

impl HasPlaceholder for Name {
    fn placeholder() -> Self {
        Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Failed to construct placeholder")
    }

    fn placeholder_other() -> Self {
        Name::new(Variant::Eastern, "Jun-fan", "Lee", "Bruce")
            .expect("Failed to construct placeholder")
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
)]
pub enum Variant {
    Western,
    Eastern,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn new_name() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Name construction should not fail");
        assert_eq!(name.family_name, "Wayne");
        assert_eq!(name.given_name, "Bruce");
        assert_eq!(name.nickname, "Batman");
    }

    #[test]
    fn new_as_ref() {
        let name = Name::new(
            Variant::Western,
            "Wayne".to_string(),
            "Bruce".to_string(),
            "Batman".to_string(),
        )
        .expect("Name construction should not fail");
        assert_eq!(name.family_name, "Wayne");
        assert_eq!(name.given_name, "Bruce");
        assert_eq!(name.nickname, "Batman");
    }

    #[test]
    fn placeholder() {
        let placeholder = Name::placeholder();
        assert_eq!(placeholder.family_name, "Wayne");
        assert_eq!(placeholder.given_name, "Bruce");
        assert_eq!(placeholder.nickname, "Batman");
    }

    #[test]
    fn placeholder_other() {
        let placeholder = Name::placeholder_other();
        assert_eq!(placeholder.family_name, "Jun-fan");
        assert_eq!(placeholder.given_name, "Lee");
        assert_eq!(placeholder.nickname, "Bruce");
    }

    #[test]
    fn name_get_set() {
        let mut name = Name::placeholder();
        assert_eq!(name.family_name, "Wayne");
        assert_eq!(name.given_name, "Bruce");
        assert_eq!(name.nickname, "Batman");
        let new_name = Name::new(Variant::Western, "Kent", "Clark", "Superman")
            .expect("Name construction should not fail");
        name = new_name.clone();
        assert_eq!(name, new_name);
    }

    #[test]
    fn update() {
        let mut name = Name::placeholder();
        assert_eq!(name.family_name, "Wayne");
        let new_name = Name::new(Variant::Western, "Kent", "Clark", "Superman")
            .expect("Name construction should not fail");
        name.family_name = new_name.family_name;
        assert_eq!(name.family_name, "Kent");
    }

    #[test]
    fn empty_family_name() {
        let name = Name::new(Variant::Western, "", "Clark", "Superman");
        assert_eq!(name, Err(CommonError::InvalidDisplayNameEmpty));
    }

    #[test]
    fn empty_given_name() {
        let name = Name::new(Variant::Western, "Kent", "", "Superman");
        assert_eq!(name, Err(CommonError::InvalidDisplayNameEmpty));
    }

    #[test]
    fn empty_nickname() {
        let name = Name::new(Variant::Western, "Kent", "Clark", "");
        assert_eq!(name, Err(CommonError::InvalidDisplayNameEmpty));
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
                "variant": "Western",
                "familyName": "Wayne",
                "givenName": "Bruce",
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
                "variant": "Eastern",
                "familyName": "Jun-fan",
                "givenName": "Lee",
                "nickname": "Bruce"
            }
            "#,
        )
    }
}
