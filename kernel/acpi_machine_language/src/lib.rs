extern crate proc_macro;

use {
    quote::{
        ToTokens,
        format_ident,
        quote,
    },
    std::{
        borrow::Borrow,
        ops::RangeInclusive,
    },
    syn::{
        AngleBracketedGenericArguments,
        Attribute,
        Data,
        DataEnum,
        DataStruct,
        DeriveInput,
        Expr,
        ExprLit,
        ExprRange,
        Field,
        Fields,
        FieldsUnnamed,
        GenericArgument,
        Ident,
        Lit,
        Meta,
        MetaNameValue,
        Path,
        PathArguments,
        PathSegment,
        RangeLimits,
        Type,
        TypePath,
        Variant,
        parse,
    },
};

#[proc_macro_derive(Reader, attributes(debug, encoding_value, encoding_value_max, encoding_value_min, matching_elements))]
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

enum Encoding {
    Range(RangeInclusive<u8>),
    Value(u8),
}

impl From<u8> for Encoding {
    fn from(value: u8) -> Self {
        Self::Value(value)
    }
}

impl From<RangeInclusive<u8>> for Encoding {
    fn from(range: RangeInclusive<u8>) -> Self {
        Self::Range(range)
    }
}

struct TypeAttribute {
    encoding: Option<Encoding>,
    matching_elements: usize,
}

impl From<&DeriveInput> for TypeAttribute {
    fn from(derive_input: &DeriveInput) -> Self {
        let DeriveInput {
            attrs,
            vis: _,
            ident: _,
            generics: _,
            data: _,
        } = derive_input;
        let (encoding_value, encoding_value_max, encoding_value_min, matching_elements): (Option<u8>, Option<u8>, Option<u8>, Option<usize>) = attrs
            .iter()
            .map(|attribute| {
                let Attribute {
                    pound_token: _,
                    style: _,
                    bracket_token: _,
                    meta,
                } = attribute;
                match meta {
                    Meta::NameValue(MetaNameValue {
                        path,
                        eq_token: _,
                        value,
                    }) => match path
                        .to_token_stream()
                        .to_string()
                        .as_str() {
                        "encoding_value" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => {
                                let encoding_value: u8 = lit_int
                                    .base10_digits()
                                    .parse()
                                    .unwrap();
                                (Some(encoding_value), None, None, None)
                            },
                            _ => (None, None, None, None),
                        },
                        "encoding_value_max" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => {
                                let encoding_value_max: u8 = lit_int
                                    .base10_digits()
                                    .parse()
                                    .unwrap();
                                (None, Some(encoding_value_max), None, None)
                            },
                            _ => (None, None, None, None),
                        },
                        "encoding_value_min" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => {
                                let encoding_value_min: u8 = lit_int
                                    .base10_digits()
                                    .parse()
                                    .unwrap();
                                (None, None, Some(encoding_value_min), None)
                            },
                            _ => (None, None, None, None),
                        },
                        "matching_elements" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => {
                                let matching_elements: usize = lit_int
                                    .base10_digits()
                                    .parse()
                                    .unwrap();
                                (None, None, None, Some(matching_elements))
                            },
                            _ => (None, None, None, None),
                        },
                        _ => (None, None, None, None),
                    },
                    _ => (None, None, None, None),
                }
            })
            .fold((None, None, None, None), |(encoding_value, encoding_value_max, encoding_value_min, matching_elements), (new_encoding_value, new_encoding_value_max, new_encoding_value_min, new_matching_elements)| (encoding_value.or(new_encoding_value), encoding_value_max.or(new_encoding_value_max), encoding_value_min.or(new_encoding_value_min), matching_elements.or(new_matching_elements)));
        let encoding: Option<Encoding> = match (encoding_value, encoding_value_max, encoding_value_min) {
            (Some(encoding_value), None, None) => Some(encoding_value.into()),
            (None, Some(encoding_value_max), Some(encoding_value_min)) => {
                assert!(encoding_value_min < encoding_value_max);
                Some((encoding_value_min..=encoding_value_max).into())
            },
            _ => None,
        };
        let matching_elements: usize = matching_elements.unwrap_or(1);
        Self {
            encoding,
            matching_elements,
        }
    }
}

struct FieldAttribute {
    debug: bool,
}

