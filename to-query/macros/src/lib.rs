use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, Expr, ExprLit};

#[proc_macro_derive(ToQuery, attributes(query_rename))]
pub fn to_query(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let fields_code = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .into_iter()
                .map(|field| {
                    let field_ident = field.ident.as_ref().expect("Field ident should be present");
                    let mut key_name = field_ident.to_string();

                    for attr in &field.attrs {
                        if attr.path().is_ident("query_rename") {
                            if let Meta::NameValue(nv) = &attr.meta {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &nv.value {
                                    key_name = lit_str.value();
                                }
                            }
                        }
                    }

                    let ty = &field.ty;
                    let is_option = if let syn::Type::Path(type_path) = ty {
                        type_path.path.segments.last().map(|s| s.ident == "Option").unwrap_or(false)
                    } else {
                        false
                    };

                    if is_option {
                        quote! {
                            if let Some(value) = &self.#field_ident {
                                parts.push(format!("{}={}", #key_name, value));
                            }
                        }
                    } else {
                        quote! {
                            parts.push(format!("{}={}", #key_name, self.#field_ident));
                        }
                    }
                })
                .collect::<Vec<_>>(),
            _ => panic!("ToQuery can only be derived for named structs"),
        },
        _ => panic!("ToQuery can only be derived for structs"),
    };

    let expanded = quote! {
        impl traits::ToQuery for #name {
            fn to_query(&self) -> String {
                let mut parts: Vec<String> = Vec::new();
                #(#fields_code)*
                parts.join("&")
            }
        }
    };

    TokenStream::from(expanded)
}
