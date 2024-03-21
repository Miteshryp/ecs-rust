// use proc_macro::TokenStream;
// use syn::{parse};
// use quote::quote;

// use crate::{base::derive_base};

// ///
// /// Implementation for the [`ComponentSystem`](crate::ComponentSystem) proc macro
// ///
// pub(crate) fn derive_component_system(mut ast: syn::DeriveInput) -> TokenStream {
//     let base_impl: proc_macro2::TokenStream = derive_base(&mut ast);
//     let type_name: syn::Ident = ast.ident;

//     let generate: proc_macro2::TokenStream = quote! {
//         #base_impl
//         impl BaseSystem for #type_name {

//             fn process_update(&mut self, world_container: &mut UnsafeWorldContainer) {
//                 // Getting the system in self type
//                 let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();

//                 // SAFETY:
//                 // We get the world as a mutable reference only to fetch the immutable
//                 // references of components inside the world. We do not modify the world
//                 // in any way.
//                 let components_list: Vec<&Entity> =
//                     world_container.get_world().get_all_component_ids::<<#type_name as ComponentSystem>::ComponentType>();

//                 // Running updates on all components
//                 for entity_id in
//                 components_list
//                 {

//                     // SAFETY:
//                     // The on_update function is going to be user implemented, hence user must
//                     // not possess 2 mutable references to the components. We supply the user
//                     // with the mutable component reference, hence the user should not need
//                     // another mutable reference to the same component.
//                     //
//                     // The user cannot get a new mutable reference to the same type since
//                     // the component manager stores the components in a RefCell, and each
//                     // component is fetched using a RefMut type.
//                     // we fetched the entity ids of all components of the given type, and
//                     // we occupy the RefMut for the component that is being processed and
//                     // passing that in the `component` parameter.
//                     // So, if the user tries to get a mutable reference to the component
//                     // being processed, it will result in a panic since that component is
//                     // already borrowed in this function.
//                     //
//                     // For the 2 calls to get_world_mut, only one lasts and is propogated into
//                     // the on_update call.
//                     (system
//                         as &mut dyn ComponentSystem<
//                             ComponentType = <#type_name as ComponentSystem>::ComponentType,
//                         >)
//                         .on_update(&mut world_container.get_world_mut(), *entity_id, world_container.get_world_mut().get_component_mut_ref(*entity_id));
//                 }
//             }

//             fn process_start(&mut self, world: &mut UnsafeWorldContainer) {
//                 let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();
//                 let mut world = &mut *world;

//                 // SAFETY:
//                 // This is the only existing mutable world reference in the current scope.
//                 system.on_start(&mut world.get_world_mut());
//             }
//         }
//     };

//     generate.into()
// }

use crate::base::derive_base;
use quote::quote;




pub fn derive_resource_system(mut input: syn::DeriveInput) -> proc_macro::TokenStream {
    let base_derive = derive_base(&mut input);
    
    // let (impl_generic, impl_type , where_clause) = input.generics.split_for_impl();
    let type_name = input.ident;

    let generate = quote! {
        #base_derive
        impl BaseSystem for #type_name {
            fn process_update(&mut self, world_container: &mut UnsafeWorldContainer) {
                let resource_id = world_container.get_world().get_resource_id::< <Self as ResourceSystem>::ResourceType>();
                self.on_update(&mut world_container.get_world_mut(), );
            }

            fn process_start(&mut self, world_container: &mut UnsafeWorldContainer) {

            }

            fn process_events(&mut self, world_container: &mut UnsafeWorldContainer) {

            }
        }
    };

    generate.into()
}
