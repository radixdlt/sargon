use crate::prelude::*;

macro_rules! json_data_convertible {
    ($type: ty) => {
        paste! {
            use sargon::DeserializeBytes;
            use sargon::SerializeToBytes;

            #[uniffi::export]
            pub fn [< new_ $type:snake _from_json_bytes >](
                json_bytes: &BagOfBytes,
            ) -> Result<$type> {
                json_bytes.to_vec().deserialize::<[< Internal $type>]>().map_result()
            }

            #[uniffi::export]
            pub fn [< $type:snake _to_json_bytes >]([< $type:snake >]: &$type) -> BagOfBytes {
                [< $type:snake >].into_internal().serialize_to_bytes().unwrap().into()
            }
        }
    };
}

pub(crate) use json_data_convertible;

macro_rules! json_string_convertible {
    ($type: ty) => {
        paste! {
            use sargon::DeserializeStr;
            use sargon::SerializeToString;

            #[uniffi::export]
            pub fn [< new_ $type:snake _from_json_string >](
                json_string: String,
            ) -> Result<$type> {
                json_string.deserialize::<[< Internal $type>]>().map_result()
            }

            #[uniffi::export]
            pub fn [< $type:snake _to_json_string >]([< $type:snake >]: &$type) -> String {
                [< $type:snake >].into_internal().serialize_to_string()
            }
        }
    };
    ($type: ty, $invalid_json_string: literal) => {
        json_string_convertible!($type);
        paste! {
            #[cfg(test)]
            mod [< invalid_json_test_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn from_json_string_fail() {
                    let error: CommonError = SUT::new_from_json_string($invalid_json_string).unwrap_err();

                    match error {
                        CommonError::FailedToDeserializeJSONToValue {
                            json_byte_count,
                            type_name,
                            ..
                        } => {
                            assert_eq!(
                                25,
                                json_byte_count,
                            );

                            assert_eq!(
                                {{
                                    const STRINGIFIED: &'static str = stringify!($type);
                                    STRINGIFIED
                                }}.to_owned(),
                                type_name,
                            );
                        }
                        _ => {
                            panic!("Expected CommonError::FailedToDeserializeJSONToValue but other error occurred")
                        }
                    }
                }
            }
        }

    };
}

pub(crate) use json_string_convertible;