impl From<&Field> for FieldAttribute {
    fn from(field: &Field) -> Self {
        let Field {
            attrs,
            vis: _,
            mutability: _,
            ident: _,
            colon_token: _,
            ty: _,
        } = field;
        let debug: bool = attrs
            .iter()
            .any(|attribute| {
                let Attribute {
                    pound_token: _,
                    style: _,
                    bracket_token: _,
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
        Self {
            debug,
        }
    }
}

fn derive_debug(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let format: proc_macro2::TokenStream = match data {
        Data::Enum(DataEnum {
            enum_token: _,
            brace_token: _,
            variants,
        }) => {
            let format_patterns: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs: _,
                        ident,
                        fields,
                        discriminant: _,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token: _,
                            unnamed,
                        }) => {
                            let field_names: Vec<Ident> = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, _field)| format_ident!("field{}", index))
                                .collect();
                            let format_fields: Vec<proc_macro2::TokenStream> = field_names
                                .iter()
                                .map(|field_name| quote! {
                                    field(#field_name)
                                })
                                .collect();
                            quote! {
                                Self::#ident(#(#field_names),*) => debug_tuple.#(#format_fields).*
                            }
                        },
                        _ => unimplemented!(),
                    }
                })
                .collect();
            quote! {
                let mut debug_tuple: core::fmt::DebugTuple = formatter.debug_tuple(stringify!(#ident));
                match self {
                    #(#format_patterns),*
                };
                debug_tuple.finish()
            }
        },
        Data::Struct(DataStruct {
            struct_token: _,
            fields,
            semi_token: _,
        }) => {
            let (unpack, format): (proc_macro2::TokenStream, proc_macro2::TokenStream) = match fields {
                Fields::Unit => {
                    let unpack: proc_macro2::TokenStream = quote! {
                    };
                    let format: proc_macro2::TokenStream = quote! {
                        formatter.write_str(stringify!(#ident))
                    };
                    (unpack, format)
                },
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
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
                        let Self (#(#unpack),*) = self;
                    };
                    let format: proc_macro2::TokenStream = quote! {
                        formatter
                            .debug_tuple(stringify!(#ident))
                            #(#format)*
                            .finish()
                    };
                    (unpack, format)
                },
                _ => unimplemented!(),
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
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        encoding,
        matching_elements,
    } = derive_input.into();
    let convert: proc_macro2::TokenStream = match data {
        Data::Enum(DataEnum {
            enum_token: _,
            brace_token: _,
            variants,
        }) => {
            let convert_patterns: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs: _,
                        ident,
                        fields,
                        discriminant: _,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token: _,
                            unnamed,
                        }) => {
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident: _,
                                colon_token: _,
                                ty,
                            } = unnamed
                                .first()
                                .unwrap();
                            let matches: proc_macro2::TokenStream = quote! {
                                #ty::matches(aml)
                            };
                            let (field_names, reads): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, field)| {
                                    let field_name: Ident = format_ident!("field{}", index);
                                    let Field {
                                        attrs: _,
                                        vis: _,
                                        mutability: _,
                                        ident: _,
                                        colon_token: _,
                                        ty,
                                    } = field;
                                    let read: proc_macro2::TokenStream = quote! {
                                        let (#field_name, aml): (#ty, &[u8]) = #ty::read(aml);
                                    };
                                    (field_name, read)
                                })
                                .fold((Vec::new(), Vec::new()), |(mut field_names, mut reads), (field_name, read)| {
                                    field_names.push(field_name);
                                    reads.push(read);
                                    (field_names, reads)
                                });
                            quote! {
                                if #matches {
                                    #(#reads)*
                                    Self::#ident(#(#field_names),*)
                                }
                            }
                        },
                        _ => unimplemented!(),
                    }
                })
                .collect();
            quote! {
                #(#convert_patterns) else * else {
                    panic!("aml = {:#x?}", aml)
                }
            }
        },
        Data::Struct(DataStruct {
            struct_token: _,
            fields,
            semi_token: _,
        }) => match fields {
            Fields::Unit => quote! {
                Self
            },
            Fields::Unnamed(FieldsUnnamed {
                paren_token: _,
                unnamed,
            }) => {
                let (convert, pack): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = match encoding {
                    Some(encoding) => match encoding {
                        Encoding::Range(range) => {
                            let field_type: proc_macro2::TokenStream = unnamed
                                .first()
                                .unwrap()
                                .to_token_stream();
                            assert_eq!(field_type.to_string().as_str(), "u8");
                            let field_name: Ident = format_ident!("field");
                            let start: u8 = *range.start();
                            let convert: proc_macro2::TokenStream = quote! {
                                let #field_name: #field_type = *aml.first().unwrap();
                                let #field_name: #field_type = #field_name - #start;
                            };
                            let pack: proc_macro2::TokenStream = quote! {
                                #field_name
                            };
                            (vec![convert], vec![pack])
                        },
                        Encoding::Value(_value) => unimplemented!(),
                    },
                    None => unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let field_name: Ident = format_ident!("field{}", index);
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident: _,
                                colon_token: _,
                                ty,
                            } = field;
                            let FieldAttribute {
                                debug,
                            } = field.into();
                            let convert: proc_macro2::TokenStream = match ty {
                                Type::Path(TypePath {
                                    qself: _,
                                    path,
                                }) => {
                                    let Path {
                                        leading_colon: _,
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
                                        "Box" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token: _,
                                                lt_token: _,
                                                args,
                                                gt_token: _,
                                            }) => match args
                                                .first()
                                                .unwrap() {
                                                GenericArgument::Type(element_type) => quote! {
                                                    let (#field_name, aml): (#element_type, &[u8]) = #element_type::read(aml);
                                                    let #field_name: #ty = Box::new(#field_name);
                                                },
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        "Vec" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token: _,
                                                lt_token: _,
                                                args,
                                                gt_token: _,
                                            }) => match args
                                                .first()
                                                .unwrap() {
                                                GenericArgument::Type(element_type) => {
                                                    let continuation_condition: proc_macro2::TokenStream = quote! {
                                                        !aml.is_empty() && #element_type::matches(aml)
                                                    };
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
                                                        while #continuation_condition {
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
                                        _ => quote! {
                                            let (#field_name, aml): (#ty, &[u8]) = #ty::read(aml);
                                        },
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
                        }),
                };
                quote! {
                    assert!(Self::matches(aml), "aml = {:#x?}", aml);
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
                #convert
            }
        }
    }
}

