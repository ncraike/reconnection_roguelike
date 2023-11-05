use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn integer_unit_impl(input: DeriveInput, primitive_type: TokenStream2) -> TokenStream2 {
    let name = input.ident;

    quote! {
        use units::integer::IntegerUnit;

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
