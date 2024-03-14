extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn;

mod component;
mod system;
mod base;

/// ### ECS System derive
///
/// Used to implement the internal `BaseSystem` structure fitting to the
/// component system.
///
///
/// After declaring the struct as a `ComponentSystem`, the user can
/// implement the `System` trait on the struct. Once this process is
/// complete, the [`App`](ecs_rust::App) class can register the system as a component
/// system.
///
/// ---
///
/// ### Example:
///
/// ```
///
/// #[derive(Component)]
/// struct Position {
///     x: f32,
///     y: f32
/// }
///
/// #[derive(ComponentSystem)]
/// struct PositionSystem {
///     // .. (We can declare a system specific state if we want to)
/// }
///
/// impl ComponentSystem for PositionSystem {
///     type ComponentType = Position;
///
///     fn on_update(&self, world: WorldArg, entity_id: EntityId, component: &mut TestComponent) {
///         // update logic for the component
///     }
/// }
/// 
/// fn main() {
///     let app = App::new();
///     app.add_component_system(PositionSystem {});
///     ...
///     ... // component addition
///     app.start();
///         
/// }
///
/// ```
///
/// @TODO: Add different types of events in the future
///
#[proc_macro_derive(ComponentSystem)]
pub fn system_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    system::derive_component_system(ast)
}

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
/// struct 
/// ```
///
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    component::derive_component(ast)
}


#[proc_macro_derive(ECSBase)]
pub fn base_derive(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();
    base::derive_base(&mut ast).to_token_stream().into()
}
