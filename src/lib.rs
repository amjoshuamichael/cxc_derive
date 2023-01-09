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

    let fields = fields_punctuated.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = type_data_to_tokens(&field.ty);
        quote! { #field_ident: #field_ty }
    });

    let output = quote! {
        impl cxc::XcReflect for #name {
            fn alias_code<'a>() -> &'a str {
                stringify!(
                    #name = {
                        #(#fields),*
                    }
                )
            }
        }
    };

    TokenStream::from(output).into()
}

fn type_data_to_tokens(typ: &Type) -> TokenStream {
    match typ {
        Type::Path(TypePath { path, .. }) => {
            let type_name = path.segments.iter().last();
            quote! { #type_name }
        }
        Type::Array(TypeArray { elem, len, .. }) => {
            let base = type_data_to_tokens(&**elem);
            quote! { [ #len ] #base }
        }
        Type::Tuple(_) => todo!("XcReflect derive does not yet support tuples"),
        _ => panic!("XcReflect derive does not support this field type"),
    }
}
