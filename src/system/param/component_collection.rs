use std::ops::{Deref, DerefMut};
use std::path::Iter;

use ecs_macros::SystemParam;
use tokio::sync::OwnedRwLockWriteGuard;

use crate::component::handles::{ComponentHandle, MutComponentHandle};
use crate::component::Component;
use crate::system::param::{InitError, SystemParam};
use crate::world::World;
use crate::ECSBase;

///
/// ### Description
///
/// A structure which can be a [`parameter`](crate::system::SystemParam)
/// in a [`system`](crate::System) function which enables the user to
/// immutably access and iterate over a specified type of [Component] in
/// [`world`][crate::World]
///
#[derive(SystemParam)]
pub struct ComponentCollection<C: Component + 'static> {
    locks: Vec<ComponentHandle<C>>,
}

impl<C: Component + 'static> ComponentCollection<C> {
    pub fn get_conditional_collection<Func: FnMut(&mut ComponentHandle<C>) -> bool>(
        &mut self,
        mut filter_func: Func
    ) -> Vec<&mut ComponentHandle<C>> {
        self.locks.iter_mut().filter_map(|handle| {
            if !(filter_func)(handle) {
                return None
            }
            Some(handle)
        }).collect()
    }
}



impl<C: Component + 'static> IntoIterator for ComponentCollection<C> {
    type Item = ComponentHandle<C>;

    type IntoIter = std::vec::IntoIter<ComponentHandle<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.locks.into_iter()
    }
}

impl<C: Component + 'static> SystemParam for ComponentCollection<C> {
    fn initialise(world: &World) -> (Option<super::InitError>, Option<Self>)
    where
        Self: Sized,
    {
        let self_obj = Self {
            locks: match world.get_all_component_locks::<C>() {
                Some(x) => {
                    let lock_vec: Vec<ComponentHandle<C>> = x
                        .into_iter()
                        .map(|(entity, lock)| ComponentHandle::new(lock, entity))
                        .collect();

                    if lock_vec.len() == 0 {
                        return (Some(InitError {}), None);
                    }

                    lock_vec
                }
                None => return (Some(InitError {}), None),
            },
        };

        return (None, Some(self_obj));
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(C::type_id());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}

///
/// ### Description
///
/// A structure which can be a [`parameter`](crate::system::SystemParam)
/// in a [`system`](crate::System) function which enables the user to
/// mmutably access and iterate over a specified type of [Component] in
/// [`world`][crate::World]
///
#[derive(SystemParam)]
pub struct ComponentCollectionMut<C: Component + 'static> {
    locks: Vec<MutComponentHandle<C>>,
}

impl<C: Component + 'static> ComponentCollectionMut<C> {
    pub fn get_conditional_collection<Func: FnMut(&mut MutComponentHandle<C>) -> bool>(
        &mut self,
        mut filter_func: Func
    ) -> Vec<&mut MutComponentHandle<C>> {
        self.locks.iter_mut().filter_map(|handle| {
            if !(filter_func)(handle) {
                return None
            }
            Some(handle)
        }).collect()
    }
}

impl<C: Component + 'static> IntoIterator for ComponentCollectionMut<C> {
    type Item = MutComponentHandle<C>;

    type IntoIter = std::vec::IntoIter<MutComponentHandle<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.locks.into_iter()
    }
}

impl<C: Component + 'static> SystemParam for ComponentCollectionMut<C> {
    fn initialise(world: &World) -> (Option<super::InitError>, Option<Self>)
    where
        Self: Sized,
    {
        let self_obj = Self {
            locks: match world.get_all_component_locks_mut::<C>() {
                Some(x) => {
                    let lock_vec: Vec<MutComponentHandle<C>> = x
                        .into_iter()
                        .map(|(entity, lock)| MutComponentHandle::new(lock, entity))
                        .collect();

                    if lock_vec.len() == 0 {
                        return (Some(InitError {}), None);
                    }

                    lock_vec
                }
                None => return (Some(InitError {}), None),
            },
        };

        return (None, Some(self_obj));
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(C::type_id());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        true
    }
}
