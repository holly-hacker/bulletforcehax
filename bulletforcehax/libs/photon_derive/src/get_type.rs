use super::attr::*;
use proc_macro2::*;
use quote::quote;
use syn::*;

pub fn get_packet_type_fn(name: &Ident, data: &DataEnum) -> Result<TokenStream> {
    // ideally, this would be Result<Iter<TokenStream>> of some kind but I honestly can't make the damn borrow- and typechecker happy
    let cases: Result<Vec<TokenStream>> = data.variants.iter().map(|v| get_packet_type_cases(v, &name)).collect();

    cases.map(|cases| {
        quote! {
            pub fn get_type(&self) -> u8 {
                match self {
                    #(#cases,)*
                }
            }
        }
    })
}

fn get_packet_type_cases(v: &Variant, name: &Ident) -> Result<TokenStream> {
    // parse attribute
    let type_lit = get_packet_type_data(v)?;

    let ident = &v.ident;
    Ok(match &v.fields {
        Fields::Unit => quote!(#name::#ident => #type_lit),
        Fields::Unnamed(f) => {
            let underscores = f.unnamed.iter().map(|_| quote!(_));
            quote!(#name::#ident(#(#underscores,)*) => #type_lit)
        }
        Fields::Named(_f) => quote!(#name::#ident {..} => #type_lit),
    })
}
