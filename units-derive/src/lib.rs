extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(annotated_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
