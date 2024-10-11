extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, PathArguments};
use proc_macro2::Ident;

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
    let test_mod_name = Ident::new(&format!("{}_tests", name.to_string().to_lowercase()), name.span());

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
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions: Vec<_> = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    match &f.ty {
                        Type::Path(type_path) => {
                            // Check if it's a Vec<T> by looking at the type
                            if let Some(segment) = type_path.path.segments.last() {
                                if segment.ident == "Vec" {
                                    // Call into_vec() for Vec types
                                    return quote! {
                                        #field_name: #field_name.into_vec()
                                    };
                                } else if segment.ident == "HashMap" {
                                    // Call into_hash_map() for HashMap types
                                    return quote! {
                                        #field_name: #field_name.into_hash_map()
                                    };
                                }
                            }
                        }
                        _ => {}
                    }
                    // Default to calling .into() for non-Vec types
                    quote! {
                        #field_name: #field_name.into()
                    }
                }).collect::<Vec<_>>();

                quote! {
                    #internal_name::#variant_name { #( #field_names ),* } => Self::#variant_name { #( #field_conversions ),* }
                }
            }
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
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_conversions: Vec<_> = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    match &f.ty {
                        Type::Path(type_path) => {
                            // Check if it's a Vec<T> by looking at the type
                            if let Some(segment) = type_path.path.segments.last() {
                                if segment.ident == "Vec" {
                                    // Call into_vec() for Vec types
                                    return quote! {
                                        #field_name: #field_name.into_internal_vec()
                                    };
                                } else if segment.ident == "HashMap" {
                                    // Call into_hash_map() for HashMap types
                                    return quote! {
                                        #field_name: #field_name.into_internal_hash_map()
                                    };
                                }
                            }
                        }
                        _ => {}
                    }
                    // Default to calling .into() for non-Vec types
                    quote! {
                        #field_name: #field_name.into()
                    }
                }).collect::<Vec<_>>();
                quote! {
                    Self::#variant_name { #( #field_names ),* } => #internal_name::#variant_name  { #( #field_conversions ),* }
                }
            },
            _ => panic!("FromInto macro supports only tuple-style or unit-style variants"),
        }
    });

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

    // Generate the implementation of the `into_internal()` function
    let expanded = quote! {
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
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
