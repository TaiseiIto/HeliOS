extern crate proc_macro;

use {
    proc_macro2::TokenTree,
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
    let first_reader: proc_macro2::TokenStream = derive_first_reader(&derive_input);
    let iter: proc_macro2::TokenStream = derive_reference_to_symbol_iterator(&derive_input);
    let length: proc_macro2::TokenStream = derive_with_length(&derive_input);
    let matches: proc_macro2::TokenStream = derive_matcher(&derive_input);
    let reader: proc_macro2::TokenStream = derive_reader(&derive_input);
    let string_from_self: proc_macro2::TokenStream = derive_string_from_self(&derive_input);
    quote! {
        #analyzer
        #char_from_self
        #debug
        #first_reader
        #iter
        #length
        #matches
        #reader
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
    defined_object_name: Option<Ident>,
    derive_debug: bool,
    derive_first_reader: bool,
    derive_matches: bool,
    derive_reader: bool,
    derive_string_from_self: bool,
    encoding: Option<Encoding>,
    flags: bool,
    has_field_list: bool,
    matching_elements: usize,
    string: bool,
}

impl From<&DeriveInput> for TypeAttribute {
    fn from(derive_input: &DeriveInput) -> Self {
        let DeriveInput {
            attrs,
            vis: _,
            ident,
            generics: _,
            data,
        } = derive_input;
        let defined_object_name: Option<Ident> = ident
            .to_string()
            .strip_prefix("Def")
            .map(|defined_object_name| format_ident!("{}", defined_object_name));
        let derive_debug: bool = attrs
            .iter()
            .all(|attribute| {
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
                            arguments: _,
                        } = segments
                            .iter()
                            .last()
                            .unwrap();
                        match ident
                            .to_string()
                            .as_str() {
                            "manual" => tokens
                                .clone()
                                .into_iter()
                                .all(|token_tree| match token_tree {
                                    TokenTree::Ident(manual_arg) => {
                                        let manual_arg: String = manual_arg.to_string();
                                        !matches!(manual_arg.as_str(), "debug")
                                    },
                                    _ => true,
                                }),
                            _ => true,
                        }
                    },
                    _ => true,
                }
            });
        let derive_first_reader: bool = attrs
            .iter()
            .all(|attribute| {
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
                            arguments: _,
                        } = segments
                            .iter()
                            .last()
                            .unwrap();
                        match ident
                            .to_string()
                            .as_str() {
                            "manual" => tokens
                                .clone()
                                .into_iter()
                                .all(|token_tree| match token_tree {
                                    TokenTree::Ident(manual_arg) => {
                                        let manual_arg: String = manual_arg.to_string();
                                        !matches!(manual_arg.as_str(), "first_reader")
                                    },
                                    _ => true,
                                }),
                            _ => true,
                        }
                    },
                    _ => true,
                }
            });
        let derive_matches: bool = attrs
            .iter()
            .all(|attribute| {
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
                            arguments: _,
                        } = segments
                            .iter()
                            .last()
                            .unwrap();
                        match ident
                            .to_string()
                            .as_str() {
                            "manual" => tokens
                                .clone()
                                .into_iter()
                                .all(|token_tree| match token_tree {
                                    TokenTree::Ident(manual_arg) => {
                                        let manual_arg: String = manual_arg.to_string();
                                        !matches!(manual_arg.as_str(), "matches")
                                    },
                                    _ => true,
                                }),
                            _ => true,
                        }
                    },
                    _ => true,
                }
            });
        let derive_reader: bool = attrs
            .iter()
            .all(|attribute| {
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
                            arguments: _,
                        } = segments
                            .iter()
                            .last()
                            .unwrap();
                        match ident
                            .to_string()
                            .as_str() {
                            "manual" => tokens
                                .clone()
                                .into_iter()
                                .all(|token_tree| match token_tree {
                                    TokenTree::Ident(manual_arg) => {
                                        let manual_arg: String = manual_arg.to_string();
                                        !matches!(manual_arg.as_str(), "reader")
                                    },
                                    _ => true,
                                }),
                            _ => true,
                        }
                    },
                    _ => true,
                }
            });
        let derive_string_from_self: bool = attrs
            .iter()
            .all(|attribute| {
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
                            arguments: _,
                        } = segments
                            .iter()
                            .last()
                            .unwrap();
                        match ident
                            .to_string()
                            .as_str() {
                            "manual" => tokens
                                .clone()
                                .into_iter()
                                .all(|token_tree| match token_tree {
                                    TokenTree::Ident(manual_arg) => {
                                        let manual_arg: String = manual_arg .to_string();
                                        !matches!(manual_arg.as_str(), "string_from_self")
                                    },
                                    _ => true,
                                }),
                            _ => true,
                        }
                    },
                    _ => true,
                }
            });
        let encoding_value: Option<u8> = attrs
            .iter()
            .find_map(|attribute| {
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
                            }) => lit_int
                                .base10_parse()
                                .ok(),
                            _ => unimplemented!(),
                        },
                        _ => None,
                    },
                    _ => None,
                }
            });
        let encoding_value_max: Option<u8> = attrs
            .iter()
            .find_map(|attribute| {
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
                        "encoding_value_max" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => lit_int
                                .base10_parse()
                                .ok(),
                            _ => unimplemented!(),
                        }
                        _ => None,
                    }
                    _ => None,
                }
            });
        let encoding_value_min: Option<u8> = attrs
            .iter()
            .find_map(|attribute| {
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
                        "encoding_value_min" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => lit_int
                                .base10_parse()
                                .ok(),
                            _ => unimplemented!(),
                        }
                        _ => None,
                    }
                    _ => None,
                }
            });
        let flags: bool = attrs
            .iter()
            .any(|attribute| {
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
                        tokens: _,
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
                        let ident: String = ident.to_string();
                        matches!(ident.as_str(), "bitfield")
                    },
                    _ => false,
                }
            });
        let has_field_list: bool = match data {
            Data::Struct(DataStruct {
                struct_token: _,
                fields: Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }),
                semi_token: _,
            }) => unnamed
                .iter()
                .any(|field| {
                    let Field {
                        attrs: _,
                        vis: _,
                        mutability: _,
                        ident: _,
                        colon_token: _,
                        ty,
                    } = field;
                    ty
                        .to_token_stream()
                        .to_string()
                        .as_str() == "FieldList"
                }),
            _ => false,
        };
        let matching_elements: Option<usize> = attrs
            .iter()
            .find_map(|attribute| {
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
                        "matching_elements" => match value {
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(lit_int),
                            }) => lit_int
                                .base10_parse()
                                .ok(),
                            _ => unimplemented!(),
                        },
                        _ => None,
                    }
                    _ => None,
                }
            });
        let string: bool = attrs
            .iter()
            .any(|attribute| {
                let Attribute {
                    pound_token: _,
                    style: _,
                    bracket_token: _,
                    meta,
                } = attribute;
                match meta {
                    Meta::Path(path) => {
                        let path: String = path
                            .to_token_stream()
                            .to_string();
                        matches!(path.as_str(), "string")
                    },
                    _ => false,
                }
            });
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
            defined_object_name,
            derive_debug,
            derive_first_reader,
            derive_matches,
            derive_reader,
            derive_string_from_self,
            encoding,
            flags,
            has_field_list,
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
            .fold((false, None, false, false), |(debug, delimiter, no_leftover, not_string), (next_debug, next_delimiter, next_no_leftover, next_not_string)| (debug || next_debug, delimiter.or(next_delimiter), no_leftover || next_no_leftover, not_string || next_not_string));
        Self {
            debug,
            delimiter,
            no_leftover,
            not_string,
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
    quote! {
        impl crate::acpi::machine_language::syntax::Analyzer for #ident {
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
        defined_object_name: _,
        derive_debug,
        derive_first_reader: _,
        derive_matches: _,
        derive_reader: _,
        derive_string_from_self: _,
        encoding: _,
        flags,
        has_field_list: _,
        matching_elements: _,
        string,
    } = derive_input.into();
    if !derive_debug || flags {
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
                            .fold((Vec::new(), Vec::new()), |(mut unpack, mut format), (next_unpack, next_format)| {
                                unpack.push(next_unpack);
                                format.push(next_format);
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

fn derive_first_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        defined_object_name,
        derive_debug: _,
        derive_first_reader,
        derive_matches: _,
        derive_reader: _,
        derive_string_from_self: _,
        encoding,
        flags,
        has_field_list,
        matching_elements: _,
        string: _,
    } = derive_input.into();
    if derive_first_reader {
        let first_read: proc_macro2::TokenStream = if flags {
            quote! {
                assert!(Self::matches(aml), "aml = {:02x?}", aml);
                match aml {
                    [symbol, aml @ ..] => {
                        let symbol: u8 = *symbol;
                        let symbol: Self = symbol.into();
                        (symbol, aml)
                    },
                    _ => unreachable!(),
                }
            }
        } else {
            match data {
                Data::Enum(DataEnum {
                    enum_token: _,
                    brace_token: _,
                    variants,
                }) => {
                    let read_patterns: Vec<proc_macro2::TokenStream> = variants
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
                                        let symbol = Self::#ident;
                                        let aml: &[u8] = &aml[symbol.length()..];
                                        (symbol, aml)
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
                                                                    let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::first_read(symbol_aml, root, current.clone());
                                                                    let #field_name: #ty = Box::new(#field_name);
                                                                },
                                                                _ => unimplemented!(),
                                                            },
                                                            _ => unimplemented!(),
                                                        }
                                                        _ => quote! {
                                                            let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::first_read(symbol_aml, root, current.clone());
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
                                            let symbol_aml: &[u8] = aml;
                                            #(#reads)*
                                            let symbol = Self::#ident(#(#field_names), *);
                                            let aml: &[u8] = &aml[symbol.length()..];
                                            (symbol, aml)
                                        }
                                    }
                                },
                                _ => unimplemented!(),
                            }
                        })
                        .collect();
                    quote! {
                        #(#read_patterns) else * else {
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
                        let symbol = Self;
                        let aml: &[u8] = &aml[symbol.length()..];
                        (symbol, aml)
                    },
                    Fields::Unnamed(FieldsUnnamed {
                        paren_token: _,
                        unnamed,
                    }) => {
                        let (read, pack): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = match encoding {
                            Some(_encoding) => {
                                let field_type: proc_macro2::TokenStream = unnamed
                                    .first()
                                    .unwrap()
                                    .to_token_stream();
                                let field_name: Ident = format_ident!("field");
                                let read: proc_macro2::TokenStream = quote! {
                                    let (#field_name, symbol_aml): (#field_type, &[u8]) = match symbol_aml {
                                        [#field_name, symbol_aml @ ..] => {
                                            let #field_name: u8 = *#field_name;
                                            let #field_name: #field_type = #field_name as #field_type;
                                            (#field_name, symbol_aml)
                                        },
                                        _ => unreachable!(),
                                    };
                                };
                                let pack: proc_macro2::TokenStream = quote! {
                                    #field_name
                                };
                                (vec![read], vec![pack])
                            }
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
                                    let read: proc_macro2::TokenStream = match ty {
                                        Type::Array(TypeArray {
                                            bracket_token: _,
                                            elem,
                                            semi_token: _,
                                            len,
                                        }) => {
                                            let mutable_current: proc_macro2::TokenStream = match elem
                                                .to_token_stream()
                                                .to_string()
                                                .as_str() {
                                                "NameString" => if has_field_list {
                                                    quote! {
                                                    }
                                                } else {
                                                    quote! {
                                                        let mut current: crate::acpi::machine_language::semantics::Path = current.clone();
                                                    }
                                                },
                                                _ => quote! {
                                                },
                                            };
                                            let add_node: proc_macro2::TokenStream = match elem
                                                .to_token_stream()
                                                .to_string()
                                                .as_str() {
                                                "NameString" => if has_field_list {
                                                    quote! {
                                                    }
                                                } else {
                                                    let defined_object_name: &Ident = defined_object_name
                                                        .as_ref()
                                                        .unwrap();
                                                    quote! {
                                                        if index == 0 {
                                                            current += (&element).into();
                                                            root.add_node(&current, crate::acpi::machine_language::semantics::Object::#defined_object_name);
                                                        }
                                                    }
                                                },
                                                _ => quote! {
                                                },
                                            };
                                            quote! {
                                                #mutable_current
                                                let (elements, symbol_aml): (alloc::vec::Vec<#elem>, &[u8]) = (0..#len)
                                                    .fold((alloc::vec::Vec::new(), symbol_aml), |(mut elements, symbol_aml), index| {
                                                        let (element, symbol_aml): (#elem, &[u8]) = #elem::first_read(symbol_aml, root, current.clone());
                                                        elements.push(element);
                                                        #add_node
                                                        (elements, symbol_aml)
                                                    });
                                                let #field_name: #ty = elements
                                                    .try_into()
                                                    .unwrap();
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
                                                            let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::first_read(symbol_aml, root, current.clone());
                                                            let #field_name: #ty = Box::new(#field_name);
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                "NameString" => if has_field_list {
                                                    quote! {
                                                        let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::first_read(symbol_aml, root, current.clone());
                                                    }
                                                } else {
                                                    let defined_object_name: &Ident = defined_object_name
                                                        .as_ref()
                                                        .unwrap();
                                                    quote! {
                                                        let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::first_read(symbol_aml, root, current.clone());
                                                        let current: crate::acpi::machine_language::semantics::Path = current + (&#field_name).into();
                                                        root.add_node(&current, crate::acpi::machine_language::semantics::Object::#defined_object_name);
                                                    }
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
                                                            let (#field_name, symbol_aml): (Option<#element_type>, &[u8]) = if #element_type::matches(symbol_aml) {
                                                                let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::first_read(symbol_aml, root, current.clone());
                                                                (Some(#field_name), symbol_aml)
                                                            } else {
                                                                (None, symbol_aml)
                                                            };
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                "PkgLength" => if index + 1 == unnamed.len() {
                                                    quote! {
                                                        let #field_name: #ty = symbol_aml.into();
                                                    }
                                                } else {
                                                    quote! {
                                                        let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::first_read(symbol_aml, root, current.clone());
                                                    }
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
                                                                let mut symbol_aml: &[u8] = symbol_aml;
                                                                let mut #field_name: Vec<#element_type> = Vec::new();
                                                                while if symbol_aml.is_empty() {
                                                                    false
                                                                } else {
                                                                    #element_type::matches(symbol_aml)
                                                                } {
                                                                    let (element, remaining_aml): (#element_type, &[u8]) = #element_type::first_read(symbol_aml, root, current.clone());
                                                                    #debug
                                                                    symbol_aml = remaining_aml;
                                                                    #field_name.push(element);
                                                                }
                                                            }
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                _ => quote! {
                                                    let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::first_read(symbol_aml, root, current.clone());
                                                },
                                            }
                                        },
                                        _ => unimplemented!(),
                                    };
                                    let read: proc_macro2::TokenStream = if no_leftover {
                                        quote! {
                                            #read
                                            assert!(symbol_aml.is_empty(), "symbol_aml = {:02x?}", symbol_aml);
                                        }
                                    } else {
                                        read
                                    };
                                    let pack: proc_macro2::TokenStream = quote! {
                                        #field_name
                                    };
                                    (read, pack)
                                })
                                .fold((Vec::new(), Vec::new()), |(mut read, mut pack), (next_read, next_pack)| {
                                    read.push(next_read);
                                    pack.push(next_pack);
                                    (read, pack)
                                }),
                        };
                        quote! {
                            assert!(Self::matches(aml), "aml = {:#x?}", aml);
                            let symbol_aml: &[u8] = aml;
                            #(#read)*
                            let symbol = Self(#(#pack),*);
                            let aml: &[u8] = &aml[symbol.length()..];
                            (symbol, aml)
                        }
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        };
        quote! {
            impl crate::acpi::machine_language::syntax::FirstReader for #ident {
                fn first_read<'a>(aml: &'a [u8], root: &mut semantics::Node, current: semantics::Path) -> (Self, &'a [u8]) {
                    crate::com2_println!("Read {:02x?} as {}", &aml[0..core::cmp::min(10, aml.len())], stringify!(#ident));
                    #first_read
                }
            }
        }
    } else {
        quote! {
        }
    }
}

fn derive_reference_to_symbol_iterator(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        defined_object_name: _,
        derive_debug: _,
        derive_first_reader: _,
        derive_matches: _,
        derive_reader: _,
        derive_string_from_self: _,
        encoding,
        flags,
        has_field_list: _,
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
                                                elem: _,
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
                                                    arguments: _,
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
                                    .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (next_field_name, push_next_field)| {
                                        field_names.push(next_field_name);
                                        push_fields.push(push_next_field);
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
                                    elem: _,
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
                                        arguments: _,
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
                        .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (next_field_name, push_next_field)| {
                            field_names.push(next_field_name);
                            push_fields.push(push_next_field);
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
    let push_mut_symbols: proc_macro2::TokenStream = if encoding.is_some() || flags {
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
                                                elem: _,
                                                semi_token: _,
                                                len: _,
                                            }) => quote! {
                                                #field_name
                                                    .as_mut_slice()
                                                    .iter_mut()
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
                                                    arguments: _,
                                                } = segments
                                                    .iter()
                                                    .last()
                                                    .unwrap();
                                                match ident
                                                    .to_string()
                                                    .as_str() {
                                                    "Box" => quote! {
                                                        symbols.push_back(&mut **#field_name);
                                                    },
                                                    "Option" => quote! {
                                                        if let Some(element) = #field_name {
                                                            symbols.push_back(element);
                                                        }
                                                    },
                                                    "Vec" => quote! {
                                                        #field_name
                                                            .iter_mut()
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
                                    .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (next_field_name, push_next_field)| {
                                        field_names.push(next_field_name);
                                        push_fields.push(push_next_field);
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
                                    elem: _,
                                    semi_token: _,
                                    len: _,
                                }) => quote! {
                                    #field_name
                                        .as_mut_slice()
                                        .iter_mut()
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
                                        arguments: _,
                                    } = segments
                                        .iter()
                                        .last()
                                        .unwrap();
                                    match ident
                                        .to_string()
                                        .as_str() {
                                        "Box" => quote! {
                                            symbols.push_back(&mut **#field_name);
                                        },
                                        "Option" => quote! {
                                            if let Some(element) = #field_name {
                                                symbols.push_back(element);
                                            }
                                        },
                                        "Vec" => quote! {
                                            #field_name
                                                .iter_mut()
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
                        .fold((Vec::new(), Vec::new()), |(mut field_names, mut push_fields), (next_field_name, push_next_field)| {
                            field_names.push(next_field_name);
                            push_fields.push(push_next_field);
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
        impl crate::acpi::machine_language::syntax::ReferenceToSymbolIterator for #ident {
            fn iter(&self) -> crate::acpi::machine_language::syntax::SymbolIterator<'_> {
                let mut symbols: alloc::collections::vec_deque::VecDeque<&dyn crate::acpi::machine_language::syntax::Analyzer> = alloc::collections::vec_deque::VecDeque::new();
                #push_symbols
                crate::acpi::machine_language::syntax::SymbolIterator {
                    symbols,
                }
            }

            fn iter_mut(&mut self) -> crate::acpi::machine_language::syntax::MutSymbolIterator<'_> {
                let mut symbols: alloc::collections::vec_deque::VecDeque<&mut dyn crate::acpi::machine_language::syntax::Analyzer> = alloc::collections::vec_deque::VecDeque::new();
                #push_mut_symbols
                crate::acpi::machine_language::syntax::MutSymbolIterator {
                    symbols,
                }
            }
        }
    }
}

fn derive_with_length(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        defined_object_name: _,
        derive_debug: _,
        derive_first_reader: _,
        derive_matches: _,
        derive_reader: _,
        derive_string_from_self: _,
        encoding,
        flags,
        has_field_list: _,
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
        impl crate::acpi::machine_language::syntax::WithLength for #ident {
            fn length(&self) -> usize {
                #length
            }
        }
    }
}

fn derive_matcher(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        defined_object_name: _,
        derive_debug: _,
        derive_first_reader: _,
        derive_matches,
        derive_reader: _,
        derive_string_from_self: _,
        encoding,
        flags,
        has_field_list: _,
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
    if derive_matches {
        quote! {
            impl crate::acpi::machine_language::syntax::Matcher for #ident {
                fn matches(aml: &[u8]) -> bool {
                    #matches
                }
            }
        }
    } else {
        quote! {
        }
    }
}

fn derive_reader(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics: _,
        data,
    } = derive_input;
    let TypeAttribute {
        defined_object_name: _,
        derive_debug: _,
        derive_first_reader: _,
        derive_matches: _,
        derive_reader,
        derive_string_from_self: _,
        encoding,
        flags,
        has_field_list: _,
        matching_elements: _,
        string: _,
    } = derive_input.into();
    if derive_reader {
        let read: proc_macro2::TokenStream = if flags {
            quote! {
                assert!(Self::matches(aml), "aml = {:02x?}", aml);
                match aml {
                    [symbol, aml @ ..] => {
                        let symbol: u8 = *symbol;
                        let symbol: Self = symbol.into();
                        (symbol, aml)
                    },
                    _ => unreachable!(),
                }
            }
        } else {
            match data {
                Data::Enum(DataEnum {
                    enum_token: _,
                    brace_token: _,
                    variants,
                }) => {
                    let read_patterns: Vec<proc_macro2::TokenStream> = variants
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
                                        let symbol = Self::#ident;
                                        let aml: &[u8] = &aml[symbol.length()..];
                                        (symbol, aml)
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
                                                                    let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::read(symbol_aml);
                                                                    let #field_name: #ty = Box::new(#field_name);
                                                                },
                                                                _ => unimplemented!(),
                                                            },
                                                            _ => unimplemented!(),
                                                        }
                                                        _ => quote! {
                                                            let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::read(symbol_aml);
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
                                            let symbol_aml: &[u8] = aml;
                                            #(#reads)*
                                            let symbol = Self::#ident(#(#field_names), *);
                                            let aml: &[u8] = &aml[symbol.length()..];
                                            (symbol, aml)
                                        }
                                    }
                                },
                                _ => unimplemented!(),
                            }
                        })
                        .collect();
                    quote! {
                        #(#read_patterns) else * else {
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
                        let symbol = Self;
                        let aml: &[u8] = &aml[symbol.length()..];
                        (symbol, aml)
                    },
                    Fields::Unnamed(FieldsUnnamed {
                        paren_token: _,
                        unnamed,
                    }) => {
                        let (read, pack): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = match encoding {
                            Some(_encoding) => {
                                let field_type: proc_macro2::TokenStream = unnamed
                                    .first()
                                    .unwrap()
                                    .to_token_stream();
                                let field_name: Ident = format_ident!("field");
                                let read: proc_macro2::TokenStream = quote! {
                                    let (#field_name, symbol_aml): (#field_type, &[u8]) = match symbol_aml {
                                        [#field_name, symbol_aml @ ..] => {
                                            let #field_name: u8 = *#field_name;
                                            let #field_name: #field_type = #field_name as #field_type;
                                            (#field_name, symbol_aml)
                                        },
                                        _ => unreachable!(),
                                    };
                                };
                                let pack: proc_macro2::TokenStream = quote! {
                                    #field_name
                                };
                                (vec![read], vec![pack])
                            }
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
                                    let read: proc_macro2::TokenStream = match ty {
                                        Type::Array(TypeArray {
                                            bracket_token: _,
                                            elem,
                                            semi_token: _,
                                            len,
                                        }) => quote! {
                                            let (elements, symbol_aml): (alloc::vec::Vec<#elem>, &[u8]) = (0..#len)
                                                .fold((alloc::vec::Vec::new(), symbol_aml), |(mut elements, symbol_aml), _| {
                                                    let (element, symbol_aml): (#elem, &[u8]) = #elem::read(symbol_aml);
                                                    elements.push(element);
                                                    (elements, symbol_aml)
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
                                                            let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::read(symbol_aml);
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
                                                            let (#field_name, symbol_aml): (Option<#element_type>, &[u8]) = if #element_type::matches(symbol_aml) {
                                                                let (#field_name, symbol_aml): (#element_type, &[u8]) = #element_type::read(symbol_aml);
                                                                (Some(#field_name), symbol_aml)
                                                            } else {
                                                                (None, symbol_aml)
                                                            };
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                "PkgLength" => if index + 1 == unnamed.len() {
                                                    quote! {
                                                        let #field_name: #ty = symbol_aml.into();
                                                    }
                                                } else {
                                                    quote! {
                                                        let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::read(symbol_aml);
                                                    }
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
                                                                let mut symbol_aml: &[u8] = symbol_aml;
                                                                let mut #field_name: Vec<#element_type> = Vec::new();
                                                                while if symbol_aml.is_empty() {
                                                                    false
                                                                } else {
                                                                    #element_type::matches(symbol_aml)
                                                                } {
                                                                    let (element, remaining_aml): (#element_type, &[u8]) = #element_type::read(symbol_aml);
                                                                    #debug
                                                                    symbol_aml = remaining_aml;
                                                                    #field_name.push(element);
                                                                }
                                                            }
                                                        },
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                _ => quote! {
                                                    let (#field_name, symbol_aml): (#ty, &[u8]) = #ty::read(symbol_aml);
                                                },
                                            }
                                        },
                                        _ => unimplemented!(),
                                    };
                                    let read: proc_macro2::TokenStream = if no_leftover {
                                        quote! {
                                            #read
                                            assert!(symbol_aml.is_empty(), "symbol_aml = {:02x?}", symbol_aml);
                                        }
                                    } else {
                                        read
                                    };
                                    let pack: proc_macro2::TokenStream = quote! {
                                        #field_name
                                    };
                                    (read, pack)
                                })
                                .fold((Vec::new(), Vec::new()), |(mut read, mut pack), (next_read, next_pack)| {
                                    read.push(next_read);
                                    pack.push(next_pack);
                                    (read, pack)
                                }),
                        };
                        quote! {
                            assert!(Self::matches(aml), "aml = {:#x?}", aml);
                            let symbol_aml: &[u8] = aml;
                            #(#read)*
                            let symbol = Self(#(#pack),*);
                            let aml: &[u8] = &aml[symbol.length()..];
                            (symbol, aml)
                        }
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        };
        quote! {
            impl crate::acpi::machine_language::syntax::Reader for #ident {
                fn read(aml: &[u8]) -> (Self, &[u8]) {
                    #read
                }
            }
        }
    } else {
        quote! {
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
        defined_object_name: _,
        derive_debug: _,
        derive_first_reader: _,
        derive_matches: _,
        derive_reader: _,
        derive_string_from_self,
        encoding: _,
        flags: _,
        has_field_list: _,
        matching_elements: _,
        string,
    } = derive_input.into();
    let source_type_name: &Ident = ident;
    if !derive_string_from_self {
        quote! {
        }
    } else if string {
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
                        let string: Self = Self::new() + #(#field_references)+*;
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
                    let string: Self = {
                        #convert
                    };
                    string
                        .chars()
                        .filter(|character| (0x20..=0x7e).contains(&(*character as u8)))
                        .collect()
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

