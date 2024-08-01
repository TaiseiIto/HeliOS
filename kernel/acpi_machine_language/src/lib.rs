extern crate proc_macro;

use {
    proc_macro2,
    quote::{
        ToTokens,
        format_ident,
        quote,
    },
    syn::{
        AngleBracketedGenericArguments,
        Attribute,
        Data,
        DataStruct,
        DeriveInput,
        Field,
        Fields,
        FieldsUnnamed,
        GenericArgument,
        Ident,
        Meta,
        Path,
        PathArguments,
        PathSegment,
        Type,
        TypePath,
        parse,
    },
};

#[proc_macro_derive(Symbol, attributes(debug))]
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
            let (unpack, format): (proc_macro2::TokenStream, proc_macro2::TokenStream) = match fields {
                Fields::Unnamed(FieldsUnnamed {
                    paren_token,
                    unnamed,
                }) => {
                    let (unpack, format): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, _field)| {
                            let field_name: Ident = format_ident!("field{}", index);
                            let unpack: proc_macro2::TokenStream = quote! {
                                #field_name
                            };
                            let format: proc_macro2::TokenStream = quote! {
                                .field(#field_name)
                            };
                            (unpack, format)
                        })
                        .fold((Vec::new(), Vec::new()), |(mut unpack, mut format), (new_unpack, new_format)| {
                            unpack.push(new_unpack);
                            format.push(new_format);
                            (unpack, format)
                        });
                    let unpack: proc_macro2::TokenStream = quote! {
                        (#(#unpack),*)
                    };
                    let format: proc_macro2::TokenStream = quote! {
                        #(#format)*
                    };
                    (unpack, format)
                },
                _ => unimplemented!(),
            };
            quote! {
                let Self #unpack = self;
                formatter
                    .debug_tuple(stringify!(#ident))
                    #format
                    .finish()
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
    let convert: proc_macro2::TokenStream = match data {
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
                            mutability,
                        } = field;
                        let debug: bool = attrs
                            .iter()
                            .any(|attribute| {
                                let Attribute {
                                    pound_token,
                                    style,
                                    bracket_token,
                                    meta,
                                } = attribute;
                                match meta {
                                    Meta::Path(path) => matches!(path
                                        .to_token_stream()
                                        .to_string()
                                        .as_str(), "debug"),
                                    _ => false,
                                }
                            });
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
                                match ident
                                    .to_string()
                                    .as_str() {
                                    "Vec" => match arguments {
                                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                            colon2_token,
                                            lt_token,
                                            args,
                                            gt_token,
                                        }) => match args.first().unwrap() {
                                            GenericArgument::Type(element_type) => {
                                                let debug: proc_macro2::TokenStream = if debug {
                                                    quote! {
                                                        crate::com2_println!("element = {:#x?}", element);
                                                    }
                                                } else {
                                                    quote! {
                                                    }
                                                };
                                                quote! {
                                                    let mut aml: &[u8] = aml;
                                                    let mut #field_name: Vec<#element_type> = Vec::new();
                                                    while !aml.is_empty() {
                                                        let (element, remaining_aml): (#element_type, &[u8]) = #element_type::read(aml);
                                                        #debug
                                                        aml = remaining_aml;
                                                        #field_name.push(element);
                                                    }
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
                quote! {
                    #(#convert)*
                    Self(#(#pack),*)
                }
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    quote! {
        impl From<&[u8]> for #ident {
            fn from(aml: &[u8]) -> Self {
                assert!(Self::matches(aml), "aml = {:#x?}", aml);
                #convert
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
    let length: proc_macro2::TokenStream = derive_length(derive_input);
    let matches: proc_macro2::TokenStream = derive_matches(derive_input);
    quote! {
        impl Reader<'_> for #ident {
            #length
            #matches
        }
    }
}

fn derive_length(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    let length: proc_macro2::TokenStream = match data {
        Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => {
            let unpack: proc_macro2::TokenStream = quote! {
                let Self(field0) = self;
            };
            let accumulate: proc_macro2::TokenStream = quote! {
                field0
                    .iter()
                    .map(|element| element.length())
                    .sum::<usize>()
            };
            quote! {
                #unpack
                #accumulate
            }
        },
        _ => unimplemented!(),
    };
    quote! {
        fn length(&self) -> usize {
            #length
        }
    }
}

fn derive_matches(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    quote! {
        fn matches(aml: &[u8]) -> bool {
            if aml.is_empty() {
                true
            } else {
                TermObj::matches(aml)
            }
        }
    }
}

