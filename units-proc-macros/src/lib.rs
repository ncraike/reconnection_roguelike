use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod integer;

#[proc_macro_derive(IntegerUnitI32)]
pub fn derive_integer_unit_i32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // TODO: infer this from struct field
    let primitive_type = quote! { i32 };
    TokenStream::from(integer::integer_unit_impl(input, primitive_type))
}
