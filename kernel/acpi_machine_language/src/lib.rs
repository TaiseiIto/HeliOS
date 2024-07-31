extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn,
};

#[proc_macro_derive(Symbol)]
pub fn acpi_machine_language(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        attrs,
        vis,
        ident, // Type name
        generics,
        data,
    } = syn::parse(input).unwrap();
    match data {
        syn::Data::Struct(syn::DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => match fields {
            syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token,
                unnamed,
            }) => quote! {
            }.into(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

