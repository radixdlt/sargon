use crate::prelude::*;

use super::{export_ordered_set, import_ordered_set_from};

impl<V: PartialEq + Eq + Clone + std::hash::Hash + 'static> Serialize
    for OrderedSet<V>
where
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        export_ordered_set(self)
            .map_err(serde::ser::Error::custom)
            .and_then(|v| serializer.collect_seq(v))
    }
}

impl<'de, V: PartialEq + Eq + Clone + std::hash::Hash + 'static>
    Deserialize<'de> for OrderedSet<V>
where
    V: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let items = Vec::<V>::deserialize(deserializer)?;
        import_ordered_set_from(items).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OrderedSet<User>;

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
