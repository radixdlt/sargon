use crate::prelude::*;

pub trait JsonDataDeserializing: for<'a> Deserialize<'a> {
    fn new_from_json_bytes(json: impl AsRef<[u8]>) -> Result<Self> {
        let json = json.as_ref();
        serde_json::from_slice::<Self>(json).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: json.len() as u64,
                type_name: type_name::<Self>(),
            }
        })
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
                json_bytes: BagOfBytes,
            ) -> Result<$type> {
                $type::new_from_json_bytes(json_bytes)
            }

            #[uniffi::export]
            pub fn [< $type:snake _to_json_bytes >]([< $type:snake >]: &$type) -> BagOfBytes {
                [< $type:snake >].to_json_bytes().into()
            }

            #[cfg(test)]
            mod [< uniffi_test_ $type:snake >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $type;

                #[test]
                fn json_bytes_roundtrip() {
                    let sut = SUT::sample();
                    let json_bytes = [< $type:snake _to_json_bytes >](&sut);
                    assert_eq!(sut, [< new_ $type:snake _from_json_bytes >](json_bytes).unwrap());
                }
            }

            #[cfg(test)]
            mod [< test_ $type:snake >] {
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
                    assert_eq!(
                        SUT::new_from_json_bytes(BagOfBytes::sample()),
                        Err(CommonError::FailedToDeserializeJSONToValue {
                            json_byte_count: 32,
                            type_name: {{
                                const STRINGIFIED: &'static str = stringify!($type);
                                STRINGIFIED
                            }}.to_owned()
                        })
                    );
                }
            }
        }
    };
}

pub(crate) use json_data_convertible;
