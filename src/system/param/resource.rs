use ecs_macros::{ECSBase, Resource, SystemParam};
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use super::{InitError, SystemParam};
use crate::ECSBase;
use crate::{resource::Resource, world::World};
use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

pub enum ResourceFetchResult {
    Success,
    Occupied,
    DoesNotExist,
}

///
/// ### Description
///
/// A Resource Handle to access unique resources created in the world.
/// This structure provides a resource access to users in the system function
/// by storing an owned guard to a particular resource in the system.
///     

#[derive(SystemParam)]
pub struct ResourceHandle<R: Resource + 'static>
where
    R: Resource,
{
    inner_guard_box: OwnedRwLockReadGuard<Box<dyn Resource>>,
    _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for ResourceHandle<R> {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>) {
        match (*world).get_resource_ref_lock::<R>() {
            (ResourceFetchResult::Success, Some(guard_box)) => (
                None,
                Some(Self {
                    inner_guard_box: guard_box,
                    _marker: std::marker::PhantomData,
                }),
            ),
            (ResourceFetchResult::Occupied, None) => (None, None),
            (ResourceFetchResult::DoesNotExist, None) => (Some(InitError {}), None),
            _ => panic!("Invalid result of initialisation"),
        }
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(std::any::TypeId::of::<R>());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}

impl<R: Resource + 'static> Deref for ResourceHandle<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner_guard_box.as_any().downcast_ref::<R>().unwrap()
    }
}

#[derive(SystemParam)]
pub struct MutResourceHandle<R: Resource + 'static>
where
    R: Resource,
{
    inner_guard_box: OwnedRwLockWriteGuard<Box<dyn Resource>>,
    _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for MutResourceHandle<R> {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>) {
        match (*world).get_resource_mut_lock::<R>() {
            (ResourceFetchResult::Success, Some(guard_box)) => (
                None,
                Some(Self {
                    inner_guard_box: guard_box,
                    _marker: std::marker::PhantomData,
                }),
            ),
            (ResourceFetchResult::Occupied, None) => (None, None),
            (ResourceFetchResult::DoesNotExist, None) => (Some(InitError {}), None),
            _ => panic!("Invalid result of initialisation"),
        }
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        let mut hash_set = hashbrown::HashSet::new();
        hash_set.insert(std::any::TypeId::of::<R>());
        hash_set
    }

    fn is_resource_access_mut() -> bool {
        true
    }
}

impl<R: Resource + 'static> Deref for MutResourceHandle<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner_guard_box.as_any().downcast_ref::<R>().unwrap()
    }
}

impl<R: Resource + 'static> DerefMut for MutResourceHandle<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard_box
            .as_any_mut()
            .downcast_mut::<R>()
            .unwrap()
    }
}
