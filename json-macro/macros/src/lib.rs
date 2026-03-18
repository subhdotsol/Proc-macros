use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse};

#[proc_macro_derive(ToJson)]
pub fn json_serialize_data(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();

    let name = &ast.ident;

    // for printing AST
    // eprintln!("{}", name);

    let pairs = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let fields_name: Vec<_> = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap())
                    .collect();

                let extracted_pairs: Vec<_> = fields_name
                    .iter()
                    .map(|ident| {
                        let key = ident.to_string();
                        quote! {
                            format!("\"{}\" : {}" , #key , self.#ident)
                            // dont use semi colon
                            // id : 1
                            // we are passing # because we are passing it to quote
                        }
                    })
                    .collect();
                extracted_pairs
            }
            _ => {
                panic!("This macro will only work with name feilds")
            }
        },
        _ => {
            panic!("This macro will only work with the structs ")
        }
    };

    let expanded = quote! {
        // This name should match the trait name
        impl JsonSerialize for #name {
            fn to_json(&self) -> String{
                let body = vec![#(#pairs),*].join(",");
                format!("{{{}}}", body)
            }
        }
    };

    TokenStream::from(expanded)
}

// what we want
// person.to_json();

// {
//     "id": 1,
//     "age": 20,
//     "name": "neha"
// }
