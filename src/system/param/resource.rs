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
/// A Resource Handle to access unique resources created in the world
///     
/// @NOTE: We get a mutable pointer to the world as a input to the initialise method
///         This is to facilitate RwLockGuard acquisition from the world for Resource
///         handles. &mut World makes rust believe the returned guard does not have
///         lifetime which lives long enough, so we fool the borrow checker this way
///         to make it think that the reference is static.

#[derive(SystemParam)]
pub struct ResourceHandle<R: Resource + 'static>
where
    R: Resource,
{
    inner_guard_box: OwnedRwLockReadGuard<Box<dyn Resource>>,
    _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for ResourceHandle<R> {
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>) {
        // @SAFETY:
        // 1. The world does not go out of scope (Otherwise we wouldn't be executing this function)
        // 2. The guard is only returned by the world only if no other
        //      mutable access guard to the resource is alive
        unsafe {
            match (*world).get_resource_ref::<R>() {
                (ResourceFetchResult::Success, Some(guard_box)) => {
                    (
                        None,
                        Some(
                            Self {
                                inner_guard_box: guard_box,
                                _marker: std::marker::PhantomData,
                            }
                        )
                    )
                },
                (ResourceFetchResult::Occupied, None) => (None, None),
                (ResourceFetchResult::DoesNotExist, None) => (Some(InitError{}), None),
                _ => panic!("Invalid result of initialisation")
            }
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
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>) {
        // @SAFETY:
        // 1. See safety description in [`ResourceHandle`]
        // 2. The lock is only returned by the world only when the Resource is
        //      not mutably or immutably borrowed by some process in the system.
        unsafe {
            match (*world).get_resource_mut::<R>() {
                (ResourceFetchResult::Success, Some(guard_box)) => {
                    (
                        None,
                        Some(
                            Self {
                                inner_guard_box: guard_box,
                                _marker: std::marker::PhantomData,
                            }
                        )
                    )
                },
                (ResourceFetchResult::Occupied, None) => (None, None),
                (ResourceFetchResult::DoesNotExist, None) => (Some(InitError{}), None),
                _ => panic!("Invalid result of initialisation")
            }
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
