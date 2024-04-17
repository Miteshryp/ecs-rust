extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{self, parse_macro_input};
use utils::AllTuples;

mod base;
mod component;
mod resource;
mod event;
mod utils;
mod system_param;




/// ### ECS Component derive
///
/// Used to implement correct `Component` trait for the type to enable
/// compatibility with the ECS system.
///
/// Only after attaching this derive on a Component can we appropriately
/// add it to a `World` in an app after registering the component in the
/// app.
/// ---
///
/// ### Example:
///
/// ```
/// #[derive(Component)]
/// struct Position {
///     x: f32,
///     y: f32
/// }
/// // The position component can now be registered in a world
/// // and be used.
/// ```
///
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    component::derive_component(ast)
}

#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    resource::derive_resource(ast)
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    event::derive_event(ast)
}

#[proc_macro_derive(ECSBase)]
pub fn base_derive(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    base::derive_base(&mut ast).to_token_stream().into()
}

#[proc_macro_derive(SystemParam)]
pub fn system_param_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    system_param::derive_system_param(ast).into()
}



#[proc_macro]
pub fn implement_tuples(input: TokenStream) -> TokenStream {
    let tuple_info = parse_macro_input!(input as AllTuples);
    let invocation_len = tuple_info.end - tuple_info.start + 1;
    
    let mut macro_parameters: Vec<proc_macro2::TokenStream> = Vec::with_capacity(invocation_len);
    
    for index in tuple_info.start..=tuple_info.end {
        let idents = tuple_info.idents.iter().map(|ident| {
            format_ident!("{}{}", ident, index)
        });

        if tuple_info.idents.len() < 2 {
            macro_parameters.push(quote! {
                #(#idents)* // [P0, P1, ... , Pn]
            });
        } else {
            macro_parameters.push(quote!{
                (#(#idents),*) // [(P0, F0), (P1, F1) ... , (Pn, Fn)]
            });
        }
    }

    let macro_caller = tuple_info.macro_caller;
    let invocations = (tuple_info.start..=tuple_info.end).map(|i| {
        let param_range = &macro_parameters[..=i]; // [(P0,F0), (P1,F1), .. (Pi,Fi), ... (Pn, Fn)]
        quote! {
            #macro_caller!(#(#param_range),*);
        }
    });
    
    let generate = quote! {
        #(#invocations)*
    };
    generate.into()
}
