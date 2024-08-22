extern crate proc_macro;

use {
    quote::{
        ToTokens,
        format_ident,
        quote,
    },
    std::{
        collections::BTreeSet,
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
        Field,
        Fields,
        FieldsUnnamed,
        GenericArgument,
        Ident,
        Lit,
        LitStr,
        Meta,
        MetaList,
        MetaNameValue,
        ParenthesizedGenericArguments,
        Path,
        PathArguments,
        PathSegment,
        Type,
        TypeArray,
        TypePath,
        Variant,
        parse,
    },
};

#[proc_macro_derive(Analyzer, attributes(
    debug,
    delimiter,
    encoding_value,
    encoding_value_max,
    encoding_value_min,
    flags,
    manual,
    matching_elements,
    matching_type,
    no_leftover,
    not_string,
    string,
))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();
    let analyzer: proc_macro2::TokenStream = derive_analyzer(&derive_input);
    let char_from_self: proc_macro2::TokenStream = derive_char_from_self(&derive_input);
    let debug: proc_macro2::TokenStream = derive_debug(&derive_input);
    let from_slice_u8: proc_macro2::TokenStream = derive_from_slice_u8(&derive_input);
    let string_from_self: proc_macro2::TokenStream = derive_string_from_self(&derive_input);
    quote! {
        #analyzer
        #char_from_self
        #debug
        #from_slice_u8
        #string_from_self
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
    derive_from_slice_u8: bool,
    derive_matches: bool,
    encoding: Option<Encoding>,
    flags: bool,
    matching_elements: usize,
    string: bool,
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
        let (derive_from_slice_u8, derive_matches, encoding_value, encoding_value_max, encoding_value_min, flags, matching_elements, string): (bool, bool, Option<u8>, Option<u8>, Option<u8>, bool, Option<usize>, bool) = attrs
            .iter()
            .map(|attribute| {
                let Attribute {
                    pound_token: _,
                    style: _,
                    bracket_token: _,
                    meta,
                } = attribute;
                match meta {
                    Meta::List(MetaList {
                        path,
                        delimiter: _,
                        tokens,
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
                        dbg!(ident);
                        dbg!(arguments);
                        dbg!(tokens);
                        (true, true, None, None, None, false, None, false)
                    },
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
                                (true, true, Some(encoding_value), None, None, false, None, false)
                            },
                            _ => unimplemented!(),
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
                                (true, true, None, Some(encoding_value_max), None, false, None, false)
                            },
                            _ => unimplemented!(),
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
                                (true, true, None, None, Some(encoding_value_min), false, None, false)
                            },
                            _ => unimplemented!(),
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
                                (true, true, None, None, None, false, Some(matching_elements), false)
                            },
                            _ => unimplemented!(),
                        },
                        _ => (true, true, None, None, None, false, None, false),
                    },
                    Meta::Path(path) => match path
                        .to_token_stream()
                        .to_string()
                        .as_str() {
                            "flags" => (true, true, None, None, None, true, None, false),
                            "string" => (true, true, None, None, None, false, None, true),
                            _ => unimplemented!(),
                        },
                    _ => (true, true, None, None, None, false, None, false),
                }
            })
            .fold((true, true, None, None, None, false, None, false), |(derive_from_slice_u8, derive_matches, encoding_value, encoding_value_max, encoding_value_min, flags, matching_elements, string), (new_derive_from_slice_u8, new_derive_matches, new_encoding_value, new_encoding_value_max, new_encoding_value_min, new_flags, new_matching_elements, new_string)| (derive_from_slice_u8 && new_derive_from_slice_u8, derive_matches && new_derive_matches, encoding_value.or(new_encoding_value), encoding_value_max.or(new_encoding_value_max), encoding_value_min.or(new_encoding_value_min), flags || new_flags, matching_elements.or(new_matching_elements), string || new_string));
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
            derive_from_slice_u8,
            derive_matches,
            encoding,
            flags,
            matching_elements,
            string,
        }
    }
}

