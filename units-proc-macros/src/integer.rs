use super::utils::find_exactly_one_outer_helper_attr;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{Attribute, DeriveInput, Meta};

pub fn integer_unit_impl(input: &DeriveInput, primitive_type: TokenStream2) -> TokenStream2 {
    let name = input.ident.clone();

    quote! {
        use ::units::integer::IntegerUnit;

        impl IntegerUnit<#primitive_type> for #name {
            fn new(quantity: #primitive_type) -> Self {
                Self(quantity)
            }

            fn zero() -> Self {
                Self(0)
            }

            fn to_primitive(&self) -> #primitive_type {
                self.0
            }

            fn abs(&self) -> Self {
                Self::new(self.to_primitive().abs())
            }
        }
    }
}

pub fn parse_base_unit_helper_attr(
    base_unit_attr: &Attribute,
) -> (TokenStream2, TokenStream2, TokenStream2) {
    let base_unit_tokens = match &base_unit_attr.meta {
        Meta::List(meta_list) => &meta_list.tokens,
        _ => panic!("Use #[base_unit(...)] with type, width and height. Example: #[base_unit[SomeType, 3, 4]]"),
    };
    let base_unit_token_trees: Vec<TokenTree> = base_unit_tokens.clone().into_iter().collect();
    match base_unit_token_trees.len() {
        5 => (),
        _ => panic!("Use #[base_unit(...)] with type, width and height. Example: #[base_unit[SomeType, 3, 4]]"),
    }

    let base_unit_type = base_unit_token_trees[0].clone();
    let _commma = base_unit_token_trees[1].clone();
    let base_width = base_unit_token_trees[2].clone();
    let _commma = base_unit_token_trees[3].clone();
    let base_height = base_unit_token_trees[4].clone();

    return (base_unit_type.into(), base_width.into(), base_height.into());
}

pub fn derived_integer_unit_impl(
    input: &DeriveInput,
    primitive_type: TokenStream2,
) -> TokenStream2 {
    let name = input.ident.clone();
    let parent_impl = integer_unit_impl(&input, primitive_type.clone());

    let base_unit_attr =
        find_exactly_one_outer_helper_attr(&input, "base_unit", "#[base_unit[SomeType, 3, 4]]");
    let (base_unit_type, base_width, base_height) = parse_base_unit_helper_attr(&base_unit_attr);

    quote! {
        #parent_impl

        use ::units::integer::{DerivedIntegerUnitDisparateXY, XYAxes};
        use ::units::utils::{div_ceil, div_floor};

        impl DerivedIntegerUnitDisparateXY<#primitive_type, #base_unit_type> for #name {

            fn to_base_unit(&self, in_axis: XYAxes) -> #base_unit_type {
                match in_axis {
                    XYAxes::X => #base_unit_type::new(self.to_primitive() * #base_width),
                    XYAxes::Y => #base_unit_type::new(self.to_primitive() * #base_height),
                }
            }

            fn from_base_unit_to_floor(base_quantity: #base_unit_type, in_axis: XYAxes) -> Self {
                match in_axis {
                    XYAxes::X => Self(div_floor(base_quantity, #base_width)),
                    XYAxes::Y => Self(div_floor(base_quantity, #base_height)),
                }
            }

            fn from_base_unit_to_ceil(base_quantity: #base_unit_type, in_axis: XYAxes) -> Self {
                match in_axis {
                    XYAxes::X => Self(div_ceil(base_quantity, #base_width)),
                    XYAxes::Y => Self(div_ceil(base_quantity, #base_height)),
                }
            }
        }
    }
}

pub fn parse_convert_to_helper_attr(convert_to_attr: &Attribute) -> Vec<TokenStream2> {
    let convert_to_tokens = match &convert_to_attr.meta {
        Meta::List(meta_list) => meta_list.tokens.clone(),
        _ => panic!(
            "Use #[convert_to(...)] with a base type and series of types to convert to. Example: #[convert_to(MyBaseType, SomeType, SomeOtherType)]"
        ),
    };
    let convert_to_token_trees: Vec<TokenTree> = convert_to_tokens.into_iter().collect();
    let type_idents: Vec<TokenTree> = convert_to_token_trees
        .iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Ident(_ident) => Some(token_tree.clone()),
            _ => None,
        })
        .collect();
    if type_idents.len() < 2 {
        panic!("Use #[convert_to(...)] with a base type and series of types to convert to. Example: #[convert_to(MyBaseType, SomeType, SomeOtherType)]");
    }
    assert!(type_idents.len() > 0);

    type_idents
        .iter()
        .map(|ident_tt| TokenStream2::from(ident_tt.clone()))
        .collect()
}

pub fn convert_integer_unit_impl(
    input: &DeriveInput,
    primitive_type: TokenStream2,
) -> TokenStream2 {
    let name = input.ident.clone();

    let convert_to_attr = find_exactly_one_outer_helper_attr(
        &input,
        "convert_to",
        "#[convert_to(BaseType, SomeType, SomeOtherType)]",
    );
    let convert_to_types = parse_convert_to_helper_attr(&convert_to_attr);

    let mut result = quote! {
        use ::units::integer::ConvertibleIntegerUnitDisparateXY;
    };

    let base_type = convert_to_types[0].clone();
    for other_unit_type in convert_to_types[1..].iter() {
        let convert_impl = quote! {
            impl ConvertibleIntegerUnitDisparateXY<#primitive_type, #base_type, #other_unit_type> for #name {
                fn convert_to_floor(&self, in_axis: XYAxes) -> #other_unit_type {
                    #other_unit_type::from_base_unit_to_floor(self.to_base_unit(in_axis), in_axis)
                }

                fn convert_to_ceil(&self, in_axis: XYAxes) -> #other_unit_type {
                    #other_unit_type::from_base_unit_to_ceil(self.to_base_unit(in_axis), in_axis)
                }
            }
        };
        result.extend(convert_impl);
    }

    result
}
