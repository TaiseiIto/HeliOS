extern crate proc_macro;

use {
    proc_macro2::TokenTree,
    quote::quote,
    syn::{
        Attribute,
        DeriveInput,
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
    quote! {
    }   .try_into()
        .unwrap()
}

