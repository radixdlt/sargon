use crate::prelude::*;
use sargon_core_error::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

pub trait SerializeToBytes {
    fn serialize_to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait DeserializeFromBytes {
    fn deserialize_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Serialize> SerializeToBytes for T {
    fn serialize_to_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
    }
}

impl<T: DeserializeOwned> DeserializeFromBytes for T {
    fn deserialize_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: Sized,
    {
        let slice = bytes.as_ref();
        serde_json::from_slice(slice)
            .map_failed_to_deserialize_bytes::<Self>(slice)
    }
}

pub trait DeserializeBytes {
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T>;
}

impl DeserializeBytes for Vec<u8> {
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        T::deserialize_from_bytes(self)
    }
}

impl DeserializeBytes for &[u8] {
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        T::deserialize_from_bytes(self)
    }
}

pub trait SerializeToString {
    fn serialize_to_string(&self) -> String;
}

pub trait DeserializeFromString {
    fn deserialize_from_string(str: impl AsRef<str>) -> Result<Self>
    where
        Self: Sized;
}

impl<T: DeserializeOwned> DeserializeFromString for T {
    fn deserialize_from_string(str: impl AsRef<str>) -> Result<Self>
    where
        Self: Sized,
    {
        let json_string = str.as_ref().to_owned();
        let json_value = serde_json::Value::String(json_string.clone());
        serde_json::from_value(json_value)
            .map_failed_to_deserialize_string::<Self>(json_string)
    }
}

impl<T: Serialize> SerializeToString for T {
    fn serialize_to_string(&self) -> String {
        let value = serde_json::to_value(self).unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Self>()
            )
        });
        match value {
            serde_json::Value::String(str) => str.to_owned(),
            _ => unreachable!("never happen"),
        }
    }
}

pub trait DeserializeStr {
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T>;
}

impl DeserializeStr for str {
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        T::deserialize_from_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_from_slice() {
        let value = "hello";
        let serialized = value.serialize_to_bytes().unwrap();
        let deserialized = String::deserialize_from_bytes(&serialized).unwrap();
        pretty_assertions::assert_eq!(value, deserialized);
    }

    #[test]
    fn serialize() {
        let value = "hello";
        let serialized = value.serialize_to_bytes().unwrap();
        let deserialized: String =
            String::deserialize_from_bytes(&serialized).unwrap();
        pretty_assertions::assert_eq!(value, deserialized);
    }

    #[test]
    fn deserialize_from_slice_error() {
        let slice = b"invalid json";
        let result: Result<String> = String::deserialize_from_bytes(slice);
        assert!(matches!(
            result,
            Err(CommonError::FailedToDeserializeJSONToValue { .. })
        ));
    }

    #[test]
    fn deserialize_from_string() {
        let value = "hello";
        let serialized = value.serialize_to_string();
        let deserialized =
            String::deserialize_from_string(&serialized).unwrap();
        pretty_assertions::assert_eq!(value, deserialized);
    }
}
