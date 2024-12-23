use std::any::TypeId;

use crate::prelude::*;
use serde::de::Error;
use serde::de::*;
use serde::ser::*;

use super::{export_identified_vec_of, import_identified_vec_of_from};

impl<V: Debug + PartialEq + Eq + Clone + Identifiable + 'static> Serialize
    for IdentifiedVecOf<V>
where
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        export_identified_vec_of(self)
            .map_err(serde::ser::Error::custom)
            .and_then(|v| serializer.collect_seq(v))
    }
}

impl<'de, V: Debug + PartialEq + Eq + Clone + Identifiable + 'static>
    Deserialize<'de> for IdentifiedVecOf<V>
where
    V: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let items = Vec::<V>::deserialize(deserializer)?;
        import_identified_vec_of_from(items).map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "id": 0,
                    "name": "Alice"
                },
                {
                    "id": 2,
                    "name": "Carol"
                },
                {
                    "id": 4,
                    "name": "Erin"
                },
                {
                    "id": 6,
                    "name": "Grace"
                }
            ]
            "#,
        );
    }

    #[test]
    fn duplicates_in_json_throws() {
        let json = r#"
        [
            {
                "id": 0,
                "name": "Alice"
            },
            {
                "id": 0,
                "name": "Alice"
            },
            {
                "id": 2,
                "name": "Carol"
            },
            {
                "id": 4,
                "name": "Erin"
            },
            {
                "id": 6,
                "name": "Grace"
            }
        ]
        "#;
        assert!(serde_json::from_str::<SUT>(json).is_err());
    }
}
