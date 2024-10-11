extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// #[proc_macro_derive(EnumConversion)]
// pub fn conversion_derive(input: TokenStream) -> TokenStream {
// //     // Parse the input tokens into a syntax tree
// //     let input = parse_macro_input!(input as DeriveInput);

// //     // Get the enum name (the external enum)
// //     let external_enum_name = input.ident;

// //     // Construct the corresponding "Internal" enum name
// //     let internal_enum_name = quote::format_ident!("Internal{}", external_enum_name);

// //     // Match to make sure we are dealing with an enum
// //     let data = match input.data {
// //         Data::Enum(data) => data,
// //         _ => panic!("EnumConversion can only be applied to enums"),
// //     };

// //     // Collect variants
// //     let variants: Vec<_> = data.variants.iter().collect();

// //     // Generate matching patterns for From and Into
// //     let from_matches = variants.iter().map(|v| {
// //         let variant_name = &v.ident;
// //         match &v.fields {
// //             Fields::Unit => quote! {
// //                 #internal_enum_name::#variant_name => #external_enum_name::#variant_name,
// //             },
// //             _ => panic!("Enum variants with fields are not supported"),
// //         }
// //     });

// //     let into_matches = variants.iter().map(|v| {
// //         let variant_name = &v.ident;
// //         match &v.fields {
// //             Fields::Unit => quote! {
// //                 #external_enum_name::#variant_name => #internal_enum_name::#variant_name,
// //             },
// //             _ => panic!("Enum variants with fields are not supported"),
// //         }
// //     });

//     // Generate the final output for both From and Into implementations
//     let expanded = quote! {
//         // impl From<#internal_enum_name> for #external_enum_name {
//         //     fn from(value: #internal_enum_name) -> Self {
//         //         match value {
//         //             #(#from_matches)*
//         //         }
//         //     }
//         // }

//         // impl Into<#internal_enum_name> for #external_enum_name {
//         //     fn into(self) -> #internal_enum_name {
//         //         match self {
//         //             #(#into_matches)*
//         //         }
//         //     }
//         // }
//     };

//     // Convert expanded code into TokenStream and return it
//     TokenStream::from(expanded)
// }

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
