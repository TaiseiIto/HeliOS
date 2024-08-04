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
        Meta,
        MetaNameValue,
        Path,
        PathArguments,
        PathSegment,
        Type,
        TypePath,
        Variant,
        parse,
    },
};

#[proc_macro_derive(Reader, attributes(always_matches, encoding_value, debug))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dbg!(&input);
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

struct TypeAttribute {
    always_matches: bool,
    encoding_value: Option<u8>,
}

impl From<&DeriveInput> for TypeAttribute {
    fn from(derive_input: &DeriveInput) -> Self {
        let DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = derive_input;
        let always_matches: bool = attrs
            .iter()
            .find(|attribute| {
                let Attribute {
                    pound_token,
                    style,
                    bracket_token,
                    meta,
                } = attribute;
                match meta {
                    Meta::Path(path) => {
                        matches!(path
                            .to_token_stream()
                            .to_string()
                            .as_str(), "always_matches")
                    },
                    _ => false,
                }
            })
            .is_some();
        let encoding_value: Option<u8> = attrs
            .iter()
            .find_map(|attribute| {
                let Attribute {
                    pound_token,
                    style,
                    bracket_token,
                    meta,
                } = attribute;
                match meta {
                    Meta::NameValue(MetaNameValue {
                        path,
                        eq_token,
                        value,
                    }) => match value {
                        Expr::Lit(ExprLit {
                            attrs,
                            lit,
                        }) => match lit {
                            Lit::Int(lit_int) => Some({
                                let lit_int: u8 = lit_int
                                    .base10_digits()
                                    .parse()
                                    .unwrap();
                                lit_int
                            }),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                }
            });
        dbg!(encoding_value);
        Self {
            always_matches,
            encoding_value,
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
            vis,
            mutability,
            ident,
            colon_token,
            ty,
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
        Self {
            debug,
        }
    }
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
        Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => {
            let format_patterns: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs,
                        ident,
                        fields,
                        discriminant,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token,
                            unnamed,
                        }) => {
                            let field_names: Vec<Ident> = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, field)| format_ident!("field{}", index))
                                .collect();
                            let format_fields: Vec<proc_macro2::TokenStream> = field_names
                                .iter()
                                .map(|field_name| quote! {
                                    .field(#field_name)
                                })
                                .collect();
                            quote! {
                                Self::#ident(#(#field_names),*) => formatter #(#format_fields)*
                            }
                        },
                        _ => unimplemented!(),
                    }
                })
                .collect();
            quote! {
                let mut debug_tuple: core::fmt::DebugTuple = formater.debug_tuple(stringify!(#ident));
                match self {
                    #(#format_patterns),*
                }
                debug_tuple.finish()
            }
        },
        Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
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
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    let convert: proc_macro2::TokenStream = match data {
        Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => {
            let convert_patterns: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs,
                        ident,
                        fields,
                        discriminant,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token,
                            unnamed,
                        }) => {
                            let matches: proc_macro2::TokenStream = {
                                let Field {
                                    attrs,
                                    vis,
                                    mutability,
                                    ident,
                                    colon_token,
                                    ty,
                                } = unnamed
                                    .first()
                                    .unwrap();
                                quote! {
                                    #ty::matches(aml)
                                }
                            };
                            let (field_names, reads): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, field)| {
                                    let field_name: Ident = format_ident!("field{}", index);
                                    let Field {
                                        attrs,
                                        vis,
                                        mutability,
                                        ident,
                                        colon_token,
                                        ty,
                                    } = field;
                                    let read: proc_macro2::TokenStream = quote! {
                                        let (#field_name, aml): (#ty, $[u8]) = #ty::read(aml);
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
            struct_token,
            fields,
            semi_token,
        }) => match fields {
            Fields::Unit => quote! {
                Self
            },
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
                            mutability,
                            ident,
                            colon_token,
                            ty,
                        } = field;
                        let FieldAttribute {
                            debug,
                        } = field.into();
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
        Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => {
            let accumulate: Vec<proc_macro2::TokenStream> = variants
                .iter()
                .map(|variant| {
                    let Variant {
                        attrs,
                        ident,
                        fields,
                        discriminant,
                    } = variant;
                    match fields {
                        Fields::Unnamed(FieldsUnnamed {
                            paren_token,
                            unnamed,
                        }) => {
                            let (field_names, field_lengths): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, field)| {
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
            struct_token,
            fields,
            semi_token,
        }) => match fields {
            Fields::Unit => quote! {
                1
            },
            Fields::Unnamed(FieldsUnnamed {
                paren_token,
                unnamed,
            }) => {
                let (unpack, accumulate): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) = unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        let field_name: Ident = format_ident!("field{}", index);
                        let Field {
                            attrs,
                            vis,
                            mutability,
                            ident,
                            colon_token,
                            ty,
                        } = field;
                        let unpack: proc_macro2::TokenStream = quote! {
                            #field_name
                        };
                        let accumulate: proc_macro2::TokenStream = match ty {
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
                                    "Vec" => quote! {
                                        #field_name
                                            .iter()
                                            .map(|element| element.length())
                                            .sum::<usize>()
                                    },
                                    _ => unimplemented!(),
                                }
                            },
                            _ => unimplemented!(),
                        };
                        (unpack, accumulate)
                    })
                    .fold((Vec::new(), Vec::new()), |(mut unpack, mut accumulate), (new_unpack, new_accumulate)| {
                        unpack.push(new_unpack);
                        accumulate.push(new_accumulate);
                        (unpack, accumulate)
                    });
                quote! {
                    let Self (#(#unpack),*) = self;
                    #(#accumulate)+*
                }
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
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    let TypeAttribute {
        always_matches,
        encoding_value,
    } = derive_input.into();
    let matches: proc_macro2::TokenStream = if always_matches {
        quote! {
            true
        }
    } else {
        match data {
            Data::Struct(DataStruct {
                struct_token,
                fields,
                semi_token,
            }) => match fields {
                Fields::Unit => {
                    let encoding_value: u8 = encoding_value.unwrap();
                    quote! {
                        aml
                            .first()
                            .is_some_and(|head| *head == #encoding_value)
                    }
                },
                Fields::Unnamed(FieldsUnnamed {
                    paren_token,
                    unnamed,
                }) => {
                    let Field {
                        attrs,
                        vis,
                        mutability,
                        ident,
                        colon_token,
                        ty,
                    } = unnamed
                        .first()
                        .unwrap();
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
                                "Vec" => match arguments {
                                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                        colon2_token,
                                        lt_token,
                                        args,
                                        gt_token,
                                    }) => match args.first().unwrap() {
                                        GenericArgument::Type(element_type) => quote! {
                                            if aml.is_empty() {
                                                true
                                            } else {
                                                #element_type::matches(aml)
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
                    }
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

