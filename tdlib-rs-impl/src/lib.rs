use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    match item {
        syn::Item::Struct(e) => {
            match e.fields {
                syn::Fields::Named(fields) => {
                    let ident = e.ident;
                    let mut streams = Vec::new();
                    for i in fields.named {
                        let name = i.ident;
                        streams.push(quote!(v.extend_from_slice(&self.#name.serialize());));
                    }
                    let stream = quote!(
                        impl crate::objects::traits::Serialize for #ident {
                            fn serialize(&self) -> Vec<u8> {
                                let mut v = Vec::new();
                                #(#streams)*
                                v
                            }
                        }
                    );
                    stream.into()
                }
                _ => panic!("Unimplemented"),
            }
        }
        _ => panic!("Unimplemented"),
    }
}
