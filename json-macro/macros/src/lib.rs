use proc_macro::TokenStream;
use quote::quote;
use syn::{ Data, DeriveInput, Fields, parse };

#[proc_macro_derive(ToJson)]
pub fn json_serialize_data(input: TokenStream) -> TokenStream {
    // generate AST using syn
    let ast: DeriveInput = parse(input).unwrap();

    let name = &ast.ident;

    // for printing AST
    // eprintln!("{}", name);

    let pairs = match &ast.data {
        Data::Struct(data_struct) =>
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let fields_name: Vec<_> = fields.named
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
                _ => { panic!("This macro will only work with name feilds") }
            }
        _ => { panic!("This macro will only work with the structs ") }
    };

    let expanded =
        quote! {
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

#[proc_macro_derive(FromJson)]
pub fn json_deserialize_data(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    let (field_parsing, field_names) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut field_parsing = Vec::new();
                    let mut field_names = Vec::new();

                    for field in &fields.named {
                        let ty = &field.ty;
                        let ident = field.ident.as_ref().unwrap();
                        let key = field.ident.as_ref().unwrap().to_string();

                        field_parsing.push(
                            quote! {
                                let #ident:#ty = map.get(#key)
                                .ok_or("missing field")?
                                .as_str()
                                .parse::<#ty>()
                                .map_err(|_| "parse error")?;
                            }
                        );

                        field_names.push(
                            quote! {
                                #ident
                            }
                        );
                    }

                    (field_parsing, field_names)
                }
                _ => panic!("Only allowed on the named fields"),
            }
        }
        _ => panic!("Only allowed on the structs"),
    };

    let expanded =
        quote! {
        impl JsonDeserialize for #name {
            fn from_json(json: &str) -> Result<Self,String>{
                // first we need to drop the curly braces
                let s = json.trim()
                .trim_start_matches('{')
                .trim_end_matches('}');

                let mut map = std::collections::HashMap::new();

                for pair in s.split(','){
                    // we get "id":20
                    let mut parts = pair.split(':');

                    let key = parts.next()
                    .ok_or("invalid json")?
                    .trim()
                    .trim_matches('"');
                    
                    let value = parts.next()
                    .ok_or("invalid json")?
                    .trim();

                    map.insert(key.to_string(),value.to_string());
                }
                
                // let key = value 
                #(#field_parsing)*
                 Ok(#name{
                    #(#field_names),*
                })

            }
        }
    };

    TokenStream::from(expanded)
}
