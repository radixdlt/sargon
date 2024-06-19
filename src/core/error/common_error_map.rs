use de::StdError;

use crate::prelude::*;

pub trait MapToFailedToDeserializeJSONToValue<R> {
    fn map_failed_to_deserialize_string<T>(
        self,
        input: impl AsRef<str>,
    ) -> Result<R, CommonError>;
    fn map_failed_to_deserialize_bytes<T>(
        self,
        input: &[u8],
    ) -> Result<R, CommonError>;
}

impl<R> MapToFailedToDeserializeJSONToValue<R>
    for Result<R, serde_json::Error>
{
    fn map_failed_to_deserialize_string<T>(
        self,
        input: impl AsRef<str>,
    ) -> Result<R, CommonError> {
        self.map_err(|e| {
            error!(
                "Failed to deserialize JSON to {}, from:\n{:?}\nError: {}",
                type_name::<T>(),
                input.as_ref().to_string().chars().take(500),
                e
            );
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: input.as_ref().len() as u64,
                type_name: type_name::<T>(),
                message: format!("{}", e),
            }
        })
    }

    fn map_failed_to_deserialize_bytes<T>(
        self,
        input: &[u8],
    ) -> Result<R, CommonError> {
        self.map_err(|e| {
            error!(
                "Failed to deserialize JSON to {}, from (UTF-8):\n{:?}\nError: {}", 
                type_name::<T>(),
                String::from_utf8(input.to_vec()).map(|json| json.chars().take(500).collect_vec()),
                e
            );
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: input.len() as u64,
                type_name: type_name::<T>(),
                message: format!("{}", e)
            }
        })
    }
}
