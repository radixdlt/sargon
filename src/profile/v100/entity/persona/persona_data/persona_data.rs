use crate::prelude::*;

/// Personal information a user has associated with a certain Persona, of different kinds, such as name,
/// email address(es) or phone number(s). This information is only ever stored in Profile and is never
/// uploaded to the Radix Network.
///
/// These entries of different kinds can be queried for in a request sent by a dApp, e.g.
/// Radix Dashboard might ask "Give me ongoing access to Name and 2 Email addresses for
/// a Persona" (just a silly example, Radix Dashboard would never ask for that and why 2 email addresses?).
///
/// The Profile will then use the fact that each Persona Data Entry has a stable ID so that Profile can
/// refer the entry just by the ID, and Profile can thus record which Persona Data Entry a user has selected
/// to share with the dApp, without duplicating the value of that entry (just like how we use FactorSourceIDs).
/// Since a dApp can ask for *ongoing* access next time the user interacts with the same dApp, if user has
/// not revoked the dApps access, the wallet clients will automatically send back the Persona Data Entry values
/// even if they have been updated - the value might have changed but their IDs have not. Thus if a user
/// deletes a Persona Data Entry (e.g. a phone number), and later re-inputs the same phone number, even
/// it the exact same value is used, it will still be treated as a new entry since its ID is new, meaning
/// that the next time the user interacts with a previously authorized dApp the wallet cannot automatically
/// respond back to dApp with the PersonaData, but user will have to re-authorize the request for ongoing
/// access for the requested PersonaData entries.
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
    /// A persons name they have chosen to associated with a Persona, e.g. "Bruce 'Batman' Wayne" using Western name variant,
    /// or `"Jun-fan 'Bruce' Lee"` using Eastern name variant (family name comes before given name(s)).
    ///
    /// Note that the type is Option of `PersonaDataIdentifiedName` and not of type [`PersonaDataEntryName`][name],
    /// `PersonaDataIdentifiedName` is essentially a tuple of `(Uuid, PersonaDataEntryName)`.
    ///
    /// [name]: PersonaDataEntryName
    pub name: Option<PersonaDataIdentifiedName>,

    /// A collection of [`PersonaDataIdentifiedPhoneNumber`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataEntryPhoneNumber)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    pub phone_numbers: CollectionOfPhoneNumbers,

    /// A collection of [`PersonaDataEntryEmailAddress`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataIdentifiedEmailAddress)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    pub email_addresses: CollectionOfEmailAddresses,
}

impl PersonaData {
    pub fn new(
        name: impl Into<Option<PersonaDataIdentifiedName>>,
        phone_numbers: CollectionOfPhoneNumbers,
        email_addresses: CollectionOfEmailAddresses,
    ) -> Self {
        Self {
            name: name.into(),
            phone_numbers,
            email_addresses,
        }
    }
}

/// Private trait giving syntax sugar `dbg_string()` of
/// `std::fmt::Debug` types, exactly like `to_string()` of
/// for `std::fmt::Display
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
    /// A textual representation of all present entries of this PersonaData,
    /// optionally their IDs are included if `include_id` is `true`.
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

impl HasSampleValues for PersonaData {
    fn sample() -> Self {
        Self::new(
            PersonaDataIdentifiedName::sample(),
            CollectionOfPhoneNumbers::sample(),
            CollectionOfEmailAddresses::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            PersonaDataIdentifiedName::sample_other(),
            CollectionOfPhoneNumbers::sample_other(),
            CollectionOfEmailAddresses::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(PersonaData::sample(), PersonaData::sample());
        assert_eq!(PersonaData::sample_other(), PersonaData::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(PersonaData::sample(), PersonaData::sample_other());
    }

    #[test]
    fn new_persona_data() {
        let name = PersonaDataEntryName::new(
            PersonaDataNameVariant::Western,
            "Skywalker",
            "Anakin",
            "Darth Vader",
        )
        .unwrap();

        let persona_data = PersonaData {
            name: Some(PersonaDataIdentifiedName::with_id(
                PersonaDataEntryID::sample_one(),
                name.clone(),
            )),
            ..Default::default()
        };

        assert_eq!(
            persona_data.name,
            Some(PersonaDataIdentifiedName::with_id(
                "00000000-0000-0000-0000-000000000001".parse().unwrap(),
                name
            ))
        );
    }

    #[test]
    fn sample() {
        let persona_data = PersonaData::sample();
        let identified_entry = PersonaDataIdentifiedName::sample();
        assert_eq!(persona_data.name, Some(identified_entry));
    }

    #[test]
    fn sample_other() {
        assert_eq!(
            PersonaData::sample_other().name,
            Some(PersonaDataIdentifiedName::sample_other())
        );
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = PersonaData::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "name": {
                    "id": "00000000-0000-0000-0000-000000000001",
                    "value": {
                        "variant": "western",
                        "familyName": "Wayne",
                        "givenNames": "Bruce",
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
        assert_eq!(format!("{}", PersonaData::sample()), "name: Bruce Batman Wayne\nphone: +46123456789\nphone: +44987654321\nemail: alan@turing.hero\nemail: satoshi@nakamoto.btc");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", PersonaData::sample()), "name: Bruce Batman Wayne\nphone: +46123456789 - 00000000-0000-0000-0000-000000000001\nphone: +44987654321 - 00000000-0000-0000-0000-000000000002\nemail: alan@turing.hero - 00000000-0000-0000-0000-000000000001\nemail: satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002");
    }
}
