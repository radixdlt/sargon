extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, PathArguments, Type, TypePath};
use proc_macro2::Ident;

/// TODO: Clean up and document this code

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
                self.clone().into()
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}

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

#[proc_macro_derive(InternalConversionV3)]
pub fn internal_conversion_derive_v3(input: TokenStream) -> TokenStream {
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

fn handle_enum(name: &syn::Ident, data: syn::DataEnum) -> proc_macro2::TokenStream {
    let internal_name = quote::format_ident!("Internal{}", name);
    let test_mod_name = Ident::new(&format!("{}_tests", name.to_string().to_lowercase()), name.span());

    // Build the match arms for the `From` implementation
    let from_match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(_) => {
                quote! {
                    #internal_name::#variant_name(inner) => Self::#variant_name(inner.into())
                }
            },
            Fields::Unit => {
                quote! {
                    #internal_name::#variant_name => Self::#variant_name
                }
            },
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions: Vec<_> = generate_field_conversions(&fields);
                quote! {
                    #internal_name::#variant_name { #( #field_names ),* } => Self::#variant_name { #( #field_conversions ),* }
                }
            }
        }
    });

    // Build the match arms for the `Into` implementation
    let into_match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(_) => {
                quote! {
                    Self::#variant_name(inner) => #internal_name::#variant_name(inner.into())
                }
            },
            Fields::Unit => {
                quote! {
                    Self::#variant_name => #internal_name::#variant_name
                }
            },
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions: Vec<_> = generate_internal_field_conversions(&fields);
                quote! {
                    Self::#variant_name { #( #field_names ),* } => #internal_name::#variant_name  { #( #field_conversions ),* }
                }
            },
            _ => panic!("FromInto macro supports only tuple-style or unit-style variants"),
        }
    });

    // Generate the test cases for the conversion functions.
    // This will test each enum variant to ensure that it can be converted to the internal type and back.
    let test_cases = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let test_func_name = Ident::new(&format!("test_roundtrip_conversion_for_{}", variant_name.to_string().to_lowercase()), variant_name.span());
        match &variant.fields {
            // Tuple-style (unnamed fields) variant
            Fields::Unnamed(_) => {
                quote! {
                    #[test]
                    fn #test_func_name() {
                        let original = #internal_name::#variant_name(HasSampleValues::sample());
                        let converted: #name = original.clone().into();
                        let roundtrip: #internal_name = converted.into();
                        assert_eq!(original, roundtrip);
                    }
                }
            },
            // Unit-style variant (no fields)
            Fields::Unit => {
                quote! {
                    #[test]
                    fn #test_func_name() {
                        let original = #internal_name::#variant_name;
                        let converted: #name = original.clone().into();
                        let roundtrip: #internal_name = converted.into();
                        assert_eq!(original, roundtrip);
                    }
                }
            },
            // Struct-style variant with named fields
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                let field_defaults: Vec<_> = field_names.iter().map(|_| quote! { HasSampleValues::sample() }).collect();
                quote! {
                    #[test]
                    fn #test_func_name() {
                        let original = #internal_name::#variant_name {
                            #( #field_names: #field_defaults ),*
                        };
                        let converted: #name = original.clone().into();
                        let roundtrip: #internal_name = converted.into();
                        assert_eq!(original, roundtrip);
                    }
                }
            },
            _ => panic!("FromInto macro supports only tuple-style, unit-style, or struct-style variants"),
        }
    });

    // Generate the final implementation code
    quote! {
        impl #name {
            pub fn into_internal(&self) -> #internal_name {
                self.clone().into()
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

        #[cfg(test)]
        mod #test_mod_name {
            use super::*;

            #(#test_cases)*
        }
    }
}

