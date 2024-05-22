mod factory;
mod json_data_convertible;
mod logged_panic;
mod string_utils;

pub use factory::*;
pub use json_data_convertible::*;
pub use logged_panic::*;
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
