extern crate proc_macro;

use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

mod attr;
use attr::{find_packet_type_attr, get_packet_type_lit};
mod utils;

#[proc_macro_derive(PacketTypeImpl, attributes(packet_type))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let data = match ast.data {
        syn::Data::Enum(e) => e,
        _ => panic!("Attribute only applies to enums"),
    };

    let get_type_cases = data.variants.iter().map(|v| {
        // parse attribute
        let type_lit = get_packet_type_lit(find_packet_type_attr(&v.attrs).expect(&format!("Couldn't find packet_type attribute on {}", v.ident)));

        let ident = &v.ident;
        match &v.fields {
            Fields::Unit => quote!(#name::#ident => #type_lit),
            Fields::Unnamed(f) => {
                let underscores = f.unnamed.iter().map(|_| quote!(_));
                quote!(#name::#ident(#(#underscores,)*) => #type_lit)
            }
            Fields::Named(_f) => quote!(#name::#ident {..} => #type_lit),
        }
    });

    {
        quote! {
            impl #name {
                pub fn get_type(&self) -> u8 {
                    match self {
                        #(#get_type_cases,)*
                    }
                }
            }
        }
    }
    .into()
}
