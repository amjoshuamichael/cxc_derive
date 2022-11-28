use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Type, TypePath};

#[proc_macro_derive(XcReflect)]
pub fn xc_reflect(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let fields_punctuated = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        Data::Enum(_) => todo!("XcReflect derive does not yet support enums"),
        _ => panic!("XcReflect derive does not support this type"),
    };

    let fields = fields_punctuated.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();

        let field_type_name = match &field.ty {
            Type::Path(TypePath { path, .. }) => path.segments.iter().last(),
            Type::Tuple(_) => todo!("XcReflect derive does not yet support tuples"),
            _ => panic!("XcReflect derive does not support this type"),
        };

        quote! { #field_ident: #field_type_name }
    });

    let output = quote! {
        impl cxc::XcReflect for #name {
            fn alias_code<'a>() -> &'a str {
                stringify!(
                    #name = {
                        #(#fields)*,
                    }
                )
            }
        }
    };

    TokenStream::from(output)
}
