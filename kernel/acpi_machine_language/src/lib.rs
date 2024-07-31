extern crate proc_macro;

use {
    proc_macro2,
    quote::quote,
    syn::{
        DeriveInput,
        parse,
    },
};

#[proc_macro_derive(Symbol)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();
    let debug: proc_macro2::TokenStream = derive_debug(&derive_input);
    let from_slice_u8: proc_macro2::TokenStream = derive_from_slice_u8(&derive_input);
    let reader: proc_macro2::TokenStream = derive_reader(&derive_input);
    quote! {
        #debug
        #from_slice_u8
        #reader
    }   .try_into()
        .unwrap()
}

fn derive_debug(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let type_name: proc_macro2::TokenStream = derive_input
        .ident
        .to_string()
        .parse()
        .unwrap();
    quote! {
        impl core::fmt::Debug for #type_name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter
                    .debug_tuple(stringify!(#type_name))
                    .field(&self.0)
                    .finish()
            }
        }
    }
}

fn derive_from_slice_u8(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    quote! {
    }
}

fn derive_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    quote! {
    }
}

