mod entity;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity)]
pub fn entity_fn(input: TokenStream) -> TokenStream {
    entity::implement_entity_trait(input)
}

#[proc_macro_attribute]
pub fn add_entity_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    entity::add_entity_fields(input)
}
