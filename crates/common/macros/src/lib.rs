mod detail;

use proc_macro::TokenStream;
use syn::{parse::Parser, *};

#[proc_macro_attribute]
pub fn append_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let field = Field::parse_named.parse(args).unwrap();

    detail::proc::util::append::append_field(input, field)
}
