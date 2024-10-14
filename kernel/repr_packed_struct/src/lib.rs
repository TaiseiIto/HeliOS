extern crate proc_macro;

use {
    proc_macro2::TokenTree,
    quote::quote,
    syn::{
        DeriveInput,
        parse,
    },
};

#[proc_macro_derive(OffsetGetter)]
pub fn derive_offset_getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();
    quote! {
    }   .try_into()
        .unwrap()
}

