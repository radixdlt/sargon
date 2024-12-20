use proc_macro2::Ident;
use quote::quote;
use syn::{Field, Type, TypePath};

/// Determines the conversion call to be used for the given field.
pub fn conversion_call(
    field: &Field,
    into_internal: bool,
) -> proc_macro2::TokenStream {
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
                    if let Some(inner_type) =
                        extract_inner_type_ident(type_path)
                    {
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
                } else {
                    quote! { into() }
                }
            } else {
                quote! { into() }
            }
        }
        _ => {
            quote! { into() }
        }
    }
}

fn extract_inner_type_ident(type_path: &TypePath) -> Option<&Ident> {
    if let Some(segment) = type_path.path.segments.last() {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(syn::GenericArgument::Type(Type::Path(
                inner_type_path,
            ))) = args.args.first()
            {
                if let Some(inner_segment) =
                    inner_type_path.path.segments.last()
                {
                    return Some(&inner_segment.ident);
                }
            }
        }
    }
    None
}
