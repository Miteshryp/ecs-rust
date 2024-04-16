use std::ops::{Deref, DerefMut};
use std::path::Iter;

use ecs_macros::SystemParam;
use tokio::sync::OwnedRwLockWriteGuard;

use crate::ECSBase;
use crate::component::handles::{ComponentHandle, MutComponentHandle};
use crate::system::param::{InitError, SystemParam};
use crate::component::Component;


/// @TODO: Documents
#[derive(SystemParam)]
pub struct ComponentCollection<C: Component + 'static> {
    locks: Vec<ComponentHandle<C>>
}

impl<C: Component + 'static> IntoIterator for ComponentCollection<C> {
    type Item = ComponentHandle<C>;

    type IntoIter = std::vec::IntoIter<ComponentHandle<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.locks.into_iter()
    }
}


impl<C: Component + 'static> SystemParam for ComponentCollection<C> {
    fn initialise(world: *mut crate::world::World) -> (Option<super::InitError>, Option<Self>) where Self: Sized {
        unsafe {
            let self_obj = Self {
                locks: match (*world).get_all_component_locks::<C>() {
                    Some(x) => {
                        x.into_iter().map(|lock| ComponentHandle::new(lock) ).collect()
                    },
                    None => return (Some(InitError{}), None),
                },
            };

            return (None, Some(self_obj))
        }
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





/// @TODO: Document
#[derive(SystemParam)]
pub struct ComponentCollectionMut<C: Component + 'static> {
    locks: Vec<MutComponentHandle<C>>,
}


impl<C: Component + 'static> IntoIterator for ComponentCollectionMut<C> {
    type Item = MutComponentHandle<C>;

    type IntoIter = std::vec::IntoIter<MutComponentHandle<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.locks.into_iter()
    }
}


impl<C: Component + 'static> SystemParam for ComponentCollectionMut<C> {
    fn initialise(world: *mut crate::world::World) -> (Option<super::InitError>, Option<Self>) where Self: Sized {
        unsafe {
            let self_obj = Self {
                locks: match (*world).get_all_component_locks_mut::<C>() {
                    Some(x) => x.into_iter().map(|lock| MutComponentHandle::new(lock)).collect(),
                    None => return (Some(InitError{}), None),
                },
            };

            return (None, Some(self_obj))
        }
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
