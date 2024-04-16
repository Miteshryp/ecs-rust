pub mod base_query;

use base_query::SystemQuery;

use std::{
    any::TypeId,
    vec::IntoIter,
};

use crate::ECSBase;
use ecs_macros::SystemParam;

use crate::world::World;

use super::{InitError, SystemParam};




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
#[derive(SystemParam)]
pub struct Query<T: SystemQuery> {
    entity_tuple_vec: Vec<<T as SystemQuery>::EntityComponentHandleTuple>,
}

impl<T: SystemQuery> IntoIterator for Query<T> {
    type Item = <T as SystemQuery>::EntityComponentHandleTuple;

    type IntoIter = IntoIter<<T as SystemQuery>::EntityComponentHandleTuple>;

    fn into_iter(self) -> Self::IntoIter {
        self.entity_tuple_vec.into_iter()
    }
}



/// SAFETY: See SAFETY at [SystemParam]
// unsafe impl<T: SystemQuery> Sync for Query<T>{}

impl<T: SystemQuery + 'static> SystemParam for Query<T> {
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        // SystemQueries
        if let Some(extracted_tuples) = T::get_components_for_entities(world) {
            if extracted_tuples.len() == 0 {
                // There's no point in running the system if its going to run with
                // no elements in the query.
                // @NOTE: This could however present a situation where
                //      the user does not know this behavior for query
                // @TODO: Document this behavior in both Query structs
                return (Some(InitError {}), None);
            }
            (
                None,
                Some(Self {
                    entity_tuple_vec: extracted_tuples,
                }),
            )
        } else {
            (None, None)
        }
    }

    fn get_resource_access_type() -> hashbrown::HashSet<TypeId> {
        T::get_component_typeid_set()
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}





// @TODO: Document
#[derive(SystemParam)]
pub struct QueryMut<T: SystemQuery> {
    entity_tuple_vec: Vec<<T as SystemQuery>::EntityMutComponentHandleTuple>,
}


impl<T: SystemQuery> IntoIterator for QueryMut<T> {
    type Item = <T as SystemQuery>::EntityMutComponentHandleTuple;

    type IntoIter = IntoIter<<T as SystemQuery>::EntityMutComponentHandleTuple>;

    fn into_iter(self) -> Self::IntoIter {
        self.entity_tuple_vec.into_iter()
    }
}



impl<T: SystemQuery + 'static> SystemParam for QueryMut<T> {
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        if let Some(extracted_tuples) = T::get_mut_components_for_entities(world) {
            if extracted_tuples.len() == 0 {
                // There's no point in running the system if its going to run with
                // no elements in the query.
                // @NOTE: This could however present a situation where
                //      the user does not know this behavior for query
                // @TODO: Document this behavior in both Query structs
                return (Some(InitError {}), None);
            }
            (
                None,
                Some(Self {
                    entity_tuple_vec: extracted_tuples,
                }),
            )
        } else {
            (None, None)
        }
    }

    fn get_resource_access_type() -> hashbrown::HashSet<TypeId> {
        T::get_component_typeid_set()
    }

    fn is_resource_access_mut() -> bool {
        true
    }
}
// @TODO: Implement iterators for all query types

