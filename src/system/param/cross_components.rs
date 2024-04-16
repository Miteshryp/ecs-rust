use ecs_macros::SystemParam;
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::{
    component::{
        handles::{CrossComponentHandle, CrossMutComponentHandle},
        Component,
    },
    system::param::SystemParam,
    ECSBase,
};

use super::InitError;

/// @TODO: Document
#[derive(SystemParam)]
pub struct CrossComponentCollection<C: Component + 'static> {
    component_vec: Vec<OwnedRwLockReadGuard<C>>,
}

impl<C: Component + 'static> SystemParam for CrossComponentCollection<C> {
    fn initialise(world: *mut crate::world::World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        unsafe {
            let component_vec = match (*world).get_all_component_locks::<C>() {
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

impl<C: Component + 'static> CrossComponentCollection<C> {
    pub fn handler<Func: FnMut(CrossComponentHandle<C>, CrossComponentHandle<C>)>(
        &mut self,
        mut func: Func,
    ) {
        for i in 0..self.component_vec.len() {
            for j in i + 1..self.component_vec.len() {
                let a = CrossComponentHandle {
                    inner: self.component_vec.get(i).unwrap(),
                };

                let b = CrossComponentHandle {
                    inner: self.component_vec.get(j).unwrap(),
                };

                (func)(a, b);
            }
        }
    }
}




/// @TODO: Document
#[derive(SystemParam)]
pub struct CrossComponentCollectionMut<C: Component + 'static> {
    component_vec: Vec<OwnedRwLockWriteGuard<C>>,
}

impl<C: Component + 'static> SystemParam for CrossComponentCollectionMut<C> {
    fn initialise(world: *mut crate::world::World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        unsafe {
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
    pub fn handler<Func: FnMut(CrossMutComponentHandle<C>, CrossMutComponentHandle<C>)>(
        &mut self,
        mut func: Func,
    ) {
        for i in 0..self.component_vec.len() {
            for j in i + 1..self.component_vec.len() {
                let mut_slice = self.component_vec.as_mut_slice();
                let (first, last) = mut_slice.split_at_mut(i + 1);
                let first_len = first.len();

                let a = CrossMutComponentHandle {
                    inner: &mut first[first.len() - 1],
                };

                let b = CrossMutComponentHandle {
                    inner: &mut last[j - first_len],
                };

                (func)(a, b);
            }
        }
    }
}
