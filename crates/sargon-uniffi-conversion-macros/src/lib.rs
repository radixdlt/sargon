extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(InternalConversion)]
pub fn internal_conversion_derive(input: TokenStream) -> TokenStream {
    // Parse the input into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type the macro is applied to
    let name = input.ident;

    // Construct the name of the internal type by prefixing with "Internal"
    let internal_name = quote::format_ident!("Internal{}", name);

    // Generate the implementation of the `into_internal()` function
    let expanded = quote! {
        impl #name {
            pub fn into_internal(&self) -> #internal_name {
                unimplemented!()
                //self.clone().into()
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}

#[proc_macro_derive(InternalConversionV2)]
pub fn internal_conversion_derive_v2(input: TokenStream) -> TokenStream {
    // Parse the input into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type the macro is applied to
    let name = input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("FromInto can only be derived for enums"),
    };

    // Construct the name of the internal type by prefixing with "Internal"
    let internal_name = quote::format_ident!("Internal{}", name);

    let from_match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            // Tuple-style (unnamed fields) variant
            Fields::Unnamed(_) => {
                quote! {
                    #internal_name::#variant_name(inner) => Self::#variant_name(inner.into())
                }
            },
            // Unit-style variant (no fields)
            Fields::Unit => {
                quote! {
                    #internal_name::#variant_name => Self::#variant_name
                }
            },
            _ => panic!("FromInto macro supports only tuple-style or unit-style variants"),
        }
    });

    let into_match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            // Tuple-style (unnamed fields) variant
            Fields::Unnamed(_) => {
                quote! {
                    Self::#variant_name(inner) => #internal_name::#variant_name(inner.into())
                }
            },
            // Unit-style variant (no fields)
            Fields::Unit => {
                quote! {
                    Self::#variant_name => #internal_name::#variant_name
                }
            },
            _ => panic!("FromInto macro supports only tuple-style or unit-style variants"),
        }
    });

    // Generate the implementation of the `into_internal()` function
    let expanded = quote! {
        impl #name {
            pub fn into_internal(&self) -> #internal_name {
                unimplemented!()
                //self.clone().into()
            }
        }

        impl From<#internal_name> for #name {
            fn from(value: #internal_name) -> Self {
                match value {
                    #(#from_match_arms,)*
                }
            }
        }

        impl Into<#internal_name> for #name {
            fn into(self) -> #internal_name {
                match self {
                    #(#into_match_arms,)*
                }
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
