#[macro_use]
extern crate darling;
extern crate syn;

mod repo;

use proc_macro::TokenStream;

#[proc_macro_derive(Repo, attributes(repo))]
pub fn repo(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    repo::expand_repo(input).into()
}
