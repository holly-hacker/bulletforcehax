use super::utils::IterSingleExt;
use syn::{AttrStyle, Attribute, Lit, Meta, NestedMeta};

pub fn find_packet_type_attr(attrs: &Vec<Attribute>) -> Option<&Attribute> {
    Some(attrs.iter().filter(is_packet_type_attr).single().expect("Couldn't find attribute"))
}

fn is_packet_type_attr(attr: &&Attribute) -> bool {
    attr.path.is_ident("packet_type") && attr.style == AttrStyle::Outer
}

pub fn get_packet_type_lit(attr: &Attribute) -> Lit {
    // pretty shitty and roundabout way of just getting the first literal
    if let Ok(meta) = attr.parse_meta() {
        match meta {
            Meta::List(list) => {
                let nested = list.nested;
                assert_eq!(nested.len(), 1, "Expected only 1 constant");
                if let NestedMeta::Lit(lit) = nested.first().unwrap() {
                    lit.to_owned()
                } else {
                    panic!("Expected literal");
                }
            }
            _ => panic!("Expected list"),
        }
    } else {
        panic!("Could not parse attribute parameters");
    }
}
