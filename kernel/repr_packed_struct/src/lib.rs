extern crate proc_macro;

use {
    proc_macro2::TokenTree,
    quote::{
        format_ident,
        quote,
    },
    std::iter,
    syn::{
        Attribute,
        Data,
        DataStruct,
        DeriveInput,
        Field,
        Fields,
        FieldsNamed,
        Ident,
        Meta,
        MetaList,
        Path,
        PathSegment,
        parse,
    },
};

#[proc_macro_derive(OffsetGetter)]
pub fn derive_offset_getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics: _,
        data,
    } = parse(input).unwrap();
    let repr_packed: bool = attrs
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
                        "repr" => tokens
                            .clone()
                            .into_iter()
                            .any(|token_tree| match token_tree {
                                TokenTree::Ident(repr_arg) => {
                                    let repr_arg: String = repr_arg.to_string();
                                    matches!(repr_arg.as_str(), "packed")
                                },
                                _ => false,
                            }),
                        _ => false,
                    }
                },
                _ => false,
            }
        });
    assert!(repr_packed);
    let getters: Vec<proc_macro2::TokenStream> = match data {
        Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Named(FieldsNamed {
                brace_token: _,
                named,
            }),
            semi_token: _,
        }) => {
            let fields: Vec<&Field> = named
                .iter()
                .collect();
            let previous_fields: Vec<Option<&Field>> = iter::once(None)
                .chain(fields[..fields.len() - 1]
                    .iter()
                    .map(|field| Some(*field)))
                .collect();
            fields
                .into_iter()
                .zip(previous_fields)
                .map(|(field, previous_field)| {
                    let Field {
                        attrs: _,
                        vis: _,
                        mutability: _,
                        ident,
                        colon_token: _,
                        ty: _,
                    } = field;
                    let ident: &Ident = ident
                        .as_ref()
                        .unwrap();
                    let getter_name: Ident = format_ident!("{}_offset", ident);
                    let getter: proc_macro2::TokenStream = match previous_field {
                        Some(previous_field) => {
                            let Field {
                                attrs: _,
                                vis: _,
                                mutability: _,
                                ident,
                                colon_token: _,
                                ty,
                            } = previous_field;
                            let ident: &Ident = ident
                                .as_ref()
                                .unwrap();
                            let previous_getter_name: Ident = format_ident!("{}_offset", ident);
                            quote! {
                                Self::#previous_getter_name() + core::mem::size_of::<#ty>()
                            }
                        },
                        None => quote! {
                            0
                        },
                    };
                    quote! {
                        pub const fn #getter_name() -> usize {
                            #getter
                        }
                    }
                })
                .collect()
        },
        _ => unreachable!(),
    };
    quote! {
        impl #ident {
            #(#getters)*
        }
    }   .try_into()
        .unwrap()
}

