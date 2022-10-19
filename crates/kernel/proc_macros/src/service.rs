use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand_service(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse2::<DeriveInput>(input).unwrap();

    let output = quote! {
        impl kernel_services::Service for #ident {}
    };

    output.into()
}
