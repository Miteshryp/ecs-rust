use ecs_macros::SystemParam;
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::{
    component::{
        handles::{ComponentRefHandle, MutComponentRefHandle},
        Component,
    },
    entity::Entity,
    system::param::SystemParam,
    world::World,
    ecs_base::ECSBase
};

use super::InitError;

///
/// ### Description
///
/// This is a [`parameter`](crate::system::SystemParam) type for a
/// [`system`](crate::System) function defined by the user which lets
/// them immutably access all components of a specific type as a combination.
///
/// This structure produces a cross product set of components of type
/// [`C`] stored in the world
///
/// This parameter is useful for creating comparison based systems
/// for a component, which could be used for interactions, updations, etc
///
///
#[derive(SystemParam)]
pub struct CrossComponentCollection<C: Component + 'static> {
    /// Storing the acquired locks in the vector to later
    /// create appropriate handles by using reference to
    /// the read guards stored here
    component_vec: Vec<(Entity, OwnedRwLockReadGuard<C>)>,
}

impl<C: Component + 'static> SystemParam for CrossComponentCollection<C> {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        let component_vec = match world.get_all_component_locks::<C>() {
            Some(x) => {
                if x.len() < 2 {
                    // This type of system should not execute
                    // with a single component
                    return (Some(InitError {}), None);
                }
                x
            }
            None => return (Some(InitError {}), None),
        };

        (None, Some(Self { component_vec }))
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(std::any::TypeId::of::<C>());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}

// @TODO: Write conditional executor method
impl<C: Component + 'static> CrossComponentCollection<C> {
    ///
    /// ### Description
    ///
    /// Specifies the operation to be performed on each combination
    /// by taking a closure
    ///
    /// The closure parameter contains 2 [CrossComponentHandle] types
    /// which provide immutable access to a component instance. The
    /// 2 components are guaranteed to be from different entities
    ///
    pub fn execute_handler<Func>(&self, execution_func: Func)
    where
        Func: FnMut(ComponentRefHandle<C>, ComponentRefHandle<C>),
    {
        Self::execute_handler_on_component_arr(&self.component_vec, execution_func, |_| true);
    }

    // @TODO: Document
    pub fn execute_filtered_handler<Func, FilterFunc>(
        &self,
        execution_func: Func,
        filter_func: FilterFunc,
    ) where
        Func: FnMut(ComponentRefHandle<C>, ComponentRefHandle<C>),
        FilterFunc: FnMut(ComponentRefHandle<C>) -> bool,
    {
        Self::execute_handler_on_component_arr(&self.component_vec, execution_func, filter_func);
    }
}


/// Private methods for [CrossComponentCollection]
impl<C: Component + 'static> CrossComponentCollection<C> {
    fn execute_handler_on_component_arr<Func, FilterFunc>(
        components: &Vec<(Entity, OwnedRwLockReadGuard<C>)>,
        mut execution_func: Func,
        mut filter_func: FilterFunc,
    ) where
        Func: FnMut(ComponentRefHandle<C>, ComponentRefHandle<C>),
        FilterFunc: FnMut(ComponentRefHandle<C>) -> bool,
    {
        let mut component_vec: Vec<&(Entity, OwnedRwLockReadGuard<C>)> = components
            .iter()
            .filter_map(|tuple| {
                let handle = ComponentRefHandle {
                    entity_id: tuple.0,
                    inner: &tuple.1,
                };

                if !filter_func(handle) {
                    return None;
                }
                Some(tuple)
            })
            .collect();

        for i in 0..component_vec.len() {
            for j in (i + 1)..component_vec.len() {
                let mut_slice = component_vec.as_mut_slice();
                let (first, last) = mut_slice.split_at_mut(i + 1);
                let first_len = first.len();

                let a = ComponentRefHandle {
                    entity_id: first[first.len() - 1].0,
                    inner: &first[first.len() - 1].1,
                };

                let b = ComponentRefHandle {
                    entity_id: last[j - first_len].0,
                    inner: &last[j - first_len].1,
                };

                (execution_func)(a, b);
            }
        }
    }
}

