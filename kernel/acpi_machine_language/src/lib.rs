extern crate proc_macro;

use {
    proc_macro2,
    quote::{
        format_ident,
        quote,
    },
    syn::{
        AngleBracketed,
        AngleBracketedGenericArguments,
        Data,
        DataStruct,
        DeriveInput,
        Field,
        Fields,
        FieldsUnnamed,
        GenericArgument,
        Ident,
        Path,
        PathSegment,
        Type,
        TypePath,
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
        use crate::acpi::machine_language::Reader;
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
                    let field_names: Vec<Ident> = unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, _field)| format_ident!("field{}", index))
                        .collect();
                    let unpack: proc_macro2::TokenStream = quote! {
                        (#(#field_names),*)
                    };
                    let format_fields: Vec<proc_macro2::TokenStream> = field_names
                        .iter()
                        .map(|field_name| quote! {
                            .field(#field_name)
                        })
                        .collect();
                    let format_fields: proc_macro2::TokenStream = quote! {
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
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    let (convert, pack): (proc_macro2::TokenStream, proc_macro2::TokenStrem) = match data {
        Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => match fields {
            Fields::Unnamed(FieldsUnnamed {
                paren_token,
                unnamed,
            }) => {
                let (convert, pack): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        let field_name: Ident = format_ident!("field{}", index);
                        let Field {
                            attrs,
                            vis,
                            ident,
                            colon_token,
                            ty,
                        } = field;
                        let convert: proc_macro2::TokenStream = match ty {
                            Type::Path(TypePath {
                                qself,
                                path,
                            }) => {
                                let Path {
                                    leading_colon,
                                    segments,
                                } = path;
                                let PathSegment {
                                    ident,
                                    arguments,
                                } = segments
                                    .iter()
                                    .last()
                                    .unwrap();
                                match ident.to_string().as_str() {
                                    "Vec" => match argument {
                                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                            colon2_token,
                                            lt_token,
                                            args,
                                            gt_token,
                                        }) => match args.first().unwrap() {
                                            GenericArgument::Type(element_type) => quote! {
                                                let mut aml: &[u8] = aml;
                                                let mut #field_name: Vec<#element_type> = Vec::new();
                                                while !aml.is_empty() {
                                                    let (element, remaining_aml): (#element_type, &[u8]) = #element_type::read(aml);
                                                    crate::com2_println!("element = {:#x?}", element);
                                                    aml = remaining_aml;
                                                    #field_name.push(element);
                                                }
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                }
                            },
                            _ => unimplemented!(),
                        };
                        let pack: proc_macro2::TokenStream = quote! {
                            #field_name
                        };
                        (convert, pack)
                    })
                    .fold((Vec::new(), Vec::new()), |(mut convert, mut pack), (new_convert, new_pack)| {
                        convert.push(new_convert);
                        pack.push(new_pack);
                        (convert, pack)
                    });
                let convert: proc_macro2::TokenStream = quote! {
                    #(#convert)*
                };
                let pack: proc_macro2::TokenStream = quote! {
                    (#(#pack),*)
                };
                (convert, pack)
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    quote! {
        impl From<&[u8]> for #ident {
            fn from(aml: &[u8]) -> Self {
                assert!(Self::matches(aml), "aml = {:#x?}", aml);
                #convert
                Self #pack
            }
        }
    }
}

fn derive_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    quote! {
        impl Reader<'_> for #ident {
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

