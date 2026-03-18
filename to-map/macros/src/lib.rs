use proc_macro::TokenStream;
use syn::{DeriveInput, parse};

#[proc_macro_derive(ToMap)]
pub fn to_map(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();

    eprintln!("{:#?}", ast);

    // printing the AST

    // finding the pairs and extracting it

    // recompiling back again to token stream (rust)
    TokenStream::new()
}
