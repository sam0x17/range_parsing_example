use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, Token};

#[derive(Parse)]
struct RangeParam {
    start: Expr,
    _dotdot: Token![..],
    end: Expr,
}

#[proc_macro]
pub fn example_parse(tokens: TokenStream) -> TokenStream {
    let range_param = parse_macro_input!(tokens as RangeParam);
    let start = range_param.start.to_token_stream().to_string();
    let end = range_param.end.to_token_stream().to_string();
    quote!(concat!(#start, "..", #end)).into()
}
