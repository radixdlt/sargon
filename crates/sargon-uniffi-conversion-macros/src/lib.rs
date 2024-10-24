extern crate proc_macro;
mod common;
mod enum_conversion;
mod struct_conversion;
use enum_conversion::*;
use struct_conversion::*;

use core::panic;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

/// The proc macro that generates the From and Into implementations for the uniffi types.
///
/// Supported conversions:
/// - Unit enums like `enum MyEnum { Variant1, Variant2 }`
/// - Tuple enums like `enum MyEnum { Variant1(u32), Variant2(String) }`
/// - Named enums like `enum MyEnum { Variant1 { field1: u32 }, Variant2 { field2: String } }`
/// - Structs with named fields like `struct MyStruct { field1: u32, field2: String }`
/// - Structs with unnamed fields like `struct MyStruct(u32, String)`
/// - Structs with no fields like `struct MyStruct`
///
/// The main requirement for the above conversions to work is that the uniffi type matches
/// the internal type in terms of the fields for structs and variants for enums:
/// - For enums, the macro will lookup the variant in the internal enum based on the variant name
/// - For structs, the macro will lookup the field in the internal struct based on the field name or position.
/// - For both enums and structs, all of the enum variants and structs fields for the internal type must be present in the uniffi type.
///
/// The macro as well knows how to convert container types:
/// - Vec<Type> <-> Vec<InternalType>, where Type: From<InternalType>, Type: Into<InternalType>
/// - Vec<Type> <-> IdentifiedVecOf<InternalType>, where Type: From<InternalType>, Type: Into<InternalType>
/// - Option<Type> <-> Option<InternalType>, where Type: From<InternalType>, Type: Into<InternalType>
/// - HashMap<Key, Type> <-> HashMap<InternalKey, InternalType> where Type: From<InternalType>, Type: Into<InternalType>, Key: From<InternalKey>, Key: Into<InternalKey>
/// - Option<Vec<Type>> <-> Option<Vec<InternalType>>, where Type: From<InternalType>, Type: Into<InternalType>
///
/// The macro will generate the following conversions:
///
/// - Unit enums like `enum MyEnum { Variant1, Variant2 }`:
/// ```rust
/// impl From<InternalEnum> for UniffiEnumType {
///   fn from(value: InternalEnum) -> Self {
///     match value {
///       InternalEnum::Variant1 => UniffiEnumType::Variant1,
///       InternalEnum::Variant2 => UniffiEnumType::Variant2,
///     }
///   }
/// }
///
/// impl Into<InternalEnum> for UniffiEnumType {
///   fn into(self) -> InternalEnum {
///     match self {
///       UniffiEnumType::Variant1 => InternalEnum::Variant1,
///       UniffiEnumType::Variant2 => InternalEnum::Variant2,
///     }
///   }
/// }
/// ```
///
/// - Tuple enums like `enum MyEnum { Variant1(u32), Variant2(String) }`:
/// ```rust
/// impl From<InternalEnum> for UniffiEnumType {
///   fn from(value: InternalEnum) -> Self {
///     match value {
///       InternalEnum::Variant1(inner) => UniffiEnumType::Variant1(inner.into()),
///       InternalEnum::Variant2(inner) => UniffiEnumType::Variant2(inner.into()),
///     }
///   }
/// }
///
/// impl Into<InternalEnum> for UniffiEnumType {
///   fn into(self) -> InternalEnum {
///     match self {
///       UniffiEnumType::Variant1(inner) => InternalEnum::Variant1(inner.into()),
///       UniffiEnumType::Variant2(inner) => InternalEnum::Variant2(inner.into()),
///     }
///   }
/// }
/// ```
///
/// - Named enums like `enum MyEnum { Variant1 { field1: u32 }, Variant2 { field2: String } }`:
/// ```rust
/// impl From<InternalEnum> for UniffiEnumType {
///   fn from(value: InternalEnum) -> Self {
///     match value {
///       InternalEnum::Variant1 { field1 } => UniffiEnumType::Variant1 { field1: field1.into() },
///       InternalEnum::Variant2 { field2 } => UniffiEnumType::Variant2 { field2: field2.into() },
///     }
///   }
/// }
///
/// impl Into<InternalEnum> for UniffiEnumType {
///   fn into(self) -> InternalEnum {
///     match self {
///       UniffiEnumType::Variant1 { field1 } => InternalEnum::Variant1 { field1: field1.into() },
///       UniffiEnumType::Variant2 { field2 } => InternalEnum::Variant2 { field2: field2.into() },
///     }
///   }
/// }
/// ```
///
/// - Struct with name fields
/// ```rust
/// impl From<InternalStruct> for UniffiStructType {
///    fn from(value: InternalType) -> Self {
///         Self {
///            field1: value.field1.into(),
///           field2: value.field2.into(),
///         }
///    }
/// }
///
/// impl Into<InternalStruct> for UniffiStructType {
///   fn into(self) -> InternalType {
///      InternalType {
///        field1: self.field1.into(),
///       field2: self.field2.into(),
///    }
/// }
/// ```
///
/// - Struct with unnamed fields
/// ```rust
/// impl From<InternalStruct> for UniffiStructType {
///   fn from(value: InternalType) -> Self {
///     Self(value.0.into(), value.1.into())
///   }
/// }
///
/// impl Into<InternalStruct> for UniffiStructType {
///   fn into(self) -> InternalType {
///     (self.0.into(), self.1.into())
///   }
/// }
/// ```
///
/// - Struct with no fields
/// ```rust
/// impl From<InternalStruct> for UniffiStructType {
///   fn from(_: InternalType) -> Self {
///     Self
///   }
/// }
///
/// impl Into<InternalStruct> for UniffiStructType {
///   fn into(self) -> InternalType {
///     InternalType
///   }
/// }
/// ```
#[proc_macro_derive(InternalConversion)]
pub fn internal_conversion_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Enum(data) => handle_enum(&name, data),
        Data::Struct(data) => handle_struct(&name, data),
        _ => panic!(
            "InternalConversion can only be derived for structs or enums"
        ),
    };

    TokenStream::from(expanded)
}
