use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, DeriveInput};

pub fn integer_unit_impl(input: DeriveInput, primitive_type: TokenStream2) -> TokenStream2 {
    let name = input.ident;

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

fn find_helper_attrs(input: DeriveInput, name: &str) -> Vec<&Attribute> {
    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(name))
        .collect()
}

pub fn derived_integer_unit_impl(input: DeriveInput, primitive_type: TokenStream2) -> TokenStream2 {
    let name = input.ident;
    let parent_impl = integer_unit_impl(input, primitive_type);
    let base_unit_attrs: Vec<&Attribute> = find_helper_attrs(input, "base_unit");
    if base_unit_attrs.len() > 1 {
        panic!("Only define #[base_unit(...)] once")
    }
    let base_unit_attr = *base_unit_attrs[0];

    base_unit

    quote! {
        #parent_impl

        use ::units::integer::{DerivedIntegerUnitDisparateXY, XYAxes};

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
                    XYAxes::X => Self(div_floor(base_quantity, TILES_1X_WIDTH_IN_PIXELS)),
                    XYAxes::Y => Self(div_floor(base_quantity, TILES_1X_HEIGHT_IN_PIXELS)),
                }
            }

            fn from_base_unit_to_ceil(base_quantity: Pixels, in_axis: XYAxes) -> Self {
                match in_axis {
                    XYAxes::X => Self(div_ceil(base_quantity, TILES_1X_WIDTH_IN_PIXELS)),
                    XYAxes::Y => Self(div_ceil(base_quantity, TILES_1X_HEIGHT_IN_PIXELS)),
                }
            }
        }
    }
}
