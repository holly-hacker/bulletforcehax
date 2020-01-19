use syn::{spanned::Spanned, *};

// TODO: return struct when we need to return multiple items
pub fn get_packet_type_data(v: &Variant) -> Result<Lit> {
    let attr = find_attr(v, "packet_type")?;

    // pretty shitty and roundabout way of just getting the first literal
    if let Ok(meta) = attr.parse_meta() {
        match meta {
            Meta::List(list) => {
                let nested = list.nested;
                assert_eq!(nested.len(), 1, "Expected only 1 constant");
                if let NestedMeta::Lit(lit) = nested.first().unwrap() {
                    Ok(lit.to_owned())
                } else {
                    Err(Error::new(nested.span(), "Expected literal"))
                }
            }
            _ => Err(Error::new(meta.span(), "Expected list")),
        }
    } else {
        Err(Error::new(attr.span(), "Could not parse attribute parameters"))
    }
}

/// Tries to find exactly 1 outer attribute matching `name`, otherwise returns `Error`
pub fn find_attr<'a>(variant: &'a Variant, name: &'static str) -> Result<&'a Attribute> {
    let mut iter = variant
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident(name) && attr.style == AttrStyle::Outer);

    if let Some(first) = iter.next() {
        if let Some(_second) = iter.next() {
            return Err(Error::new(variant.span(), format!("Found multiple {} attributes", name)));
        } else {
            Ok(first)
        }
    } else {
        Err(Error::new(variant.span(), format!("Couldn't find {} attribute", name)))
    }
}
