extern crate proc_macro;

use {
    proc_macro2,
    quote::{
        format_ident,
        quote,
    },
    syn::{
        Data,
        DataStruct,
        DeriveInput,
        Fields,
        FieldsUnnamed,
        Ident,
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
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    let format: proc_macro2::TokenStream = match data {
        Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => {
            let (unpack, format_fields): (proc_macro2::TokenStream, proc_macro2::TokenStream) = match fields {
                Fields::Unnamed(FieldsUnnamed {
                    paren_token,
                    unnamed,
                }) => {
                    let number_of_fields: usize = unnamed.len();
                    let field_names: Vec<Ident> = (0..number_of_fields)
                        .map(|index| format_ident!("field{}", index))
                        .collect();
                    let unpack: proc_macro2::TokenStream = quote! {
                        (#(#field_names),*)
                    };
                    let format_fields: Vec<proc_macro2::TokenStream> = field_names
                        .iter()
                        .map(|field_name| quote!{
                            .field(#field_name)
                        })
                        .collect();
                    let format_fields = quote! {
                        #(#format_fields)*
                    };
                    (unpack, format_fields)
                },
                _ => unimplemented!(),
            };
            let unpack: proc_macro2::TokenStream = quote! {
                let Self #unpack = self;
            };
            let format: proc_macro2::TokenStream = quote! {
                formatter
                    .debug_tuple(stringify!(#ident))
                    #format_fields
                    .finish()
            };
            quote! {
                #unpack
                #format
            }
        },
        _ => unimplemented!(),
    };
    quote! {
        impl core::fmt::Debug for #ident {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #format
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

