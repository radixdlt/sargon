use crate::common::*;
use proc_macro2::Ident;
use quote::quote;
use syn::Fields;

pub fn handle_enum(
    name: &syn::Ident,
    data: syn::DataEnum,
) -> proc_macro2::TokenStream {
    let internal_name = quote::format_ident!("Internal{}", name);
    let test_mod_name = Ident::new(
        &format!("{}_tests", name.to_string().to_lowercase()),
        name.span(),
    );

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

fn generate_field_conversions(
    fields: &syn::FieldsNamed,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_conversion = conversion_call(&f, false);
            quote! {
                #field_name: #field_name.#field_conversion
            }
        })
        .collect()
}

fn generate_internal_field_conversions(
    fields: &syn::FieldsNamed,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_conversion = conversion_call(&f, true);
            quote! {
                #field_name: #field_name.#field_conversion
            }
        })
        .collect()
}
