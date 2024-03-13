use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_quote};

/// 
/// Implementation for the [`ComponentSystem`](crate::ComponentSystem) proc macro
/// 
pub(crate) fn component_system_impl(ast: syn::DeriveInput) -> TokenStream {
    let type_name = ast.ident;

    let generate = quote! {
        impl BaseSystem for #type_name {
            fn as_any(&self) -> &dyn Any {
                self as &dyn Any
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self as &mut dyn Any
            }
        
            fn process_update(&mut self, world_container: &mut UnsafeWorldContainer) {
                // Getting the system in self type
                let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();
                
                // SAFETY:
                // We get the world as a mutable reference only to fetch the immutable 
                // references of components inside the world. We do not modify the world
                // in any way.
                let components_list: Vec<&Entity> = 
                    world_container.get_world().get_all_component_ids::<<#type_name as ComponentSystem>::ComponentType>();
                
                // Running updates on all components
                for entity_id in
                components_list
                {
        
                    // SAFETY:
                    // The on_update function is going to be user implemented, hence user must
                    // not possess 2 mutable references to the components. We supply the user
                    // with the mutable component reference, hence the user should not need 
                    // another mutable reference to the same component.
                    // 
                    // The user cannot get a new mutable reference to the same type since
                    // the component manager stores the components in a RefCell, and each
                    // component is fetched using a RefMut type. 
                    // we fetched the entity ids of all components of the given type, and
                    // we occupy the RefMut for the component that is being processed and 
                    // passing that in the `component` parameter.
                    // So, if the user tries to get a mutable reference to the component
                    // being processed, it will result in a panic since that component is 
                    // already borrowed in this function.
                    // 
                    // For the 2 calls to get_world_mut, only one lasts and is propogated into
                    // the on_update call.
                    (system
                        as &mut dyn ComponentSystem<
                            ComponentType = <#type_name as ComponentSystem>::ComponentType,
                        >)
                        .on_update(&mut world_container.get_world_mut(), *entity_id, world_container.get_world_mut().get_component_mut_ref(*entity_id));
                }
            }
        
            fn process_start(&mut self, world: &mut UnsafeWorldContainer) {
                let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();
                let mut world = &mut *world;
        
                // SAFETY:
                // This is the only existing mutable world reference in the current scope.
                system.on_start(&mut world.get_world_mut());
            }
        }
    };

    generate.into()
}


/// Implementation of the [`Component`](crate::Component) proc macro
pub(crate) fn component_impl(mut ast: syn::DeriveInput) -> TokenStream {
    let type_name = ast.ident;

    // ast.generics.make_where_clause().predicates.push(parse_quote! {});

    // let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    
    let gen = quote! {
        impl Component for #type_name {
            fn get_name() -> String {
                String::from(stringify!(#type_name))
            }

            fn as_any(&self) -> &dyn Any {
                self as &dyn Any
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self as &mut dyn Any
            }
        }
    };

    gen.into()
}