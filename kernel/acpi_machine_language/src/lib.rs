extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn,
};

#[proc_macro_derive(Symbol)]
pub fn acpi_machine_language(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        attrs,
        vis,
        ident, // Type name
        generics,
        data,
    } = syn::parse(input).unwrap();
    match data {
        syn::Data::Struct(syn::DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => match fields {
            syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token,
                unnamed,
            }) => unnamed
                .iter()
                .for_each(|field| {
                    let syn::Field {
                        attrs,
                        vis,
                        ident,
                        colon_token,
                        ty,
                        mutability,
                    } = field;
                    match ty {
                        syn::Type::Path(syn::TypePath {
                            qself,
                            path,
                        }) => {
                            let syn::Path {
                                leading_colon,
                                segments,
                            } = path;
                            segments
                                .iter()
                                .for_each(|path_segment| {
                                    let syn::PathSegment {
                                        ident, // Vec
                                        arguments,
                                    } = path_segment;
                                    match arguments {
                                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                            colon2_token,
                                            lt_token,
                                            args,
                                            gt_token,
                                        }) => args
                                            .iter()
                                            .for_each(|generic_argument| match generic_argument {
                                                syn::GenericArgument::Type(child_type) => {}, // TermObj
                                                _ => unimplemented!(),
                                            }),
                                        _ => unimplemented!(),
                                    }
                                });
                        },
                        _ => unimplemented!(),
                    }
                }),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    quote! {
    }.into()
}

