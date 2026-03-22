use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::{Data, DeriveInput, Fields, parse, token::Token};

#[proc_macro_derive(ToQuery)]
pub fn to_query(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();
    // print AST to view
    eprint!("{:#?}", ast);

    // go through the pairs

    // expand
    TokenStream::new()
}

// what do we actually want ??
//
// let search = Search {
//    term : "rust".into(),
//  page : 2 ,
// per_page : Some(50)
//}

// println!("{}", search.to_query());
// output : term= rust&page=2&per_page = 50
