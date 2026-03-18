use proc_macro::TokenStream;
use syn::{DeriveInput, parse};

#[proc_macro_derive(ToJson)]
pub fn json_serialize_data(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();
    eprintln!("{:#?}", ast);

    TokenStream::new()
}