struct VariantAttribute {
    matching_types: BTreeSet<String>,
}

impl From<&Variant> for VariantAttribute {
    fn from(variant: &Variant) -> Self {
        let Variant {
            attrs,
            ident: _,
            fields: _,
            discriminant: _,
        } = variant;
        let matching_types: BTreeSet<String> = attrs
            .iter()
            .filter_map(|attribute| {
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
                        "matching_type" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Str(matching_type),
                            }) => Some(matching_type.value()),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                }
            })
            .collect();
        Self {
            matching_types,
        }
    }
}

struct FieldAttribute {
    debug: bool,
    delimiter: Option<String>,
    no_leftover: bool,
    not_string: bool,
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
        let (debug, delimiter, no_leftover, not_string): (bool, Option<String>, bool, bool) = attrs
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
                        "delimiter" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Str(delimiter),
                            }) => (false, Some(delimiter.value()), false, false),
                            _ => (false, None, false, false),
                        },
                        _ => (false, None, false, false),
                    },
                    Meta::Path(path) => match path
                        .to_token_stream()
                        .to_string()
                        .as_str() {
                        "debug" => (true, None, false, false),
                        "no_leftover" => (false, None, true, false),
                        "not_string" => (false, None, false, true),
                        _ => (false, None, false, false),
                    },
                    _ => (false, None, false, false),
                }
            })
            .fold((false, None, false, false), |(debug, delimiter, no_leftover, not_string), (new_debug, new_delimiter, new_no_leftover, new_not_string)| (debug || new_debug, delimiter.or(new_delimiter), no_leftover || new_no_leftover, not_string || new_not_string));
        Self {
            debug,
            delimiter,
            no_leftover,
            not_string,
        }
    }
}