fn handle_struct(name: &syn::Ident, data: syn::DataStruct) -> proc_macro2::TokenStream {
    let internal_name = quote::format_ident!("Internal{}", name);
    let test_mod_name = Ident::new(&format!("{}_tests", name.to_string().to_lowercase()), name.span());
    match data.fields {
        // Named fields: e.g., struct Foo { x: i32, y: String }
        Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named.named.iter().map(|f| &f.ident).collect();
            let field_from_internal_conversions: Vec<_> = generate_struct_field_conversions(fields_named);
            let field_into_internal_conversions: Vec<_> = generate_struct_internal_field_conversions(fields_named);

            quote! {
                impl #name {
                    pub fn into_internal(&self) -> #internal_name {
                        self.clone().into()
                    }
                }

                impl From<#internal_name> for #name {
                    fn from(value: #internal_name) -> Self {
                        #name {
                            #( #field_from_internal_conversions ),*
                        }
                    }
                }

                impl Into<#internal_name> for #name {
                    fn into(self) -> #internal_name {
                        #internal_name {
                            #( #field_into_internal_conversions ),*
                        }
                    }
                }
            }
        },
        Fields::Unnamed(ref field_unnamed) => {
            let field_from_internal_conversions: Vec<_> = generate_struct_unnamed_field_conversions(field_unnamed);
            let field_into_internal_conversions: Vec<_> = generate_struct_unnamed_field_internal_conversions(field_unnamed);
            quote! {
                impl #name {
                    pub fn into_internal(&self) -> #internal_name {
                        self.clone().into()
                    }
                }

                impl From<#internal_name> for #name {
                    fn from(value: #internal_name) -> Self {
                        #name(
                            #( #field_from_internal_conversions ),*
                        )
                    }
                }

                impl Into<#internal_name> for #name {
                    fn into(self) -> #internal_name {
                        #internal_name(
                            #( #field_into_internal_conversions ),*
                        )
                    }
                }
            }
        },
        Fields::Unit => {
            quote! {
                impl #name {
                    pub fn into_internal(&self) -> #internal_name {
                        self.clone().into()
                    }
                }

                impl From<#internal_name> for #name {
                    fn from(_: #internal_name) -> Self {
                        #name
                    }
                }

                impl Into<#internal_name> for #name {
                    fn into(self) -> #internal_name {
                        #internal_name
                    }
                }
            }
        },
    }

}

fn generate_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        match &f.ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        // Call into_type() for Vec types
                        quote! {
                            #field_name: #field_name.into_type()
                        }
                    } else if segment.ident == "HashMap" {
                        // Call into_hash_map() for HashMap types
                        quote! {
                            #field_name: #field_name.into_hash_map()
                        }
                    } else if segment.ident == "Option" {
                        if let Some(inner_type) = extract_inner_type(type_path) {
                            if let Type::Path(inner_type_path) = inner_type {
                                if let Some(inner_segment) = inner_type_path.path.segments.last() {
                                    if inner_segment.ident == "Vec" {
                                        // Call into_type() for Option<Vec<T>> types
                                        return quote! {
                                            #field_name: #field_name.map(|v| v.into_type())
                                        };
                                    }
                                }
                            }
                        }

                        // Call into_option() for Option types
                        quote! {
                            #field_name: #field_name.map(|v| v.into())
                        }
                    }
                    else {
                        // Default to calling .into() for other types
                        quote! {
                            #field_name: #field_name.into()
                        }
                    }
                } else {
                    // Default case if segment is missing
                    quote! {
                        #field_name: #field_name.into()
                    }
                }
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    #field_name: #field_name.into()
                }
            }
        }
    }).collect()
}

// Helper function to extract the inner type of an Option
fn extract_inner_type(type_path: &TypePath) -> Option<&Type> {
    if let Some(segment) = type_path.path.segments.last() {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                return Some(inner_type);
            }
        }
    }
    None
}

fn extract_inner_type_ident(type_path: &TypePath) -> Option<&Ident> {
    if let Some(segment) = type_path.path.segments.last() {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                if let Type::Path(inner_type_path) = inner_type {
                    if let Some(inner_segment) = inner_type_path.path.segments.last() {
                        return Some(&inner_segment.ident);
                    }
                }
            }
        }
    }
    None
}

