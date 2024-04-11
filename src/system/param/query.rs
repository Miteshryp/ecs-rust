use std::{
    any::{Any, TypeId}, cell::{Ref, RefMut}, ops::{Deref, DerefMut}, sync::{RwLockReadGuard, RwLockWriteGuard}, vec::IntoIter
};

use crate::ECSBase;
use ecs_macros::ECSBase;

use crate::{
    component::{Component, handles::{ComponentHandle, MutComponentHandle}},
    entity::Entity,
    world::{unsafe_world::UnsafeWorldContainer, World},
};

use super::SystemParam;

pub(crate) trait SystemQuery {
    type EntityComponentHandleTuple;
    type EntityMutComponentHandleTuple;

    /// Gets the type_ids of specified component types in the tuple.
    /// This vector of type_ids could then be passed into the world
    /// to get a list of components which have the specified components
    /// attached to them
    ///
    /// We can then iterate through this list of entities to fetch the
    /// appropriate component handles
    fn get_query_entities() -> Vec<TypeId>;

    fn get_components_for_entities(
        world: *mut World
    ) -> Option<Vec<Self::EntityComponentHandleTuple>>;

    fn get_mut_components_for_entities(
        world: *mut World
    ) -> Option<Vec<Self::EntityMutComponentHandleTuple>>;
}

/// A internal structure to enclose the mutable component reference
/// fetched from the world.
/// This is primarily made to fool the borrow checker into thinking
/// that the RefMut has a static lifetime
// pub struct QueryComponentRefMut<C: Component + 'static> {
//     inner_boxed_component: Box<RefMut<'static, C>>,
// }
// impl<C: Component + 'static> Deref for QueryComponentRefMut<C> {
//     type Target = RefMut<'static, C>;

//     fn deref(&self) -> &Self::Target {
//         self.inner_boxed_component.as_ref()
//     }
// }



// @TODO: Change this implementation to store Arc based structs 
//          after making the corresponding changes in the component manager
macro_rules! query_systems {
    ($($param: ident),*) => {

        #[allow(non_snake_case)]
        impl<$($param: Component + 'static),*> SystemQuery for (Entity, $($param),*) {
            type EntityComponentHandleTuple = (Entity, $(ComponentHandle<$param>),*);
            type EntityMutComponentHandleTuple = (Entity, $(MutComponentHandle<$param>),*);

            fn get_query_entities(
            ) -> Vec<TypeId> {
                vec![$(std::any::TypeId::of::<$param>()),*]
            }

            fn get_components_for_entities(
                world: *mut World,
            ) -> Option<Vec<Self::EntityComponentHandleTuple>> {
                // Geting all entities which have the components mentioned in the tuple
                let entities: hashbrown::HashSet<&Entity> =
                unsafe { (*world).get_entities_with_components::<Self>() };
                
                // Get the mutable component access for each one of them, and push it to the vec
                let mut aggregated_vec: Vec<Self::EntityComponentHandleTuple> = vec![];
                for entity in entities {

                    // Acquiring Component references with their corressponding entities
                    let tuple = (
                        *entity,
                        $(
                            match unsafe { (*world).get_component_ref_lock::<$param>(*entity) } {
                                Some(x) => ComponentHandle::new(x),

                                // If the component fetch fails, this means that either
                                // component is unavailable, or it has been deleted. 
                                // In this case we surrender all acquired component references
                                // and return None
                                None => {
                                    return None;
                                }
                            }
                        ),*
                    );
                
                    aggregated_vec.push(tuple);
                }
                
                // If all acquisitions were successful, we have successfully 
                // acquire state access into the world for all the required
                // components. We can finally return
                Some(aggregated_vec)
            }

            fn get_mut_components_for_entities(
                world: *mut World,
            ) -> Option<Vec<Self::EntityMutComponentHandleTuple>> {
                // Geting all entities which have the components mentioned in the tuple
                let entities: hashbrown::HashSet<&Entity> =
                unsafe { (*world).get_entities_with_components::<Self>() };
                
                // Get the mutable component access for each one of them, and push it to the vec
                let mut aggregated_vec: Vec<Self::EntityMutComponentHandleTuple> = vec![];
                for entity in entities {

                    // Acquiring Component references with their corressponding entities
                    let tuple = (
                        *entity,
                        $(
                            match unsafe { (*world).get_component_ref_mut_lock::<$param>(*entity) } {
                                Some(x) => MutComponentHandle::new(x),

                                // If the component fetch fails, this means that either
                                // component is unavailable, or it has been deleted. 
                                // In this case we surrender all acquired component references
                                // and return None
                                None => {
                                    return None;
                                }
                            }
                        ),*
                    );
                
                    aggregated_vec.push(tuple);
                }
                
                // If all acquisitions were successful, we have successfully 
                // acquire state access into the world for all the required
                // components. We can finally return
                Some(aggregated_vec)
            }
        }
    }
}

ecs_macros::implement_tuples!(query_systems, 0, 20, F);





/// ### Description
/// 
/// Extracts the specified type of Components from the world to query.
/// 
/// The struct supplies the component's along with their [`Entity`](crate::entity::Entity)
/// **if and only if**
///     - All components specified are attached to the entity_id.
///     - All components are free for use and not being held by another system for use. 
///     (NOTE: This has a high chance of resulting in a deadlock through mutual starvation)
///
/// 
#[derive(ECSBase)]
pub struct Query<T: SystemQuery> {
    entity_tuple_vec: Vec<<T as SystemQuery>::EntityComponentHandleTuple>,
}

/// SAFETY: See SAFETY at [SystemParam]
unsafe impl<T: SystemQuery> Sync for Query<T>{}

impl<T: SystemQuery + 'static> SystemParam for Query<T> 
{
    // fn as_any(&self) -> &dyn Any {
    //     self as &dyn Any
    // }

    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self as &mut dyn Any
    // }

    fn initialise(world: *mut World) -> Option<Self> where Self: Sized {
        if let Some(extracted_tuples) = T::get_components_for_entities(world) {
            Some(Self{ 
                entity_tuple_vec: extracted_tuples
            })
        } else {
            None
        }
    }
    
    fn type_id() -> TypeId
    where
        Self: Sized + 'static,
    {
        TypeId::of::<Self>()
    }
}