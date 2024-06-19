use crate::prelude::*;

pub trait JsonStringDeserializing: for<'a> Deserialize<'a> {
    fn new_from_json_string(json: impl AsRef<str>) -> Result<Self> {
        let json_string = json.as_ref().to_owned();
        let json_value = serde_json::Value::String(json_string.clone());
        serde_json::from_value(json_value)
            .map_failed_to_deserialize_string::<Self>(json_string)
    }
}

pub trait JsonStringSerializing: Sized + Serialize {
    fn to_json_string(&self) -> String {
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

pub trait JsonDataDeserializing: for<'a> Deserialize<'a> {
    fn new_from_json_bytes(json: impl AsRef<[u8]>) -> Result<Self> {
        let json = json.as_ref();
        serde_json::from_slice::<Self>(json)
            .map_failed_to_deserialize_bytes::<Self>(json)
    }
}

pub trait JsonDataSerializing: Sized + Serialize {
    fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Self>()
            )
        })
    }
}

macro_rules! json_data_convertible {
    ($type: ty) => {
        paste! {

            impl JsonDataDeserializing for $type {}
            impl JsonDataSerializing for $type {}

            #[uniffi::export]
            pub fn [< new_ $type:snake _from_json_bytes >](
                json_bytes: &BagOfBytes,
            ) -> Result<$type> {
                $type::new_from_json_bytes(json_bytes)
            }

            #[uniffi::export]
            pub fn [< $type:snake _to_json_bytes >]([< $type:snake >]: &$type) -> BagOfBytes {
                [< $type:snake >].to_json_bytes().into()
            }

            #[cfg(test)]
            mod [< uniffi_test_json_as_data_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn json_bytes_roundtrip() {
                    let sut = SUT::sample();
                    let json_bytes = [< $type:snake _to_json_bytes >](&sut);
                    assert_eq!(sut, [< new_ $type:snake _from_json_bytes >](&json_bytes).unwrap());
                }
            }

            #[cfg(test)]
            mod [< test_json_as_data_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn json_bytes_roundtrip() {
                    let sut = SUT::sample();
                    let json_bytes = sut.to_json_bytes();
                    assert_eq!(sut, SUT::new_from_json_bytes(json_bytes).unwrap());
                }

                #[test]
                fn from_json_bytes_fail() {
                    let error: CommonError = SUT::new_from_json_bytes(BagOfBytes::sample()).unwrap_err();

                    match error {
                        CommonError::FailedToDeserializeJSONToValue {
                            json_byte_count,
                            type_name,
                            ..
                        } => {
                            assert_eq!(
                                32,
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
                            error!("Expected CommonError::FailedToDeserializeJSONToValue but other error occurred")
                        }
                    }
                }
            }
        }
    };
}

pub(crate) use json_data_convertible;

macro_rules! json_string_convertible {
    ($type: ty) => {
        paste! {

            impl JsonStringDeserializing for $type {}
            impl JsonStringSerializing for $type {}

            #[uniffi::export]
            pub fn [< new_ $type:snake _from_json_string >](
                json_string: String,
            ) -> Result<$type> {
                $type::new_from_json_string(json_string)
            }

            #[uniffi::export]
            pub fn [< $type:snake _to_json_string >]([< $type:snake >]: &$type) -> String {
                [< $type:snake >].to_json_string()
            }

            #[cfg(test)]
            mod [< uniffi_test_json_as_string_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn json_string_roundtrip() {
                    let sut = SUT::sample();
                    let json = [< $type:snake _to_json_string >](&sut);
                    assert_eq!(sut, [< new_ $type:snake _from_json_string >](json).unwrap())
                }
            }

            #[cfg(test)]
            mod [< test_json_as_string_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn json_string_roundtrip() {
                    let sut = SUT::sample();
                    let json_str = sut.to_json_string();
                    assert_eq!(SUT::new_from_json_string(json_str).unwrap(), sut)
                }
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
                            error!("Expected CommonError::FailedToDeserializeJSONToValue but other error occurred")
                        }
                    }
                }
            }
        }

    };
}

pub(crate) use json_string_convertible;
