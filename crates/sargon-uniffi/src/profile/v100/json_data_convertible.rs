use crate::prelude::*;

macro_rules! json_data_convertible {
    ($type: ty) => {
        paste! {
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
}

pub(crate) use json_string_convertible;
