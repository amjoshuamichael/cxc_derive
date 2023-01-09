use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Type, TypeArray, TypePath};

#[proc_macro_derive(XcReflect)]
pub fn xc_reflect(tokens: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let fields_punctuated = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        Data::Enum(_) => todo!("XcReflect derive does not yet support enums"),
        _ => panic!("XcReflect derive does not yet support this type"),
    };

    let field_names = fields_punctuated.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! { #field_name: }
    });
    let field_types = fields_punctuated
        .iter()
        .map(|field| type_data_to_tokens(&field.ty));

    let output = quote! {
        impl cxc::XcReflect for #name {
            fn alias_code() -> String {
                let mut alias = stringify!(#name = ).to_string();
                alias += " { ";

                #(
                    alias += stringify!(#field_names);
                    alias += " ";
                    alias += #field_types;
                    alias += ", ";
                )*

                alias += "}";

                alias
            }
        }
    };

    TokenStream::from(output).into()
}

fn type_data_to_tokens(typ: &Type) -> TokenStream {
    match typ {
        Type::Path(TypePath { path, .. }) => {
            let type_name = path.segments.iter().last();
            quote! { stringify! ( #type_name ) }
        }
        Type::Array(TypeArray { elem, len, .. }) => {
            let base = type_data_to_tokens(&**elem);
            quote! {
                &*("[".to_string() + &*((#len).to_string()) + "]" + #base)
            }
        }
        Type::Tuple(_) => todo!("XcReflect derive does not yet support tuples"),
        _ => panic!("XcReflect derive does not support this field type"),
    }
}
