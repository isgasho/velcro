use proc_macro::TokenStream;
use syn::parse_macro_input;
use velcro_core::{BTreeSetInput, HashSetInput, IterInput, VecInput};

#[proc_macro]
pub fn vec(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn btree_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as BTreeSetInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn hash_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as HashSetInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn iter(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as IterInput).into_output();
    TokenStream::from(output)
}
