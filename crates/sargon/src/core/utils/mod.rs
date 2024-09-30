mod factory;
mod image_url_utils;
mod image_url_utils_uniffi_fn;
mod logged_panic;
mod serialization;
mod string_utils;

pub use factory::*;
pub use image_url_utils::*;
pub use image_url_utils_uniffi_fn::*;
pub use logged_panic::*;
pub use serialization::*;
pub use string_utils::*;

pub fn type_name<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_typename() {
        struct GreatStruct {}
        assert_eq!(type_name::<GreatStruct>(), "GreatStruct");
    }
}
