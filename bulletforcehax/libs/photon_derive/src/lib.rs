extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attr;
mod get_type;
use get_type::get_packet_type_fn;

#[proc_macro_derive(PacketTypeImpl, attributes(packet_type))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let generics = &ast.generics;
    let data = match ast.data {
        syn::Data::Enum(e) => e,
        _ => panic!("Attribute only applies to enums"),
    };

    let get_type_cases = get_packet_type_fn(&name, &data);
    if let Err(err) = get_type_cases {
        return err.to_compile_error().into();
    }
    let get_type_cases = get_type_cases.unwrap();

    {
        quote! {
            impl #generics #name #generics {
                #get_type_cases
            }
        }
    }
    .into()
}
