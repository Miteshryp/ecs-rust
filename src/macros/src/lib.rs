extern crate proc_macro;

mod impl_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn;


/// ECS System derive
/// 
/// @TODO: Might need to change the name as well as implementation
///     based on how we define the user level interface for a system
///     At the time of implementing this macro, the [System] struct is 
///     a internal use struct used for defining [ComponentSystem]. The 
///     struct being used by the user is a [ComponentHandler]
/// 
#[proc_macro_derive(ComponentSystem)]
pub fn system_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_macro::component_system_impl(ast)
}


/// #### ECS Component derive
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_macro::component_impl(ast)
}