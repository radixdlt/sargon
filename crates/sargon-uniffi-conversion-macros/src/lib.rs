extern crate proc_macro;
mod enum_conversion;
mod struct_conversion;
mod common;
use struct_conversion::*;
use enum_conversion::*;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, PathArguments, Type, TypePath};
use proc_macro2::Ident;

/// TODO: Clean up and document this code

#[proc_macro_derive(InternalConversionV2)]
pub fn internal_conversion_derive_v2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Enum(data) => {
            handle_enum(&name, data)
        },
        Data::Struct(data) => {
            handle_struct(&name, data)
        },
        _ => panic!("InternalConversion can only be derived for structs or enums"),
    };


    TokenStream::from(expanded)
}