use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.string_representation(false))]
#[debug("{}", self.string_representation(true))]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    pub name: Option<PersonaDataIdentifiedName>,
    pub phone_numbers: CollectionOfPhoneNumbers,
    pub email_addresses: CollectionOfEmailAddresses,
}

impl PersonaData {
    pub fn new(
        name: Option<PersonaDataIdentifiedName>,
        phone_numbers: CollectionOfPhoneNumbers,
        email_addresses: CollectionOfEmailAddresses,
    ) -> Self {
        Self {
            name,
            phone_numbers,
            email_addresses,
        }
    }
}

trait DebugString {
    fn dbg_string(&self) -> String;
}
impl<U> DebugString for U
where
    U: std::fmt::Debug,
{
    fn dbg_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PersonaData {
    pub fn string_representation(&self, include_id: bool) -> String {
        let name = self
            .name
            .as_deref()
            .map(|v| {
                if include_id {
                    v.dbg_string()
                } else {
                    v.to_string()
                }
            })
            .map(|v| format!("name: {v}"));

        let phones = self
            .phone_numbers
            .iter()
            .cloned()
            .map(|v| {
                if include_id {
                    v.dbg_string()
                } else {
                    v.to_string()
                }
            })
            .map(|v| format!("phone: {v}"))
            .join("\n");

        let emails = self
            .email_addresses
            .iter()
            .cloned()
            .map(|v| {
                if include_id {
                    v.dbg_string()
                } else {
                    v.to_string()
                }
            })
            .map(|v| format!("email: {v}"))
            .join("\n");

        [name.unwrap_or_default(), phones, emails]
            .into_iter()
            .join("\n")
    }
}

impl HasPlaceholder for PersonaData {
    fn placeholder() -> Self {
        Self::new(
            Some(PersonaDataIdentifiedName::placeholder()),
            CollectionOfPhoneNumbers::placeholder(),
            CollectionOfEmailAddresses::placeholder(),
        )
    }

    fn placeholder_other() -> Self {
        Self::new(
            Some(PersonaDataIdentifiedName::placeholder_other()),
            CollectionOfPhoneNumbers::placeholder_other(),
            CollectionOfEmailAddresses::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(PersonaData::placeholder(), PersonaData::placeholder());
        assert_eq!(
            PersonaData::placeholder_other(),
            PersonaData::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PersonaData::placeholder(),
            PersonaData::placeholder_other()
        );
    }

    #[test]
    fn new_persona_data() {
        let name =
            Name::new(Variant::Western, "Skywalker", "Anakin", "Darth Vader")
                .unwrap();
        let persona_data = PersonaData {
            name: Some(PersonaDataIdentifiedName::with_id(
                Uuid::nil(),
                name.clone(),
            )),
            ..Default::default()
        };
        assert_eq!(
            persona_data.name,
            Some(PersonaDataIdentifiedName::with_id(
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                name
            ))
        );
    }

    #[test]
    fn placeholder() {
        let persona_data = PersonaData::placeholder();
        let identified_entry = PersonaDataIdentifiedName::placeholder();
        assert_eq!(persona_data.name, Some(identified_entry));
    }

    #[test]
    fn placeholder_other() {
        assert_eq!(
            PersonaData::placeholder_other().name,
            Some(PersonaDataIdentifiedName::placeholder_other())
        );
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = PersonaData::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "name": {
                    "id": "00000000-0000-0000-0000-000000000001",
                    "value": {
                        "variant": "Western",
                        "familyName": "Wayne",
                        "givenName": "Bruce",
                        "nickname": "Batman"
                    }
                },
                "phoneNumbers": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "value": "+46123456789"
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "value": "+44987654321"
                    }
                ],
                "emailAddresses": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "value": "alan@turing.hero"
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "value": "satoshi@nakamoto.btc"
                    }
                ]
            }
            "#,
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", PersonaData::placeholder()), "name: Bruce Batman Wayne\nphone: +46123456789\nphone: +44987654321\nemail: alan@turing.hero\nemail: satoshi@nakamoto.btc");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", PersonaData::placeholder()), "name: Bruce Batman Wayne\nphone: +46123456789 - 00000000-0000-0000-0000-000000000001\nphone: +44987654321 - 00000000-0000-0000-0000-000000000002\nemail: alan@turing.hero - 00000000-0000-0000-0000-000000000001\nemail: satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002");
    }
}
