mod entity;

use proc_macro::TokenStream;

#[proc_macro_derive(Identifiable)]
pub fn identifiable_fn(input: TokenStream) -> TokenStream {
    entity::implement_entity_trait::<false, false>(input.into()).into()
}

#[proc_macro_derive(Entity)]
pub fn entity_fn(input: TokenStream) -> TokenStream {
    entity::implement_entity_trait::<true, false>(input.into()).into()
}

#[proc_macro_derive(MutableEntity)]
pub fn mutable_entity_fn(input: TokenStream) -> TokenStream {
    entity::implement_entity_trait::<true, true>(input.into()).into()
}

#[proc_macro_attribute]
pub fn identifiable(_args: TokenStream, input: TokenStream) -> TokenStream {
    entity::add_entity_fields::<false, false>(input.into()).into()
}

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    entity::add_entity_fields::<true, false>(input.into()).into()
}

#[proc_macro_attribute]
pub fn mutable_entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    entity::add_entity_fields::<true, true>(input.into()).into()
}
