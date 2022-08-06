use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    match item {
        syn::Item::Struct(e) => match e.fields {
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
        },
        syn::Item::Enum(e) => {
            let mut streams = Vec::new();
            for i in e.variants {
                let name = i.ident;
                streams.push(quote!(Self::#name(n) => { n.serialize() }));
            }
            let ident = e.ident;
            let stream = quote!(
                impl crate::objects::traits::Serialize for #ident {
                    fn serialize(&self) -> Vec<u8> {
                        match self {
                            #(#streams)*
                        }
                    }
                }
            );
            stream.into()
        }
        _ => panic!("Unimplemented"),
    }
}

struct BoxType {
    ty: syn::Type,
}

impl Parse for BoxType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = syn::Ident::parse(input)?.to_string();
        if ident != "Box" {
            Err(syn::Error::new(input.span(), "Expected Box<T>"))
        } else {
            syn::token::Lt::parse(input)?;
            let ty = syn::Type::parse(input)?;
            syn::token::Gt::parse(input)?;
            Ok(BoxType { ty })
        }
    }
}

#[proc_macro_derive(From1)]
pub fn derive_from1(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    match item {
        syn::Item::Enum(e) => {
            let ident = e.ident;
            let mut streams = Vec::new();
            for i in e.variants {
                let name = i.ident;
                match &i.fields {
                    syn::Fields::Unnamed(fields) => {
                        if fields.unnamed.len() == 1 {
                            let ty = &fields.unnamed.first().unwrap().ty;
                            streams.push(quote!(
                                impl std::convert::From<#ty> for #ident {
                                    fn from(v: #ty) -> Self {
                                        Self::#name(v)
                                    }
                                }
                            ));
                            let mut tys = ty.to_token_stream().to_string();
                            let mut s = quote!(v);
                            loop {
                                match syn::parse_str::<BoxType>(&tys) {
                                    Ok(b) => {
                                        tys = b.ty.to_token_stream().to_string();
                                        s = quote!(Box::new(#s));
                                        let ty = b.ty;
                                        streams.push(quote!(
                                            impl std::convert::From<#ty> for #ident {
                                                fn from(v: #ty) -> Self {
                                                    Self::#name(#s)
                                                }
                                            }
                                        ));
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            let stream = quote!(
                #(#streams)*
            );
            stream.into()
        }
        _ => panic!("Unimplemented"),
    }
}
