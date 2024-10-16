use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, PathArguments, Type, TypePath};
use proc_macro2::Ident;

pub fn conversion_call(field: &Field, into_internal: bool) -> proc_macro2::TokenStream {
    match &field.ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
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
            }
        },
        _ => {
            quote! { into() }
        }
    }
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