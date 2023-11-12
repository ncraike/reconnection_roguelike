use syn::{Attribute, DeriveInput};

#[allow(dead_code)]
pub fn has_attr_named(attrs: &Vec<Attribute>, name: &str) -> bool {
    for attr in attrs.iter() {
        if attr.path().is_ident(name) {
            return true;
        }
    }
    return false;
}

pub fn find_attrs_named<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Vec<&'a Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident(name))
        .collect()
}

pub fn find_exactly_one_outer_helper_attr(
    input: &DeriveInput,
    name: &str,
    example: &str,
) -> Attribute {
    let matching_attrs: Vec<&Attribute> = find_attrs_named(&input.attrs, name);
    match matching_attrs.len() {
        1 => (),
        _ => {
            let panic_str = format!(
                "Please include exactly one #[{}(...)] helper attribute. Example: {}",
                name, example
            );
            panic!("{}", panic_str);
        }
    }
    assert_eq!(matching_attrs.len(), 1);
    matching_attrs[0].clone()
}
