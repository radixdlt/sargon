use std::any::TypeId;

use crate::prelude::*;

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
        if TypeId::of::<V>() == TypeId::of::<FactorSource>()
            && self.0.is_empty()
        {
            return Err(serde::ser::Error::custom(
                CommonError::FactorSourcesMustNotBeEmpty,
            ));
        }
        if TypeId::of::<V>() == TypeId::of::<SLIP10Curve>() && self.0.is_empty()
        {
            return Err(serde::ser::Error::custom(
                CommonError::SupportedCurvesMustNotBeEmpty,
            ));
        }
        serializer.collect_seq(self)
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
        let mut map = Self::new();
        for item in items {
            map.try_insert_unique(item).map_err(de::Error::custom)?;
        }

        if TypeId::of::<V>() == TypeId::of::<FactorSource>() && map.0.is_empty()
        {
            return Err(de::Error::custom(
                CommonError::FactorSourcesMustNotBeEmpty,
            ));
        }
        if TypeId::of::<V>() == TypeId::of::<SLIP10Curve>() && map.0.is_empty()
        {
            return Err(de::Error::custom(
                CommonError::SupportedCurvesMustNotBeEmpty,
            ));
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {

    use super::super::User;
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
