#[cfg(test)]
pub mod parameterized_tests {
    use proc_macro::TokenStream;

    #[proc_macro_attribute]
    pub fn test_each(attr: TokenStream, item: TokenStream) -> TokenStream {
        println!("attr: \"{}\"", attr.to_string());
        println!("item: \"{}\"", item.to_string());
        item
    }
}
