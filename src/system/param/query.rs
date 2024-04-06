use std::{any::TypeId, cell::Ref, vec::IntoIter};

use crate::{entity::Entity, world::{unsafe_world::UnsafeWorldContainer, World}};

use super::SystemParam;

pub(crate) trait SystemQuery {

    type IterType;
    /// Gets the type_ids of specified component types in the tuple.
    /// This vector of type_ids could then be passed into the world 
    /// to get a list of components which have the specified components
    /// attached to them
    /// 
    /// We can then iterate through this list of entities to fetch the 
    /// appropriate component handles
    fn get_query(
        //world: &mut World
    ) -> Vec<TypeId>;

}


impl<T1, T2> SystemQuery for (T1, T2) where T1: SystemParam + 'static, T2: SystemParam + 'static {
    // type ComponentArr = Vec<($(&$param),*)>;
    type IterType = std::vec::IntoIter<(Entity, Ref<T1>, Ref<T2>)>;

    fn get_query(
        //world: &mut World
    ) -> Vec<TypeId> {
        vec![std::any::TypeId::of::<T1>(), std::any::TypeId::of::<T2>()]    
    }
}

macro_rules! query_systems {
    () => {
        ($($param: ident),*) => {

            #[allow(non_snake_case)]
            impl SystemQuery for ($($param),*) {
                fn get_query(world: &mut World) -> Vec<TypeId> {
                    vec![$(std::any::TypeId::of::<>($param)),*]    
                }

                fn get_component_arr(world: &mut World) -> Vec<$(&$param),*> {

                }
            }
        }
    };
}

pub struct Query<T: SystemQuery> { 
    entities: Vec<Entity>,
    _marker: std::marker::PhantomData<T>
}

impl<T: SystemQuery> SystemParam for Query<T> {
    fn initialise(world: *mut World) -> Option<Self> {
        let component_type_vec = T::get_query();
        // let entities: Vec<Entity> = world.get_entities_with_components(&component_type_vec);
        // Self {entities, _marker: std::marker::PhantomData}

        todo!()
    }
}

// impl<T: SystemQuery> Iterator for Query<T> {
//     type Item = T::IterType;

//     fn next(&mut self) -> Option<Self::Item> {
//         T::
//     }
// }