use crate::common::conversion_call;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, PathArguments, Type, TypePath};
use proc_macro2::Ident;

pub fn handle_struct(name: &syn::Ident, data: syn::DataStruct) -> proc_macro2::TokenStream {
    let internal_name = quote::format_ident!("Internal{}", name);
    let test_mod_name = Ident::new(&format!("{}_coversion_tests", name.to_string().to_lowercase()), name.span());
    let common = quote! {
        impl #name {
            pub fn into_internal(&self) -> #internal_name {
                self.clone().into()
            }
        }

        #[cfg(test)]
        mod #test_mod_name {
            use super::*;

                #[test]
                fn test_conversion_roundtrip() {
                    let internal = #internal_name::sample();
                    let value = #name::from(internal.clone());
                    let roundrip_converted: #internal_name = value.into_internal();

                    assert_eq!(internal, roundrip_converted);
                }
        }
    };
    

    match data.fields {
        // Named fields: e.g., struct Foo { x: i32, y: String }
        Fields::Named(ref fields_named) => {
            let field_from_internal_conversions: Vec<_> = generate_struct_field_conversions(fields_named);
            let field_into_internal_conversions: Vec<_> = generate_struct_internal_field_conversions(fields_named);

            quote! {
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

                #common
            }
        },
        Fields::Unnamed(ref field_unnamed) => {
            let field_from_internal_conversions: Vec<_> = generate_struct_unnamed_field_conversions(field_unnamed);
            let field_into_internal_conversions: Vec<_> = generate_struct_unnamed_field_internal_conversions(field_unnamed);
            quote! {
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

                #common
            }
        },
        Fields::Unit => {
            quote! {
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

                #common
            }
        },
    }

}

fn generate_struct_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_conversion = conversion_call(&f, false);
        quote! {
            #field_name: value.#field_name.#field_conversion
        }
    }).collect()
}

fn generate_struct_internal_field_conversions(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_conversion = conversion_call(&f, true);
        quote! {
            #field_name: self.#field_name.#field_conversion
        }
    }).collect()
}

fn generate_struct_unnamed_field_conversions(fields: &syn::FieldsUnnamed) -> Vec<proc_macro2::TokenStream> {
    fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = syn::Index::from(i);
        let field_conversion = conversion_call(&f, false);
        quote! {
            value.#index.#field_conversion
        }
    }).collect()
}

fn generate_struct_unnamed_field_internal_conversions(fields: &syn::FieldsUnnamed) -> Vec<proc_macro2::TokenStream> {
    fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = syn::Index::from(i);
        let field_conversion = conversion_call(&f, true);
        quote! {
            self.#index.#field_conversion
        }
    }).collect()
}