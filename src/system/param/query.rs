use std::{
    any::TypeId,
    cell::{Ref, RefMut},
    ops::Deref,
    vec::IntoIter,
};

use crate::{
    component::Component,
    entity::Entity,
    world::{unsafe_world::UnsafeWorldContainer, World},
};

use super::SystemParam;

pub(crate) trait SystemQuery<'a> {
    type EntityComponentRefTuple;

    /// Gets the type_ids of specified component types in the tuple.
    /// This vector of type_ids could then be passed into the world
    /// to get a list of components which have the specified components
    /// attached to them
    ///
    /// We can then iterate through this list of entities to fetch the
    /// appropriate component handles
    fn get_query_entities() -> Vec<TypeId>;

    // fn get_components_for_entities(&self, world: &mut World) -> Option<Vec<Self::EntityRefMutTuple>>;
    fn get_components_for_entities(
        // &self,
        // world: &UnsafeWorldContainer,
        world: *mut World
    ) -> Option<Vec<Self::EntityComponentRefTuple>>;
}

/// A internal structure to enclose the mutable component reference
/// fetched from the world.
/// This is primarily made to fool the borrow checker into thinking
/// that the RefMut has a static lifetime
pub struct QueryComponentRefMut<C: Component + 'static> {
    inner_boxed_component: Box<RefMut<'static, C>>,
}
impl<C: Component + 'static> Deref for QueryComponentRefMut<C> {
    type Target = RefMut<'static, C>;

    fn deref(&self) -> &Self::Target {
        self.inner_boxed_component.as_ref()
    }
}



// @TODO: Remove this code block after reference usage has exhausted
//      Test the derived implementation
// Sample implementation of SystemQuery
// impl<'a, T1, T2> SystemQuery<'a> for (T1, T2)
// where
//     T1: Component + 'static,
//     T2: Component + 'static,
// {
//     // type EntityRefMutTuple = (Entity, RefMut<'a, T1>, RefMut<'a, T2>);
//     type EntityRefMutTuple = (Entity, QueryComponentRefMut<T1>, QueryComponentRefMut<T2>);

//     fn get_query_entities() -> Vec<TypeId> {
//         vec![std::any::TypeId::of::<T1>(), std::any::TypeId::of::<T2>()]
//     }

//     // fn get_components_for_entities(&self, world: &mut World) -> Option<Vec<Self::EntityRefMutTuple>> {
//     fn get_components_for_entities(
//         &self,
//         world: &UnsafeWorldContainer,
//     ) -> Option<Vec<Self::EntityRefMutTuple>> {
//         // 1. Get all entities which have the components mentioned in the tuple
//         // 2. Get the mutable component access for each one of them, and push it to the vec
//         // 3. If the component fetch fails, this means that either component is unavailable,
//         //      or it has been deleted.
//         //    3.a In this case we surrender all acquired component references and return None
//         // 4. If all acquisitions were successful, we have successfully acquire state access into
//         //      the world for all the required components. We can finally return

//         let world_ptr: *mut World = world.get_world_mut();

//         let entities: hashbrown::HashSet<&Entity> =
//             unsafe { (*world_ptr).get_entities_with_components::<Self>() };
//         let mut aggregated_vec: Vec<Self::EntityRefMutTuple> = vec![];

//         // Acquiring Component references with their corressponding entities
//         for entity in entities {
//             let tuple = (
//                 *entity,
//                 match unsafe { (*world_ptr).get_boxed_component_mut_ref::<T1>(*entity) } {
//                     Some(x) => QueryComponentRefMut {
//                         inner_boxed_component: x,
//                     },
//                     None => {
//                         return None;
//                     }
//                 },
//                 match unsafe { (*world_ptr).get_boxed_component_mut_ref::<T2>(*entity) } {
//                     Some(x) => QueryComponentRefMut {
//                         inner_boxed_component: x,
//                     },
//                     None => {
//                         return None;
//                     }
//                 },
//             );

//             aggregated_vec.push(tuple);
//         }

//         Some(aggregated_vec)
//     }
// }

macro_rules! query_systems {
        ($($param: ident),*) => {

            #[allow(non_snake_case)]
            impl<'a, $($param: Component + 'static),*> SystemQuery<'a> for (Entity, $($param),*) {
                type EntityComponentRefTuple = (Entity, $(QueryComponentRefMut<$param>),*);

                fn get_query_entities(
                ) -> Vec<TypeId> {
                    vec![$(std::any::TypeId::of::<$param>()),*]
                }

                fn get_components_for_entities(
                    world: *mut World,
                ) -> Option<Vec<Self::EntityComponentRefTuple>> {
                    // 1. Get all entities which have the components mentioned in the tuple
                    // 2. Get the mutable component access for each one of them, and push it to the vec
                    // 3. If the component fetch fails, this means that either component is unavailable,
                    //      or it has been deleted.
                    //    3.a In this case we surrender all acquired component references and return None
                    // 4. If all acquisitions were successful, we have successfully acquire state access into
                    //      the world for all the required components. We can finally return

                    let entities: hashbrown::HashSet<&Entity> =
                        unsafe { (*world).get_entities_with_components::<Self>() };

                    let mut aggregated_vec: Vec<Self::EntityComponentRefTuple> = vec![];

                    // Acquiring Component references with their corressponding entities
                    for entity in entities {
                        let tuple = (
                            *entity,
                            $(
                                match unsafe { (*world).get_boxed_component_mut_ref::<$param>(*entity) } {
                                    Some(x) => QueryComponentRefMut {
                                        inner_boxed_component: x,
                                    },
                                    None => {
                                        return None;
                                    }
                                }
                            ),*
                        );
                    
                        aggregated_vec.push(tuple);
                    }
                
                    Some(aggregated_vec)
                }
            }
        }
}

ecs_macros::implement_tuples!(query_systems, 0, 20, F);





pub struct Query<'b, T: for<'a> SystemQuery<'a>> {
    entity_tuple_vec: Vec<<T as SystemQuery<'b>>::EntityComponentRefTuple>,
}

impl<'b, T: for<'a> SystemQuery<'a>> SystemParam for Query<'b, T> {
    fn initialise(world: *mut World) -> Option<Self> {
        let tuple = T::get_components_for_entities(world).unwrap();
        Some(Self {
            entity_tuple_vec: tuple
        })
    }
}

// impl<T: SystemQuery> Iterator for Query<T> {
//     type Item = T::IterType;

//     fn next(&mut self) -> Option<Self::Item> {
//         T::
//     }
// }
