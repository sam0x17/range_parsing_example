use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn example_parse(_tokens: TokenStream) -> TokenStream {
    quote!().into()
}
