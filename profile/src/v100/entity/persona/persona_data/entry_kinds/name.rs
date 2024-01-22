use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    uniffi::Record,
    Default,
)]
pub struct Name {
    variant: Variant,
    family_name: String,
    given_name: String,
    nickname: String,
}

impl Name {
    pub fn new(
        variant: Variant,
        family_name: &str,
        given_name: &str,
        nickname: &str,
    ) -> Self {
        Self {
            variant,
            family_name: family_name.to_string(),
            given_name: given_name.to_string(),
            nickname: nickname.to_string(),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            Variant::Western => write!(
                f,
                "{} {} {}",
                self.given_name, self.nickname, self.family_name
            ),
            Variant::Eastern => write!(
                f,
                "{} {} {}",
                self.family_name, self.nickname, self.given_name
            ),
        }
    }
}

impl HasPlaceholder for Name {
    fn placeholder() -> Self {
        Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
    }

    fn placeholder_other() -> Self {
        Name::new(Variant::Eastern, "Jun-fan", "Lee", "Bruce")
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Enum,
)]
pub enum Variant {
    Western,
    Eastern,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Western
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    #[test]
    fn new_name() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman");
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
        let new_name = Name::new(Variant::Western, "Kent", "Clark", "Superman");
        name = new_name.clone();
        assert_eq!(name, new_name);
    }

    #[test]
    fn update() {
        let mut name = Name::placeholder();
        assert_eq!(name.family_name, "Wayne");
        let new_name = Name::new(Variant::Western, "Kent", "Clark", "Superman");
        name.family_name = new_name.family_name;
        assert_eq!(name.family_name, "Kent");
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
            r#"{
                                "variant": "Western",
                                "family_name": "Wayne",
                                "given_name": "Bruce",
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
            r#"{
                                "variant": "Eastern",
                                "family_name": "Jun-fan",
                                "given_name": "Lee",
                                "nickname": "Bruce"
                            }
                            "#,
        )
    }
}
