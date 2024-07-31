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

#[proc_macro_derive(Reader)]
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
        impl From<&[u8]> for TermList {
            fn from(aml: &[u8]) -> Self {
                assert!(Self::matches(aml), "aml = {:#x?}", aml);
                let mut aml: &[u8] = aml;
                let mut term_list: Vec<TermObj> = Vec::new();
                while !aml.is_empty() {
                    let (term_obj, remaining_aml): (TermObj, &[u8]) = TermObj::read(aml);
                    com2_println!("term_obj = {:#x?}", term_obj);
                    aml = remaining_aml;
                    term_list.push(term_obj);
                }
                Self(term_list)
            }
        }
    }
}

fn derive_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    quote! {
        impl Reader<'_> for TermList {
            fn length(&self) -> usize {
                self.0
                    .iter()
                    .map(|term_obj| term_obj.length())
                    .sum::<usize>()
            }
        
            fn matches(aml: &[u8]) -> bool {
                if aml.is_empty() {
                    true
                } else {
                    TermObj::matches(aml)
                }
            }
        }
    }
}

