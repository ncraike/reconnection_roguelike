use super::utils::find_exactly_one_outer_helper_attr;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{Attribute, DeriveInput, Meta};

pub fn integer_unit_impl(input: &DeriveInput, primitive_type: TokenStream2) -> TokenStream2 {
    let name = input.ident.clone();

    quote! {
        use ::units::integer::IntegerUnit;

        impl IntegerUnit for #name {
            type PrimitiveType = #primitive_type;

            fn new(quantity: Self::PrimitiveType) -> Self {
                Self(quantity)
            }

            fn zero() -> Self {
                Self(0)
            }

            fn to_primitive(&self) -> i32 {
                self.0
            }

            fn abs(&self) -> Self {
                Self(self.to_primitive().abs())
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
    let parent_impl = integer_unit_impl(&input, primitive_type);

    let base_unit_attr =
        find_exactly_one_outer_helper_attr(&input, "base_unit", "#[base_unit[SomeType, 3, 4]]");
    let (base_unit_type, base_width, base_height) = parse_base_unit_helper_attr(&base_unit_attr);

    quote! {
        #parent_impl

        use ::units::integer::{DerivedIntegerUnitDisparateXY, XYAxes};
        use ::units::utils::{div_ceil, div_floor};

        impl DerivedIntegerUnitDisparateXY for #name {
            type BaseUnit = #base_unit_type;

            fn to_base_unit(&self, in_axis: XYAxes) -> Self::BaseUnit {
                match in_axis {
                    XYAxes::X => Self::BaseUnit(self.to_primitive() * #base_width),
                    XYAxes::Y => Self::BaseUnit(self.to_primitive() * #base_height),
                }
            }

            fn from_base_unit_to_floor(base_quantity: Pixels, in_axis: XYAxes) -> Self {
                match in_axis {
                    XYAxes::X => Self(div_floor(base_quantity, #base_width)),
                    XYAxes::Y => Self(div_floor(base_quantity, #base_height)),
                }
            }

            fn from_base_unit_to_ceil(base_quantity: Pixels, in_axis: XYAxes) -> Self {
                match in_axis {
                    XYAxes::X => Self(div_ceil(base_quantity, #base_width)),
                    XYAxes::Y => Self(div_ceil(base_quantity, #base_height)),
                }
            }
        }
    }
}
