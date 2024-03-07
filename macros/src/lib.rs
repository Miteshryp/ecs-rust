extern crate proc_macro;

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
#[proc_macro_derive(System)]
pub fn system_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let type_name = ast.ident;

    let generate = quote! {
        impl System for #type_name {
            pub fn as_any(&self) -> &dyn Any {
                self as &dyn Any;
            }

            pub fn as_any_mut(&mut self) -> &mut dyn System {
                self as &mut dyn Any
            }
        }
    };

    generate.into()
}


/// #### ECS Component derive
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let type_name = ast.ident;
    
    let gen = quote! {
        impl Component for #type_name {
            fn get_name() -> String {
                String::from(stringify!(#type_name))
            }

            fn into_component_type(&self) -> &Self {
                self
            }

            fn into_component_type_mut(&mut self) -> &mut Self {
                self
            }
        }
    };

    gen.into()
}