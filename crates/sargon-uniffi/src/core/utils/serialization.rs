use crate::prelude::*;

pub fn deserialize_from_slice<T>(slice: &[u8]) -> Result<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    serde_json::from_slice(slice).map_err(|err| {
        let type_name = std::any::type_name::<T>().to_string();
        error!(
            "Deserialize json to type: {}\nJSON (utf8):\n{:?}",
            &type_name,
            String::from_utf8(slice.to_vec())
        );
        CommonError::FailedToDeserializeJSONToValue {
            json_byte_count: slice.len() as u64,
            type_name,
            serde_message: err.to_string(),
        }
    })
}

pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    serde_json::to_vec(value).map_err(|_| CommonError::FailedToSerializeToJSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_from_slice() {
        let value = "hello";
        let serialized = serde_json::to_vec(&value).unwrap();
        let deserialized: String =
            super::deserialize_from_slice(&serialized).unwrap();
        pretty_assertions::assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize() {
        let value = "hello";
        let serialized = super::serialize(&value).unwrap();
        let deserialized: String = serde_json::from_slice(&serialized).unwrap();
        pretty_assertions::assert_eq!(value, deserialized);
    }

    #[test]
    fn deserialize_from_slice_error() {
        let slice = b"invalid json";
        let result: Result<String> = super::deserialize_from_slice(slice);
        assert!(matches!(
            result,
            Err(CommonError::FailedToDeserializeJSONToValue { .. })
        ));
    }
}
