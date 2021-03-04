use common::Migration;
use fs::read_dir;
use proc_macro::TokenStream;
use quote::quote;
use std::{convert::TryInto, env, fs, path::Path};
use syn::LitStr;

#[proc_macro]
pub fn migrate(input: TokenStream) -> TokenStream {
    let dir = syn::parse_macro_input!(input as LitStr);
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(&dir.value());

    parse_dir(&path.to_str().unwrap()).into()
}

fn parse_dir(path: &str) -> proc_macro2::TokenStream {
    let migrations: Vec<Migration> = read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().try_into().unwrap())
        .collect();

    quote! {
        vec![ #(#migrations),* ]
    }
}
