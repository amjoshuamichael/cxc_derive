#![doc = include_str!("../README.md")]

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, Type, TypeArray, TypePath, TypePtr,
    TypeTuple, TypeBareFn, ReturnType, TypeParen, TypeReference, PathArguments, AngleBracketedGenericArguments, GenericArgument, FieldsNamed, DataEnum, FieldsUnnamed, Ident,
};

#[proc_macro_attribute]
pub fn xc_opaque(_: TokenStream1, item: TokenStream1) -> TokenStream1 {
    item
}

#[proc_macro_derive(XcReflect)]
pub fn xc_reflect(tokens: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(tokens as DeriveInput);

    let xc_opaque = input.attrs.iter().any(|attribute| match attribute.path.segments.last() {
        Some(last_seg) => last_seg.ident.to_string() == String::from("xc_opaque"),
        None => false,
    });

    let name = input.ident;

    let output = match input.data {
        _ if xc_opaque => opaque_token_stream(name.clone()),
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => struct_token_stream(fields),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => tuple_token_stream(fields),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => quote! { alias += "{}"; },
        Data::Enum(enum_) => enum_token_stream(enum_),
        Data::Union(_) => todo!("cxc does not yet contain unions."),
    };

    let output = quote! {
        impl cxc::XcReflect for #name {
            fn alias_code() -> String {
                let mut alias = stringify!(#name =).to_string() + " ";

                #output

                alias
            }
        }
    };

    TokenStream::from(output).into()
}

fn struct_token_stream(fields: FieldsNamed) -> TokenStream {
    let field_names = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { #field_name }
    });

    let field_types = fields.named
        .iter()
        .map(|field| type_data_to_tokens(&field.ty));

    quote! {
        alias += "{ ";

        #(
            alias += stringify!(#field_names :);
            alias += " ";
            alias += #field_types;
            alias += ", ";
        )*

        alias += "}";
    }
}

fn tuple_token_stream(fields: FieldsUnnamed) -> TokenStream {
    let field_types = fields.unnamed
        .iter()
        .map(|field| type_data_to_tokens(&field.ty));

    quote! {
        alias += "{ ";

        #(
            alias += #field_types;
            alias += ", ";
        )*

        alias += "}";
    }
}

fn enum_token_stream(variants: DataEnum) -> TokenStream {
    let variant_names = variants.variants.iter().map(|variant| {
        let variant_name = variant.ident.clone();
        quote! { #variant_name }
    });

    let variant_types = variants.variants.iter().map(|variant| {
        match variant.fields.clone() {
            Fields::Named(fields_named) => {
                struct_token_stream(fields_named)
            },
            Fields::Unnamed(fields_unnamed) => {
                tuple_token_stream(fields_unnamed)
            },
            Fields::Unit => quote! {},
        }
    });

    quote! {
        alias += "{ ";

        #(
            alias += stringify! { #variant_names };
            alias += " : ";
            #variant_types;
            alias += " / ";
        )*

        alias += "}";
    }
}

fn opaque_token_stream(ident: Ident) -> TokenStream {
    quote! {
        use std::mem::size_of;

        let alignment = std::mem::align_of::<#ident>();
        let int_size = &*format!("u{}", alignment * 8);

        if size_of::<#ident>() != size_of::<Option<#ident>>() {
            if size_of::<#ident>() <= 8 {
                alias += "{ ";
                alias += int_size;
                alias += " }";
            } else {
                alias += "{ [ ";
                alias += &*(size_of::<#ident>() / alignment).to_string();
                alias += " ] ";
                alias += int_size;
                alias += " }";
            }
        } else if size_of::<#ident>() == 8 {
            alias += "{ &u32, }";
        } else {
            alias += "{ bool, ";

            for _ in 1..(size_of::<Option<#ident>>() / alignment) {
                alias += int_size;
                alias += ", ";
            }

            alias += "}";
        }
    }
}

fn type_data_to_tokens(typ: &Type) -> TokenStream {
    match typ {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.iter().last().cloned().unwrap();

            let type_name = last_segment.ident;
            let generic_arguments = last_segment.arguments;

            match generic_arguments {
                PathArguments::None => quote! { stringify! ( #type_name ) },
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                    let args = args.iter().filter_map(|arg| match arg {
                       GenericArgument::Type(typ) => Some(type_data_to_tokens(typ)),
                       GenericArgument::Const(_) => 
                           todo!("cxc_derive doesn't work with const generics"),
                       _ => None,
                    }).collect::<Vec<_>>();

                    quote! { &*(
                        String::from(stringify! ( #type_name )) + 
                            " < " + #( #args + ", ")+* + ">"
                    ) }
                }
                PathArguments::Parenthesized(_) => unreachable!(),
            }
        }
        Type::Array(TypeArray { elem, len, .. }) => {
            let base = type_data_to_tokens(&**elem);
            quote! {
                &*("[".to_string() + &*((#len).to_string()) + "]" + #base)
            }
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            let elems: Vec<_> = elems.iter().map(type_data_to_tokens).collect();
            quote! {
                &*("{ ".to_string() + #( #elems + ", ")+* + "}")
            }
        }
        Type::Ptr(TypePtr { elem, .. }) | Type::Reference(TypeReference { elem, .. }) => {
            let base = type_data_to_tokens(&**elem);
            quote! {
                &*("&".to_string() + #base)
            }
        }
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            let args = inputs.iter().map(|i| type_data_to_tokens(&i.ty)).collect::<Vec<_>>();

            let ret = match output {
                ReturnType::Default => quote! {""},
                ReturnType::Type(_, ret) => {
                    let ret = type_data_to_tokens(ret);
                    quote! { "; " + #ret }
                }
            };

            quote! {
                &*(String::from("(") + #(#args + ", ")+* + ")" + #ret)
            }
        }
        Type::Paren(TypeParen { elem, .. }) => type_data_to_tokens(&*elem),
        _ => todo!("XcReflect derive does not support this field type"),
    }
}