fn derive_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data: _,
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
        attrs: _,
        vis: _,
        ident: _,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        encoding,
        matching_elements,
    } = derive_input.into();
    let length: proc_macro2::TokenStream = match data {
        Data::Enum(DataEnum {
            enum_token: _,
            brace_token: _,
            variants,
        }) => {
            let accumulate: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs: _,
                        ident,
                        fields,
                        discriminant: _,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token: _,
                            unnamed,
                        }) => {
                            let (field_names, field_lengths): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, _field)| {
                                    let field_name: Ident = format_ident!("field{}", index);
                                    let field_length: proc_macro2::TokenStream = quote! {
                                        #field_name.length()
                                    };
                                    (field_name, field_length)
                                })
                                .fold((Vec::new(), Vec::new()), |(mut field_names, mut field_lengths), (field_name, field_length)| {
                                    field_names.push(field_name);
                                    field_lengths.push(field_length);
                                    (field_names, field_lengths)
                                });
                            quote! {
                                Self::#ident(#(#field_names),*) => #(#field_lengths)+*
                            }
                        },
                        _ => unimplemented!(),
                    }
                })
                .collect();
            quote! {
                match self {
                    #(#accumulate),*
                }
            }
        },
        Data::Struct(DataStruct {
            struct_token: _,
            fields,
            semi_token: _,
        }) => match fields {
            Fields::Unit => quote! {
                1
            },
            Fields::Unnamed(FieldsUnnamed {
                paren_token: _,
                unnamed,
            }) => match encoding {
                Some(encoding) => match encoding {
                    Encoding::Range(range) => quote! {
                        1
                    },
                    Encoding::Value(value) => unimplemented!(),
                },
                None => {
                    let (unpacks, field_lengths): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let field_name: Ident = format_ident!("field{}", index);
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident: _,
                                colon_token: _,
                                ty,
                            } = field;
                            let unpack: proc_macro2::TokenStream = quote! {
                                #field_name
                            };
                            let field_length: proc_macro2::TokenStream = match ty {
                                Type::Path(TypePath {
                                    qself: _,
                                    path,
                                }) => {
                                    let Path {
                                        leading_colon: _,
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
                                        "Box" => quote! {
                                            #field_name.length()
                                        },
                                        "Vec" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token: _,
                                                lt_token: _,
                                                args: _,
                                                gt_token: _,
                                            }) => quote! {
                                                #field_name
                                                    .iter()
                                                    .map(|element| element.length())
                                                    .sum::<usize>()
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => quote! {
                                            #field_name.length()
                                        },
                                    }
                                },
                                _ => unimplemented!(),
                            };
                            (unpack, field_length)
                        })
                        .fold((Vec::new(), Vec::new()), |(mut unpacks, mut field_lengths), (unpack, field_length)| {
                            unpacks.push(unpack);
                            field_lengths.push(field_length);
                            (unpacks, field_lengths)
                        });
                    quote! {
                        let Self (#(#unpacks),*) = self;
                        #(#field_lengths)+*
                    }
                },
            },
            _ => unimplemented!(),
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
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: _,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        encoding,
        matching_elements,
    } = derive_input.into();
    let matches: proc_macro2::TokenStream = match data {
        Data::Enum(DataEnum {
            enum_token: _,
            brace_token: _,
            variants,
        }) => {
            let matches: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs: _,
                        ident: _,
                        fields,
                        discriminant: _,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token: _,
                            unnamed,
                        }) => {
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident: _,
                                colon_token: _,
                                ty,
                            } = unnamed
                                .first()
                                .unwrap();
                            quote! {
                                #ty::matches(aml)
                            }
                        },
                        _ => unimplemented!(),
                    }
                })
                .collect();
            quote! {
                #(#matches) || *
            }
        },
        Data::Struct(DataStruct {
            struct_token: _,
            fields,
            semi_token: _,
        }) => match fields {
            Fields::Unit => match encoding.unwrap() {
                Encoding::Value(value) => quote! {
                    aml
                        .first()
                        .is_some_and(|head| *head == #value)
                },
                _ => unimplemented!(),
            },
            Fields::Unnamed(FieldsUnnamed {
                paren_token: _,
                unnamed,
            }) => match encoding {
                Some(encoding) => match encoding {
                    Encoding::Range(range) => {
                        let start: u8 = *range.start();
                        let end: u8 = *range.end();
                        quote! {
                            aml
                                .first()
                                .is_some_and(|head| (#start..=#end).contains(head))
                        }
                    },
                    _ => unimplemented!(),
                },
                None => {
                    let mut matches: proc_macro2::TokenStream = quote! {
                        true
                    };
                    unnamed
                        .iter()
                        .take(matching_elements)
                        .rev()
                        .for_each(|field| {
                            let Field {
                                attrs,
                                vis,
                                mutability,
                                ident,
                                colon_token,
                                ty,
                            } = field;
                            match ty {
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
                                        "Box" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token,
                                                lt_token,
                                                args,
                                                gt_token,
                                            }) => match args
                                                .first()
                                                .unwrap() {
                                                GenericArgument::Type(element_type) => {
                                                    matches = quote! {
                                                        if #element_type::matches(aml) {
                                                            let (_, aml): (#element_type, &[u8]) = #element_type::read(aml);
                                                            #matches
                                                        } else {
                                                            false
                                                        }
                                                    };
                                                },
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        "Vec" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token,
                                                lt_token,
                                                args,
                                                gt_token,
                                            }) => match args
                                                .first()
                                                .unwrap() {
                                                GenericArgument::Type(element_type) => {
                                                    assert_eq!(matching_elements, 1);
                                                    matches = quote! {
                                                        if aml.is_empty() {
                                                            true
                                                        } else {
                                                            #element_type::matches(aml)
                                                        }
                                                    };
                                                },
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => {
                                            matches = quote! {
                                                if #ty::matches(aml) {
                                                    let (_, aml): (#ty, &[u8]) = #ty::read(aml);
                                                    #matches
                                                } else {
                                                    false
                                                }
                                            };
                                        },
                                    }
                                },
                                _ => unimplemented!(),
                            };
                        });
                    matches
                },
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    quote! {
        fn matches(aml: &[u8]) -> bool {
            #matches
        }
    }
}