fn derive_char_from_self(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    match data {
        Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Unnamed(FieldsUnnamed {
                paren_token: _,
                unnamed,
            }),
            semi_token: _,
        }) => match unnamed.first() {
            Some(Field {
                attrs: _,
                vis: _,
                mutability: _,
                ident: _,
                colon_token: _,
                ty,
            }) => match ty
                .to_token_stream()
                .to_string()
                .as_str() {
                "char" => quote! {
                    impl From<&#ident> for char {
                        fn from(source: &#ident) -> Self {
                            let #ident(character) = source;
                            *character
                        }
                    }
                },
                _ => quote! {
                },
            },
            _ => quote! {
            },
        },
        _ => quote! {
        },
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
    let TypeAttribute {
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding: _,
        flags,
        matching_elements: _,
        string,
    } = derive_input.into();
    if flags {
        quote! {
        }
    } else if string {
        quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let string: String = self.into();
                    formatter
                        .debug_tuple(stringify!(#ident))
                        .field(&string)
                        .finish()
                }
            }
        }
    } else {
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
                            Fields::Unit => quote! {
                                Self::#ident => {},
                            },
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
                                    Self::#ident(#(#field_names),*) => {
                                        debug_tuple.#(#format_fields).*;
                                    }
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
                            .map(|(index, field)| {
                                let Field {
                                    attrs: _,
                                    vis: _,
                                    mutability: _,
                                    ident: _,
                                    colon_token: _,
                                    ty,
                                } = field;
                                let field_name: Ident = format_ident!("field{}", index);
                                let unpack: proc_macro2::TokenStream = quote! {
                                    #field_name
                                };
                                let format: proc_macro2::TokenStream = match ty {
                                    Type::Array(_) => quote! {
                                        #field_name
                                            .as_slice()
                                            .iter()
                                            .for_each(|element| {
                                                debug_tuple.field(element);
                                            });
                                    },
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
                                            arguments: _,
                                        } = segments
                                            .iter()
                                            .last()
                                            .unwrap();
                                        match ident
                                            .to_string()
                                            .as_str() {
                                            "Option" => quote! {
                                                if let Some(element) = #field_name {
                                                    debug_tuple.field(element);
                                                }
                                            },
                                            "Vec" => quote! {
                                                #field_name
                                                    .iter()
                                                    .for_each(|element| {
                                                        debug_tuple.field(element);
                                                    });
                                            },
                                            _ => quote! {
                                                debug_tuple.field(#field_name);
                                            },
                                        }
                                    },
                                    _ => unimplemented!(),
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
                            let mut debug_tuple: core::fmt::DebugTuple = formatter.debug_tuple(stringify!(#ident));
                            #(#format)*
                            debug_tuple.finish()
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
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding,
        flags,
        matching_elements: _,
        string: _,
    } = derive_input.into();
    let convert: proc_macro2::TokenStream = if flags {
        quote! {
            assert!(Self::matches(aml), "aml = {:02x?}", aml);
            (*aml.first().unwrap()).into()
        }
    } else {
        match data {
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
                        let VariantAttribute {
                            matching_types,
                        } = variant.into();
                        match fields {
                            Fields::Unit => quote! {
                                if true {
                                    Self::#ident
                                }
                            },
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
                                let matches: proc_macro2::TokenStream = if matching_types.is_empty() {
                                    match ty {
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
                                                            #element_type::matches(aml)
                                                        },
                                                        _ => unimplemented!(),
                                                    }
                                                    _ => unimplemented!(),
                                                },
                                                _ => quote! {
                                                    #ty::matches(aml)
                                                },
                                            }
                                        }
                                        _ => unimplemented!(),
                                    }
                                } else {
                                    let matches: Vec<proc_macro2::TokenStream> = matching_types
                                        .iter()
                                        .map(|matching_type| {
                                            let matching_type: Ident = format_ident!("{}", matching_type);
                                            quote! {
                                                #matching_type::matches(aml)
                                            }
                                        })
                                        .collect();
                                    quote! {
                                        #(#matches) || *
                                    }
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
                                        let read: proc_macro2::TokenStream = match ty {
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
                                                    _ => quote! {
                                                        let (#field_name, aml): (#ty, &[u8]) = #ty::read(aml);
                                                    },
                                                }
                                            },
                                            _ => unimplemented!(),
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
                        Some(_encoding) => {
                            let field_type: proc_macro2::TokenStream = unnamed
                                .first()
                                .unwrap()
                                .to_token_stream();
                            let field_name: Ident = format_ident!("field");
                            let convert_field: proc_macro2::TokenStream = match field_type
                                .to_string()
                                .as_str() {
                                "char" => quote! {
                                    (*aml.first().unwrap()) as char
                                },
                                "u8" => quote! {
                                    *aml.first().unwrap()
                                },
                                _ => unimplemented!(),
                            };
                            let convert: proc_macro2::TokenStream = quote! {
                                let #field_name: #field_type = #convert_field;
                                let #field_name: #field_type = #field_name;
                            };
                            let pack: proc_macro2::TokenStream = quote! {
                                #field_name
                            };
                            (vec![convert], vec![pack])
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
                                    delimiter: _,
                                    no_leftover,
                                    not_string: _,
                                } = field.into();
                                let convert: proc_macro2::TokenStream = match ty {
                                    Type::Array(TypeArray {
                                        bracket_token: _,
                                        elem,
                                        semi_token: _,
                                        len,
                                    }) => quote! {
                                        let (elements, aml): (alloc::vec::Vec<#elem>, &[u8]) = (0..#len)
                                            .fold((alloc::vec::Vec::new(), aml), |(mut elements, aml), _| {
                                                let (element, aml): (#elem, &[u8]) = #elem::read(aml);
                                                elements.push(element);
                                                (elements, aml)
                                            });
                                        let #field_name: #ty = elements
                                            .try_into()
                                            .unwrap();
                                    },
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
                                            "Option" => match arguments {
                                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                    colon2_token: _,
                                                    lt_token: _,
                                                    args,
                                                    gt_token: _,
                                                }) => match args
                                                    .first()
                                                    .unwrap() {
                                                    GenericArgument::Type(element_type) => quote! {
                                                        let (#field_name, aml): (Option<#element_type>, &[u8]) = if #element_type::matches(aml) {
                                                            let (#field_name, aml): (#element_type, &[u8]) = #element_type::read(aml);
                                                            (Some(#field_name), aml)
                                                        } else {
                                                            (None, aml)
                                                        };
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
                                                        let debug: proc_macro2::TokenStream = if debug {
                                                            quote! {
                                                                crate::com2_println!("element = {:#x?}", element);
                                                                crate::com2_println!("remaining_aml = {:02x?}", &remaining_aml[0..core::cmp::min(10, remaining_aml.len())]);
                                                            }
                                                        } else {
                                                            quote! {
                                                            }
                                                        };
                                                        quote! {
                                                            let mut aml: &[u8] = aml;
                                                            let mut #field_name: Vec<#element_type> = Vec::new();
                                                            while if aml.is_empty() {
                                                                false
                                                            } else {
                                                                #element_type::matches(aml)
                                                            } {
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
                                            _ => if index + 1 < unnamed.len() || no_leftover {
                                                quote! {
                                                    let (#field_name, aml): (#ty, &[u8]) = #ty::read(aml);
                                                }
                                            } else {
                                                quote! {
                                                    let #field_name: #ty = aml.into();
                                                }
                                            },
                                        }
                                    },
                                    _ => unimplemented!(),
                                };
                                let convert: proc_macro2::TokenStream = if no_leftover {
                                    quote! {
                                        #convert
                                        assert!(aml.is_empty(), "aml = {:02x?}", aml);
                                    }
                                } else {
                                    convert
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
        }
    };
    quote! {
        impl From<&[u8]> for #ident {
            fn from(aml: &[u8]) -> Self {
                #convert
            }
        }
    }
}

fn derive_analyzer(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data: _,
    } = derive_input;
    let iter: proc_macro2::TokenStream = derive_iter(derive_input);
    let length: proc_macro2::TokenStream = derive_length(derive_input);
    let matches: proc_macro2::TokenStream = derive_matches(derive_input);
    let read: proc_macro2::TokenStream = derive_read();
    quote! {
        impl crate::acpi::machine_language::syntax::Analyzer for #ident {
            #iter
            #length
            #matches
            #read
        }
    }
}

fn derive_iter(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: _,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding,
        flags,
        matching_elements: _,
        string: _,
    } = derive_input.into();
    let push_symbols: proc_macro2::TokenStream = if encoding.is_some() || flags {
        quote! {
        }
    } else {
        match data {
            Data::Enum(DataEnum {
                enum_token: _,
                brace_token: _,
                variants,
            }) => {
                let push_patterns: Vec<proc_macro2::TokenStream> = variants
                    .iter()
                    .map(|variant| {
                        let Variant {
                            attrs: _,
                            ident,
                            fields,
                            discriminant: _,
                        } = variant;
                        match fields {
                            Fields::Unit => quote! {
                                Self::#ident => {},
                            },
                            Fields::Unnamed(FieldsUnnamed {
                                paren_token: _,
                                unnamed,
                            }) => {
                                let (field_names, push_fields): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
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
                                        let push_field: proc_macro2::TokenStream = match ty {
                                            Type::Array(TypeArray {
                                                bracket_token: _,
                                                elem,
                                                semi_token: _,
                                                len: _,
                                            }) => quote! {
                                                #field_name
                                                    .as_slice()
                                                    .iter()
                                                    .for_each(|element| {
                                                        symbols.push_back(element);
                                                    });
                                            },
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
                                                        symbols.push_back(&**#field_name);
                                                    },
                                                    "Option" => quote! {
                                                        if let Some(element) = #field_name {
                                                            symbols.push_back(element);
                                                        }
                                                    },
                                                    "Vec" => quote! {
                                                        #field_name
                                                            .iter()
                                                            .for_each(|element| {
                                                                symbols.push_back(element);
                                                            });
                                                    },
                                                    _ => quote! {
                                                        symbols.push_back(#field_name);
                                                    },
                                                }
                                            },
                                            _ => unimplemented!(),
                                        };
                                        (field_name, push_field)
                                    })
                                    .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (new_field_name, push_new_field)| {
                                        field_names.push(new_field_name);
                                        push_fields.push(push_new_field);
                                        (field_names, push_fields)
                                    });
                                quote! {
                                    Self::#ident(#(#field_names),*) => {
                                        #(#push_fields)*
                                    }
                                }
                            },
                            _ => unimplemented!(),
                        }
                    })
                    .collect();
                quote! {
                    match self {
                        #(#push_patterns),*
                    }
                }
            },
            Data::Struct(DataStruct {
                struct_token: _,
                fields,
                semi_token: _,
            }) => match fields {
                Fields::Unit => quote! {
                },
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }) => {
                    let (field_names, push_fields): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                        .iter()
                        .enumerate()
                        .map(|(index,field)| {
                            let field_name: Ident = format_ident!("field{}", index);
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident: _,
                                colon_token: _,
                                ty,
                            } = field;
                            let push_field: proc_macro2::TokenStream = match ty {
                                Type::Array(TypeArray {
                                    bracket_token: _,
                                    elem,
                                    semi_token: _,
                                    len: _,
                                }) => quote! {
                                    #field_name
                                        .as_slice()
                                        .iter()
                                        .for_each(|element| {
                                            symbols.push_back(element);
                                        });
                                },
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
                                            symbols.push_back(&**#field_name);
                                        },
                                        "Option" => quote! {
                                            if let Some(element) = #field_name {
                                                symbols.push_back(element);
                                            }
                                        },
                                        "Vec" => quote! {
                                            #field_name
                                                .iter()
                                                .for_each(|element| {
                                                    symbols.push_back(element);
                                                });
                                        },
                                        _ => quote! {
                                            symbols.push_back(#field_name);
                                        },
                                    }
                                },
                                _ => unimplemented!(),
                            };
                            (field_name, push_field)
                        })
                        .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (new_field_name, push_new_field)| {
                            field_names.push(new_field_name);
                            push_fields.push(push_new_field);
                            (field_names, push_fields)
                        });
                    quote! {
                        let Self(#(#field_names),*) = self;
                        #(#push_fields)*
                    }
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    };
    quote! {
        fn iter<'a>(&'a self) -> crate::acpi::machine_language::syntax::SymbolIterator<'a> {
            let mut symbols: alloc::collections::vec_deque::VecDeque<&dyn crate::acpi::machine_language::syntax::Analyzer> = alloc::collections::vec_deque::VecDeque::new();
            #push_symbols
            SymbolIterator {
                symbols,
            }
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
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding,
        flags,
        matching_elements: _,
        string: _,
    } = derive_input.into();
    let length: proc_macro2::TokenStream = if flags {
        quote! {
            1
        }
    } else {
        match data {
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
                            Fields::Unit => quote! {
                                Self::#ident => 0
                            },
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
                    Some(_encoding) => quote! {
                        1
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
                                    Type::Array(_) => quote! {
                                        #field_name
                                            .as_slice()
                                            .iter()
                                            .map(|element| element.length())
                                            .sum::<usize>()
                                    },
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
                                            "Option" => quote! {
                                                #field_name
                                                    .as_ref()
                                                    .map_or(0, |element| element.length())
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
                            let Self(#(#unpacks),*) = self;
                            #(#field_lengths)+*
                        }
                    },
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
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
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding,
        flags,
        matching_elements,
        string: _,
    } = derive_input.into();
    let matches: proc_macro2::TokenStream = if flags {
        quote! {
            aml
                .first()
                .is_some()
        }
    } else {
        match data {
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
                        let VariantAttribute {
                            matching_types,
                        } = variant.into();
                        if matching_types.is_empty() {
                            match fields {
                                Fields::Unit => quote! {
                                    true
                                },
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
                                    match ty {
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
                                                            #element_type::matches(aml)
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                _ => quote! {
                                                    #ty::matches(aml)
                                                },
                                            }
                                        },
                                        _ => unimplemented!(),
                                    }
                                },
                                _ => unimplemented!(),
                            }
                        } else {
                            let matches: Vec<proc_macro2::TokenStream> = matching_types
                                .iter()
                                .map(|matching_type| {
                                    let matching_type: Ident = format_ident!("{}", matching_type);
                                    quote! {
                                        #matching_type::matches(aml)
                                    }
                                })
                                .collect();
                            quote! {
                                #(#matches) || *
                            }
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
                    Some(encoding) => {
                        let matches: proc_macro2::TokenStream = match encoding {
                            Encoding::Range(range) => {
                                let start: u8 = *range.start();
                                let end: u8 = *range.end();
                                quote! {
                                    (#start..=#end).contains(head)
                                }
                            },
                            Encoding::Value(value) => quote! {
                                *head == #value
                            },
                        };
                        quote! {
                            aml
                                .first()
                                .is_some_and(|head| #matches)
                        }
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
                                    attrs: _,
                                    vis: _,
                                    mutability: _,
                                    ident: _,
                                    colon_token: _,
                                    ty,
                                } = field;
                                match ty {
                                    Type::Array(TypeArray {
                                        bracket_token: _,
                                        elem,
                                        semi_token: _,
                                        len,
                                    }) => match len {
                                        Expr::Lit(ExprLit {
                                            attrs: _,
                                            lit,
                                        }) => match lit {
                                            Lit::Int(lit_int) => {
                                                let len: usize = lit_int
                                                    .base10_digits()
                                                    .parse()
                                                    .unwrap();
                                                (0..len)
                                                    .for_each(|_| {
                                                        matches = quote! {
                                                            if #elem::matches(aml) {
                                                                let (_, aml): (#elem, &[u8]) = #elem::read(aml);
                                                                #matches
                                                            } else {
                                                                false
                                                            }
                                                        };
                                                    });
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
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
                                                    colon2_token: _,
                                                    lt_token: _,
                                                    args,
                                                    gt_token: _,
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
        }
    };
    quote! {
        fn matches(aml: &[u8]) -> bool {
            #matches
        }
    }
}

fn derive_read() -> proc_macro2::TokenStream {
    quote! {
        fn read(aml: &[u8]) -> (Self, &[u8]) {
            let symbol: Self = aml.into();
            let aml: &[u8] = &aml[symbol.length()..];
            (symbol, aml)
        }
    }
}

fn derive_string_from_self(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        derive_from_slice_u8: _,
        derive_matches: _,
        encoding: _,
        flags: _,
        matching_elements: _,
        string,
    } = derive_input.into();
    let source_type_name: &Ident = ident;
    if string {
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
                        let variant_name: &Ident = ident;
                        match fields {
                            Fields::Unnamed(FieldsUnnamed {
                                paren_token: _,
                                unnamed,
                            }) => {
                                let (field_names, convert_fields): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                                    .iter()
                                    .enumerate()
                                    .map(|(index, _field)| {
                                        let field_name: Ident = format_ident!("field{}", index);
                                        let convert_field: proc_macro2::TokenStream = quote! {
                                            let #field_name: String = #field_name.into();
                                        };
                                        (field_name, convert_field)
                                    })
                                    .fold((Vec::new(), Vec::new()), |(mut field_names, mut convert_fields), (field_name, convert_field)| {
                                        field_names.push(field_name);
                                        convert_fields.push(convert_field);
                                        (field_names, convert_fields)
                                    });
                                let field_references: Vec<proc_macro2::TokenStream> = field_names
                                    .iter()
                                    .map(|field_name| quote! {
                                        &#field_name
                                    })
                                    .collect();
                                quote! {
                                    #source_type_name::#variant_name(#(#field_names),*) => {
                                        #(#convert_fields)*
                                        Self::new() + #(#field_references)+*
                                    }
                                }
                            },
                            _ => unimplemented!(),
                        }
                    })
                    .collect();
                quote! {
                    match source {
                        #(#convert_patterns),*
                    }
                }
            },
            Data::Struct(DataStruct {
                struct_token: _,
                fields,
                semi_token: _,
            }) => match fields {
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }) => {
                    let (field_names, convert_fields, field_references): (Vec<Ident>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = unnamed
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
                                debug: _,
                                delimiter,
                                no_leftover: _,
                                not_string,
                            } = field.into();
                            let delimiter: String = delimiter.unwrap_or_default();
                            let delimiter: LitStr = LitStr::new(&delimiter, proc_macro2::Span::call_site());
                            let convert_field: Option<proc_macro2::TokenStream> = (!not_string).then(|| match ty {
                                Type::Array(_) => {
                                    quote! {
                                        let #field_name: Vec<String> = #field_name
                                            .as_slice()
                                            .iter()
                                            .map(|element| element.into())
                                            .collect();
                                        let #field_name: String = #field_name.join(#delimiter);
                                    }
                                },
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
                                        "Vec" => match arguments {
                                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                                colon2_token: _,
                                                lt_token: _,
                                                args,
                                                gt_token: _,
                                            }) => match args
                                                .first()
                                                .unwrap() {
                                                GenericArgument::Type(_element_type) => quote! {
                                                    let #field_name: Vec<String> = #field_name
                                                        .iter()
                                                        .map(|element| element.into())
                                                        .collect();
                                                    let #field_name: String = #field_name.join(#delimiter);
                                                },
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        _ => quote! {
                                            let #field_name: String = #field_name.into();
                                        },
                                    }
                                },
                                _ => unimplemented!(),
                            });
                            let field_reference: Option<proc_macro2::TokenStream> = (!not_string).then(|| quote! {
                                &#field_name
                            });
                            (field_name, convert_field, field_reference)
                        })
                        .fold((Vec::new(), Vec::new(), Vec::new()), |(mut field_names, mut convert_fields, mut field_references), (field_name, convert_field, field_reference)| {
                            field_names.push(field_name);
                            if let Some(convert_field) = convert_field {
                                convert_fields.push(convert_field);
                            }
                            if let Some(field_reference) = field_reference {
                                field_references.push(field_reference);
                            }
                            (field_names, convert_fields, field_references)
                        });
                    quote! {
                        let #source_type_name(#(#field_names),*) = source;
                        #(#convert_fields)*
                        let string = Self::new() + #(#field_references)+*;
                        let (string, underscores): (Self, Self) = string
                            .chars()
                            .rev()
                            .fold((Self::new(), Self::new()), |(mut string, mut underscores), character| {
                                match character {
                                    '_' => if string.is_empty() {
                                        underscores.push(character);
                                    } else {
                                        string.push(character);
                                    },
                                    character => {
                                        string.push(character);
                                    },
                                };
                                (string, underscores)
                            });
                        if string.is_empty() {
                            underscores
                        } else {
                            string
                                .chars()
                                .rev()
                                .collect()
                        }
                    }
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
        quote! {
            impl From<&#source_type_name> for String {
                fn from(source: &#source_type_name) -> Self {
                    #convert
                }
            }
        }
    } else {
        match data {
            Data::Struct(DataStruct {
                struct_token: _,
                fields: Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }),
                semi_token: _,
            }) => match unnamed.first() {
                Some(Field {
                    attrs: _,
                    vis: _,
                    mutability: _,
                    ident: _,
                    colon_token: _,
                    ty,
                }) => match ty
                    .to_token_stream()
                    .to_string()
                    .as_str() {
                    "char" => quote! {
                        impl From<&#ident> for String {
                            fn from(source: &#ident) -> Self {
                                let character: char = source.into();
                                character.into()
                            }
                        }
                    },
                    _ => quote! {
                    },
                }
                _ => quote! {
                },
            },
            _ => quote! {
            },
        }
    }
}

