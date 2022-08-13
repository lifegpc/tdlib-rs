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

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    match item {
        syn::Item::Struct(e) => match e.fields {
            syn::Fields::Named(fields) => {
                let ident = e.ident;
                let mut streams = Vec::new();
                let mut streams2 = Vec::new();
                for i in fields.named {
                    let name = i.ident;
                    let ty = i.ty;
                    streams.push(quote!(let #name = <#ty>::deserialize(data)?;));
                    streams2.push(quote!(#name,));
                }
                let stream = quote!(
                    impl crate::objects::traits::Deserialize for #ident {
                        type Error = crate::objects::error::DeserializeError;
                        fn deserialize<R: std::io::Read>(data: &mut R) -> Result<Self, Self::Error> {
                            use crate::objects::traits::Deserialize;
                            #(#streams)*
                            Ok(Self {
                                #(#streams2)*
                            })
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
                match &i.fields {
                    syn::Fields::Unnamed(fields) => {
                        if fields.unnamed.len() == 1 {
                            let ty = &fields.unnamed.first().unwrap().ty;
                            let tys = ty.to_token_stream().to_string();
                            match syn::parse_str::<BoxType>(&tys) {
                                Ok(b) => {
                                    let ty = b.ty;
                                    streams.push(quote!(if type_id == <#ty>::type_id2() {
                                        let v = <#ty>::deserialize(data)?;
                                        return Ok(Self::#name(Box::new(v)));
                                    }));
                                }
                                Err(_) => {
                                    streams.push(quote!(if type_id == <#ty>::type_id2() {
                                        let v = <#ty>::deserialize(data)?;
                                        return Ok(Self::#name(v));
                                    }));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            let ident = e.ident;
            let stream = quote!(
                impl crate::objects::traits::Deserialize for #ident {
                    type Error = crate::objects::error::DeserializeError;
                    fn deserialize<R: std::io::Read>(data: &mut R) -> Result<Self, Self::Error> {
                        use crate::objects::traits::Deserialize;
                        use crate::objects::traits::TypeId;
                        let type_id = u32::deserialize(data)?;
                        #(#streams)*
                        Err(Self::Error::from("No suitable variant found"))
                    }
                }
            );
            stream.into()
        }
        _ => panic!("Unimplemented"),
    }
}
