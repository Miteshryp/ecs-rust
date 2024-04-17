use std::any::TypeId;


use crate::{
    component::{
        handles::{ComponentHandle, MutComponentHandle},
        Component,
    },
    entity::Entity,
    world::World,
};




pub trait SystemQuery {
    type EntityComponentHandleTuple;
    type EntityMutComponentHandleTuple;

    /// Gets the type_ids of specified component types in the tuple.
    /// This vector of type_ids could then be passed into the world
    /// to get a list of components which have the specified components
    /// attached to them
    ///
    /// We can then iterate through this list of entities to fetch the
    /// appropriate component handles
    fn get_query_component_ids() -> Vec<TypeId>;

    fn get_components_for_entities(
        world: &World,
    ) -> Option<Vec<Self::EntityComponentHandleTuple>>;

    fn get_mut_components_for_entities(
        world: &World,
    ) -> Option<Vec<Self::EntityMutComponentHandleTuple>>;

    fn get_component_typeid_set() -> hashbrown::HashSet<TypeId>;
}

macro_rules! query_systems {
    ($($param: ident),*) => {

        #[allow(non_snake_case)]
        impl<$($param: Component + 'static),*> SystemQuery for (Entity, $($param),*) {
            type EntityComponentHandleTuple = (Entity, $(ComponentHandle<$param>),*);
            type EntityMutComponentHandleTuple = (Entity, $(MutComponentHandle<$param>),*);


            fn get_component_typeid_set() -> hashbrown::HashSet<TypeId> {
                let mut hash_set = hashbrown::HashSet::new();
                $(hash_set.insert(std::any::TypeId::of::<$param>());)*
                hash_set
            }


            fn get_query_component_ids(
            ) -> Vec<TypeId> {
                vec![$(std::any::TypeId::of::<$param>()),*]
            }

            fn get_components_for_entities(
                world: &World,
            ) -> Option<Vec<Self::EntityComponentHandleTuple>> {
                // Geting all entities which have the components mentioned in the tuple
                let entities: hashbrown::HashSet<&Entity> =
                world.get_entities_with_components::<Self>();

                // Get the mutable component access for each one of them, and push it to the vec
                let mut aggregated_vec: Vec<Self::EntityComponentHandleTuple> = vec![];
                for entity in entities {

                    // Acquiring Component references with their corressponding entities
                    let tuple = (
                        *entity,
                        $(
                            match world.get_component_ref_lock::<$param>(*entity) {
                                Some(x) => ComponentHandle::new(x, *entity),

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
                world: &World,
            ) -> Option<Vec<Self::EntityMutComponentHandleTuple>> {
                // Geting all entities which have the components mentioned in the tuple
                let entities: hashbrown::HashSet<&Entity> =
                world.get_entities_with_components::<Self>();

                // Get the mutable component access for each one of them, and push it to the vec
                let mut aggregated_vec: Vec<Self::EntityMutComponentHandleTuple> = vec![];
                for entity in entities {

                    // Acquiring Component references with their corressponding entities
                    let tuple = (
                        *entity,
                        $(
                            match world.get_component_ref_mut_lock::<$param>(*entity) {
                                Some(x) => MutComponentHandle::new(x, *entity),

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