///
/// ### Description
///
/// This is a [`parameter`](crate::system::SystemParam) type for a
/// [`system`](crate::System) function defined by the user which lets
/// them access and modify all components of a specific type as a combination
///
/// This structure produces a cross product set of components of type
/// [`C`] stored in the world
///
/// This parameter is useful for creating comparison based systems
/// for a component, which could be used for interactions, updations, etc
///
///
#[derive(SystemParam)]
pub struct CrossComponentCollectionMut<C: Component + 'static> {
    component_vec: Vec<(Entity, OwnedRwLockWriteGuard<C>)>,
}

impl<C: Component + 'static> SystemParam for CrossComponentCollectionMut<C> {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        let component_vec = match (*world).get_all_component_locks_mut::<C>() {
            Some(x) => {
                if x.len() < 2 {
                    // This type of system should not execute
                    // with a single component
                    return (Some(InitError {}), None);
                }
                x
            }
            None => return (Some(InitError {}), None),
        };

        (None, Some(Self { component_vec }))
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(std::any::TypeId::of::<C>());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        true
    }
}

impl<C: Component + 'static> CrossComponentCollectionMut<C> {
    ///
    /// ### Description
    ///
    /// Specifies the operation to be performed on each combination
    /// by taking a closure
    ///
    /// The closure parameter contains 2 [CrossMutComponentHandle] types
    /// which provide immutable access to a component instance. The
    /// 2 components are guaranteed to be from different entities
    ///
    pub fn execute_handler<Func: FnMut(MutComponentRefHandle<C>, MutComponentRefHandle<C>)>(
        &mut self,
        mut func: Func,
    ) {
        Self::execute_handler_on_component_arr(&mut self.component_vec, func, |_| true);
        // for i in 0..self.component_vec.len() {
        //     for j in i + 1..self.component_vec.len() {
        //         let mut_slice = self.component_vec.as_mut_slice();
        //         let (first, last) = mut_slice.split_at_mut(i + 1);
        //         let first_len = first.len();

        //         let a = MutComponentRefHandle {
        //             entity_id: first[first.len() - 1].0,
        //             inner: &mut first[first.len() - 1].1,
        //         };

        //         let b = MutComponentRefHandle {
        //             entity_id: last[j - first_len].0,
        //             inner: &mut last[j - first_len].1,
        //         };

        //         (func)(a, b);
        //     }
        // }
    }

    // @TODO: Document
    pub fn execute_filtered_handler<Func, FilterFunc>(
        &mut self,
        mut execution_func: Func,
        mut filter_func: FilterFunc,
    ) where
        Func: FnMut(MutComponentRefHandle<C>, MutComponentRefHandle<C>),
        FilterFunc: FnMut(MutComponentRefHandle<C>) -> bool,
    {
        Self::execute_handler_on_component_arr(
            &mut self.component_vec,
            execution_func,
            filter_func,
        );
    }
}



/// Private methods for [CrossComponentCollectionMut]
impl<C: Component + 'static> CrossComponentCollectionMut<C> {
    fn execute_handler_on_component_arr<Func, FilterFunc>(
        components: &mut Vec<(Entity, OwnedRwLockWriteGuard<C>)>,
        mut execution_func: Func,
        mut filter_func: FilterFunc,
    ) where
        Func: FnMut(MutComponentRefHandle<C>, MutComponentRefHandle<C>),
        FilterFunc: FnMut(MutComponentRefHandle<C>) -> bool,
    {
        let mut component_vec: Vec<&mut (Entity, OwnedRwLockWriteGuard<C>)> = components
            .iter_mut()
            .filter_map(|tuple| {
                let handle = MutComponentRefHandle {
                    entity_id: tuple.0,
                    inner: &mut tuple.1,
                };

                if !filter_func(handle) {
                    return None;
                }
                Some(tuple)
            })
            .collect();

        for i in 0..component_vec.len() {
            for j in (i + 1)..component_vec.len() {
                let mut_slice = component_vec.as_mut_slice();
                let (first, last) = mut_slice.split_at_mut(i + 1);
                let first_len = first.len();

                let a = MutComponentRefHandle {
                    entity_id: first[first.len() - 1].0,
                    inner: &mut first[first.len() - 1].1,
                };

                let b = MutComponentRefHandle {
                    entity_id: last[j - first_len].0,
                    inner: &mut last[j - first_len].1,
                };

                (execution_func)(a, b);
            }
        }
    }
}