fn generate_internal_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        match &f.ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        // Call into_type() for Vec types
                        quote! {
                            #field_name: #field_name.into_internal()
                        }
                    } else if segment.ident == "HashMap" {
                        // Call into_hash_map() for HashMap types
                        quote! {
                            #field_name: #field_name.into_internal_hash_map()
                        }
                    } else if segment.ident == "Option" {
                        if let Some(inner_type) = extract_inner_type(type_path) {
                            if let Type::Path(inner_type_path) = inner_type {
                                if let Some(inner_segment) = inner_type_path.path.segments.last() {
                                    if inner_segment.ident == "Vec" {
                                        // Call into_type() for Option<Vec<T>> types
                                        return quote! {
                                            #field_name: #field_name.map(|v| v.into_internal())
                                        };
                                    }
                                }
                            }
                        }
                        // Call into_option() for Option types
                        quote! {
                            #field_name: #field_name.map(|v| v.into())
                        }
                    } 
                    else {
                        // Default to calling .into() for other types
                        quote! {
                            #field_name: #field_name.into()
                        }
                    }
                } else {
                    // Default case if segment is missing
                    quote! {
                        #field_name: #field_name.into()
                    }
                }
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    #field_name: #field_name.into()
                }
            }
        }
    }).collect()
}

fn generate_struct_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        match &f.ty {
            Type::Path(type_path) => {
                generate_fields(type_path, field_name, false)
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    #field_name: value.#field_name.into()
                }
            }
        }
    }).collect()
}

fn generate_struct_internal_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        match &f.ty {
            Type::Path(type_path) => {
                generate_fields(type_path, field_name, true)
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    #field_name: self.#field_name.into()
                }
            }
        }
    }).collect()
}

fn generate_struct_unnamed_field_conversions(fields: &syn::FieldsUnnamed) -> Vec<proc_macro2::TokenStream> {
    fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = syn::Index::from(i);
        match &f.ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        // Call into_type() for Vec types
                        quote! {
                            value.#index.into_type()
                        }
                    } else if segment.ident == "HashMap" {
                        // Call into_hash_map() for HashMap types
                        quote! {
                            value.#index.into_hash_map()
                        }
                    } else {
                        // Default to calling .into() for other types
                        quote! {
                            value.#index.into()
                        }
                    }
                } else {
                    // Default case if segment is missing
                    quote! {
                        value.#index.into()
                    }
                }
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    value.#index.into()
                }
            }
        }
    }).collect()
}

fn generate_struct_unnamed_field_internal_conversions(fields: &syn::FieldsUnnamed) -> Vec<proc_macro2::TokenStream> {
    fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = syn::Index::from(i);
        match &f.ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        // Call into_type() for Vec types
                        quote! {
                            self.#index.into_internal()
                        }
                    } else if segment.ident == "HashMap" {
                        quote! {
                            self.#index.into_inernal_hash_map()
                        }
                    } else {
                        // Default to calling .into() for other types
                        quote! {
                            self.#index.into()
                        }
                    }
                } else {
                    // Default case if segment is missing
                    quote! {
                        self.#index.into()
                    }
                }
            }
            _ => {
                // Default case for non-Path types
                quote! {
                    self.#index.into()
                }
            }
        }
    }).collect()
}

fn generate_fields(type_path: &TypePath, field_name: &Option<Ident>, into_internal: bool) -> proc_macro2::TokenStream {
    let prefix_str= if into_internal { "self" } else { "value" };
    let prefix = Ident::new(prefix_str, proc_macro2::Span::call_site());

    let method_call: proc_macro2::TokenStream = if let Some(segment) = type_path.path.segments.last() {
        if segment.ident == "Vec" {
        
            if into_internal {
                quote! { into_internal() }
            } else {
                quote! { into_type() }
            }
        } else if segment.ident == "HashMap" {
            if into_internal {
                 quote! { into_internal_hash_map() }
            } else {
                 quote! { into_hash_map() }
            }
        } else if segment.ident == "Option" {
            if let Some(inner_type) = extract_inner_type_ident(type_path) {
                        if inner_type == "Vec" {
                            if into_internal {
                                quote! { map(|v| v.into_internal()) }
                            } else {
                                quote! { map(|v| v.into_type()) }
                            }
                        } else {
                            quote! { map(|v| v.into()) }
                        }
            } else {
            quote! { map(|v| v.into()) }
            }
        }
        else {
            quote! { into() }
        }
    } else {
        quote! { into() }
    };
    
    return quote! {
        #field_name: #prefix.#field_name.#method_call
    }
}